use core::mem::size_of;
use std::ops::Deref;
const MYSZ: usize = size_of::<usize>();

#[repr(align(8))]
pub struct MYIVec([u8; MYSZ]);

impl MYIVec {
    fn new(_: &[u8]) -> Self {
        Self([0; MYSZ])
    }
}

impl Deref for MYIVec {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.0[..]
    }
}

fn main() {
    // use tokio;
    let arr = &[1_u8; 5];
    let arr2 = &arr;
    let hi = &arr2[1..2];
    let lo = &arr[1..3];
    let ivec = MYIVec::new(arr);
    let comp1 = hi.partial_cmp(lo);
    let comp2 = hi.partial_cmp(&ivec);
    println!("{:?} {:?}", comp1, comp2);
}

