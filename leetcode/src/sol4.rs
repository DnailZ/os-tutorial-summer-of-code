struct Solution();

use std::iter::*;

struct OrdIter<T: Iterator>{
    nums1: Peekable<T>,
    nums2: Peekable<T>,
}

impl<P: Ord, T: Iterator<Item=P>> Iterator for OrdIter<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        let n1 = if let Some(n1) = self.nums1.peek() { n1 } else {
            return self.nums2.next();
        };
        
        let n2 = if let Some(n2) = self.nums2.peek() { n2 } else {
            return self.nums1.next();
        };

        if n1 < n2 {
            self.nums1.next()
        } else {
            self.nums2.next()
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len();
        let mut iter = OrdIter {
            nums1: nums1.into_iter().peekable(),
            nums2: nums2.into_iter().peekable(),
        };
        if n % 2 == 0 {
            (iter.nth(n / 2 - 1).unwrap() + iter.next().unwrap()) as f64 / 2.
        } else {
            iter.nth((n-1) / 2).unwrap() as _
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test1() {
        assert_eq!{
            Solution::find_median_sorted_arrays(
                vec![1, 3], 
                vec![2]
            ), 2.
        }
    }
}