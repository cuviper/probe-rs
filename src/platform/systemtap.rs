//! SystemTap static probes
//!
//! This is a mechanism for developers to provide static annotations for
//! meaningful points in code, with arguments that indicate some relevant
//! state. Such locations may be probed by SystemTap `process.mark("name")`,
//! and GDB can also locate them with `info probes` and `break -probe name`.
//!
//! The impact on code generation is designed to be minimal: just a single
//! `NOP` placeholder is added inline for instrumentation, and ELF notes
//! contain metadata to name the probe and describe the location of its
//! arguments.
//!
//! # Links:
//!
//! * <https://sourceware.org/systemtap/man/stapprobes.3stap.html#lbAP> (see `process.mark`)
//! * <https://sourceware.org/systemtap/wiki/AddingUserSpaceProbingToApps>
//! * <https://sourceware.org/systemtap/wiki/UserSpaceProbeImplementation>
//! * <https://sourceware.org/gdb/onlinedocs/gdb/Static-Probe-Points.html>

use core::cell::UnsafeCell;
use core::ptr;

//
// DEVELOPER NOTES
//
// Arguments are currently type-casted as isize for the supposed maximum
// register size, whereas SystemTap's long is i64 no matter the architecture.
// However, if we could figure out types here, they could be annotated more
// specifically, for example an argstr of "4@$0 -2@$1" indicates u32 and i16
// respectively. Any pointer would be fine too, like *c_char, simply 4@ or 8@
// for target_word_size.
//
// The macros in sdt.h don't know types either, so they split each argument
// into two asm inputs, roughly:
//   asm("[...]"
//       ".asciz \"%n[_SDT_S0]@%[_SDT_A0]\""
//       "[...]"
//     : :
//     [_SDT_S0] "n" ((_SDT_ARGSIGNED (x) ? 1 : -1) * (int) sizeof (x)),
//     [_SDT_A0] "nor" (x)
//     );
// where _SDT_ARGSIGNED is a macro using gcc builtins, so it's still resolved a
// compile time, and %n makes it a raw literal rather than an asm number.
//
// This might be a possible direction for Rust SDT to follow. For LLVM
// InlineAsm, the string would look like "${0:n}@$1", but we need the size/sign
// for that first input, and that must be a numeric constant no matter what
// optimization level we're at. With Rust RFC 2850 `asm!`, it might be possible
// to use positional `{}@{}` with a `const` operand for the size, but calling
// things like `mem::size_of::<T>()` is still hard when we don't know `T`.
//
// FIXME semaphores - SDT can define a short* that debuggers will increment when
// they attach, and decrement on detach. Thus a `probe_enabled!(provider,name)`
// could return if that value != 0, to be used similarly to log_enabled!(). It
// is difficult with mangling and macro hygene to connect two `probe!` and
// `probe_enabled!` calls to the same symbol, unless we forced `#[no_mangle]`.
// For now, we only use semaphores in `probe_lazy!` to skip argument evaluation
// when there's nobody attached to see the probe.
//

#[repr(transparent)]
pub struct Semaphore(UnsafeCell<u16>);

// SAFETY: the UnsafeCell is only ever read as far as Rust is
// concerned; data races require a read and a write.
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// Return a `Semaphore` that starts as disabled.
    pub const fn new() -> Self {
        Self(UnsafeCell::new(0))
    }

    /// Return whether a debugger or tracing tool is attached to a probe
    /// that uses this semaphore.
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (unsafe { ptr::read_volatile(self.0.get() as *const _) }) != 0u16
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe(
    ($provider:ident, $name:ident, $($arg:expr,)*) => ({
        $crate::sdt!([sym 0], $provider, $name, $($arg,)*);
    })
);

#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe_lazy(
    ($provider:ident, $name:ident, $($arg:expr,)*) => ({
        #[link_section = ".probes"]
        static SEMAPHORE: $crate::Semaphore = $crate::Semaphore::new();
        let enabled = SEMAPHORE.enabled();
        if enabled {
            $crate::sdt!([sym "{}" SEMAPHORE], $provider, $name, $($arg,)*);
        }
        enabled
    })
);

// Since we can't #include <sys/sdt.h>, we have to reinvent it...
// but once you take out the C/C++ type handling, there's not a lot to it.
#[doc(hidden)]
#[macro_export]
macro_rules! sdt(
    ([sym $symstr:literal $($sym:ident)?],
        $provider:ident, $name:ident, $($arg:expr,)*
    ) => (
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        $crate::sdt!([sym $symstr $($sym)?, opt att_syntax],
            $provider, $name, $($arg,)*);

        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        $crate::sdt!([sym $symstr $($sym)?, opt],
            $provider, $name, $($arg,)*);
    );

    ([sym $symstr:literal $($sym:ident)?, opt $($opt:ident)?],
        $provider:ident, $name:ident, $($arg1:expr, $($arg:expr,)*)?
    ) => (
        #[cfg(target_pointer_width = "32")]
        $crate::sdt!([sym $symstr $($sym)?, opt $($opt)?, size 4],
            $provider, $name, $("-4@{}", $arg1, $(" -4@{}", $arg,)*)?);

        #[cfg(target_pointer_width = "64")]
        $crate::sdt!([sym $symstr $($sym)?, opt $($opt)?, size 8],
            $provider, $name, $("-8@{}", $arg1, $(" -8@{}", $arg,)*)?);
    );

    ([sym $symstr:literal $($sym:ident)?, opt $($opt:ident)?, size $size:literal],
        $provider:ident, $name:ident, $($argstr:literal, $arg:expr,)*
    ) => (unsafe {
        ::core::arch::asm!(concat!(r#"
990:    nop
        .pushsection .note.stapsdt,"?","note"
        .balign 4
        .4byte 992f-991f, 994f-993f, 3
991:    .asciz "stapsdt"
992:    .balign 4
993:    ."#, $size, r#"byte 990b
        ."#, $size, r#"byte _.stapsdt.base
        ."#, $size, r#"byte "#, $symstr, r#"
        .asciz ""#, stringify!($provider), r#""
        .asciz ""#, stringify!($name), r#""
        .asciz ""#, $($argstr,)* r#""
994:    .balign 4
        .popsection
.ifndef _.stapsdt.base
        .pushsection .stapsdt.base,"aG","progbits",.stapsdt.base,comdat
        .weak _.stapsdt.base
        .hidden _.stapsdt.base
_.stapsdt.base: .space 1
        .size _.stapsdt.base, 1
        .popsection
.endif"#),
            $(sym $sym,)?
            $(in(reg) ($arg) as isize,)*
            options(readonly, nostack, preserves_flags $(, $opt)?),
        )
    });
);
