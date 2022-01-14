// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SystemTap static probes
//!
//! This is a mechanism for developers to provide static annotations for
//! meaningful points in code, with arguments that indicate some relevant
//! state.  Such locations may be probed by SystemTap `process.mark("name")`,
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

//
// DEVELOPER NOTES
//
// Arguments are currently type-casted as isize for the supposed maximum
// register size, whereas SystemTap's long is i64 no matter the architecture.
// However, if we could figure out types here, they could be annotated more
// specifically, for example an argstr of "4@$0 -2@$1" indicates u32 and i16
// respectively.  Any pointer would be fine too, like *c_char, simply 4@ or 8@
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
// This might be a possible direction for Rust SDT to follow.  For LLVM
// InlineAsm, the string would look like "${0:n}@$1", but we need the size/sign
// for that first input, and that must be a numeric constant no matter what
// optimization level we're at. With Rust RFC 2850 `asm!`, it might be possible
// to use positional `{}@{}` with a `const` operand for the size, but calling
// things like `mem::size_of::<T>()` is still hard when we don't know `T`.
//
// FIXME semaphores - SDT can define a short* that debuggers will increment when
// they attach, and decrement on detach.  Thus a probe_enabled!(provider,name)
// could return if that value != 0, to be used similarly to log_enabled!().  We
// could even be clever and skip argument evaluation altogether, the same way
// that log!() checks log_enabled!() first.
//

#[cfg(target_pointer_width = "32")]
#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe(
    ($provider:ident, $name:ident,)
    => ($crate::sdt_asm!(4, $provider, $name,));

    ($provider:ident, $name:ident, $arg1:expr, $($arg:expr,)*)
    => ($crate::sdt_asm!(4, $provider, $name,
            "-4@{}", $arg1, $(" -4@{}", $arg,)*));
);

#[cfg(target_pointer_width = "64")]
#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe(
    ($provider:ident, $name:ident,)
    => ($crate::sdt_asm!(8, $provider, $name,));

    ($provider:ident, $name:ident, $arg1:expr, $($arg:expr,)*)
    => ($crate::sdt_asm!(8, $provider, $name,
            "-8@{}", $arg1, $(" -8@{}", $arg,)*));
);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[doc(hidden)]
#[macro_export]
macro_rules! sdt_asm(
    ($size:literal, $provider:ident, $name:ident, $($argstr:literal, $arg:expr,)*)
    => (unsafe {
        $crate::_sdt_asm!($size, options(att_syntax), $provider, $name, $($argstr, $arg,)*);
    }));

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[doc(hidden)]
#[macro_export]
macro_rules! sdt_asm(
    ($size:literal, $provider:ident, $name:ident, $($argstr:literal, $arg:expr,)*)
    => (unsafe {
        $crate::_sdt_asm!($size, options(), $provider, $name, $($argstr, $arg,)*);
    }));

// Since we can't #include <sys/sdt.h>, we have to reinvent it...
// but once you take out the C/C++ type handling, there's not a lot to it.
#[doc(hidden)]
#[macro_export]
macro_rules! _sdt_asm(
    ($size:literal, options ($($opt:ident),*), $provider:ident, $name:ident, $($argstr:literal, $arg:expr,)*) => (
        ::core::arch::asm!(concat!(r#"
990:    nop
        .pushsection .note.stapsdt,"?","note"
        .balign 4
        .4byte 992f-991f, 994f-993f, 3
991:    .asciz "stapsdt"
992:    .balign 4
993:    ."#, $size, r#"byte 990b
        ."#, $size, r#"byte _.stapsdt.base
        ."#, $size, r#"byte 0 // FIXME set semaphore address
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
.endif
"#
            ),
            $(in(reg) (($arg) as isize) ,)*
            options(readonly, nostack, preserves_flags, $($opt),*),
        )
    ));
