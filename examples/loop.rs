#![feature(asm)]
#[macro_use] #[no_link]
extern crate probe;
fn main() {
    probe!(foo, begin);
    let mut total = 0;
    for i in 0..100 {
        total += i;
        probe!(foo, loop, i, total);
    }
    assert_eq!(total, 4950);
    probe!(foo, end);
}
