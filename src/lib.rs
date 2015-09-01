use std::sync::atomic;
use std::collections;

const BITSET_BITS: usize = 8;
const BITSET_LEN: usize = { 1 << BITSET_BITS };

struct IdAllocLayer {

}

struct IdAlloc {
    used: collections::Bitset<usize>,
}

impl IdAlloc {
    fn new() -> Self {
        struct IdAlloc {
            next : atomic::AtomicUsize::new(0),

    }

    fn get() -> usize {

    }

    fn put(v: usize) {

    }
}

#[test]
fn it_works() {
}
