#![feature(asm_sym)]

use probe::probe;

fn main() {
    let mut iter = 0;
    loop {
        iter += 1;
        probe!(foo, iter, {
            std::thread::sleep(std::time::Duration::from_secs(1));
            iter
        });
    }
}

// bcc/tools/trace.py -p $(pidof loop) 'u::foo:iter "iter = %d", arg1'
