#![cfg(any(feature = "alloc", feature = "std"))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use super::StalinSort;

impl<A: PartialOrd> StalinSort for Vec<A> {
    #[inline]
    fn stalin_sort(&mut self) {
        let mut cur = 0;
        while let Some(item) = self.get(cur + 1) {
            // If we have `cur + 1`, then `cur` is a valid index. Evidently, the
            // optimizer isn't able to come to this conclusion, which is why
            // we're using an unchecked index.
            let cur_item = unsafe { self.get_unchecked(cur) };
            if item < cur_item {
                self.remove(cur + 1);
            } else {
                cur += 1;
            }
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Vec::<i32>::new(), []);
    }

    #[test]
    fn single() {
        assert_eq!(vec![1], [1]);
    }

    #[test]
    fn two() {
        assert_eq!(vec![1, 2], [1, 2]);
    }

    #[test]
    fn pre_sorted() {
        let sorted = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            Vec::from(&sorted[..]).stalin_sorted(),
            sorted,
        );
    }

    #[test]
    fn trailing_unsorted() {
        assert_eq!(
            vec![42, 1, 2, 3, 4, 5, 6].stalin_sorted(),
            [42],
        );
    }

    #[test]
    fn trailing_unsorted_same() {
        assert_eq!(
            vec![42, 1, 2, 3, 4, 5, 6, 42].stalin_sorted(),
            [42, 42],
        );
    }

    #[test]
    fn trailing_unsorted_same_continued() {
        assert_eq!(
            vec![42, 1, 2, 3, 4, 5, 6, 42, 1].stalin_sorted(),
            [42, 42],
        );
    }

    #[test]
    fn trailing_unsorted_greater() {
        assert_eq!(
            vec![42, 1, 2, 3, 4, 5, 6, 43].stalin_sorted(),
            [42, 43],
        );
    }

    #[test]
    fn trailing_unsorted_greater_continued() {
        assert_eq!(
            vec![42, 1, 2, 3, 4, 5, 6, 43, 1].stalin_sorted(),
            [42, 43],
        );
    }

    #[test]
    fn trailing_unsorted_multi() {
        assert_eq!(
            vec![1, 42, 1, 2, 3, 4, 5, 6].stalin_sorted(),
            [1, 42],
        );
    }

    #[test]
    fn trailing_unsorted_multi_same() {
        assert_eq!(
            vec![1, 42, 42, 1, 2, 3, 4, 5, 6].stalin_sorted(),
            [1, 42, 42],
        );
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;

    macro_rules! ranges {
        ($($name:ident, $i:ty, $max:expr;)+) => {
            $(
                #[bench]
                fn $name(b: &mut test::Bencher) {
                    let mut vec: Vec<$i> = (1..=$max).collect();
                    b.iter(|| {
                        test::black_box(&mut vec).stalin_sort();
                        test::black_box(&mut vec);
                    });
                }
            )+
        };
    }

    ranges! {
        range_i032_1_0,      i32,  0;
        range_i032_1_1,      i32,  1;
        range_i032_1_10,     i32,  10;
        range_i032_1_100,    i32,  100;
        range_i032_1_1000,   i32,  1000;
        range_i032_1_10000,  i32,  10000;
        range_i064_1_0,      i64,  0;
        range_i064_1_1,      i64,  1;
        range_i064_1_10,     i64,  10;
        range_i064_1_100,    i64,  100;
        range_i064_1_1000,   i64,  1000;
        range_i064_1_10000,  i64,  10000;
        range_i128_1_0,      i128, 0;
        range_i128_1_1,      i128, 1;
        range_i128_1_10,     i128, 10;
        range_i128_1_100,    i128, 100;
        range_i128_1_1000,   i128, 1000;
        range_i128_1_10000,  i128, 10000;
    }
}
