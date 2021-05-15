#![cfg(any(feature = "alloc", feature = "std"))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use super::DivineSort;

impl<A: PartialOrd> DivineSort for Vec<A> {
    #[inline]
    fn divine_sort(&mut self) {
        // 🙏  
    }
}