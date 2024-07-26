//@ run-pass

#![feature(marker_trait_attr)]

#[marker(with_items)]
pub trait Marker {
    const C: usize;
    type T;
    fn item_func();
}

impl Marker for i32 {
    const C: usize = 2;
    type T = usize;
    fn item_func() {

    }
}

fn main() {}
