struct Solution{}

use std::iter::DoubleEndedIterator;

struct DoubleEndPeekable<T: DoubleEndedIterator> {
    iter: T,
    temp: Option<Option<T::Item>>,
    temp_back: Option<Option<T::Item>>,
}

impl<T: DoubleEndedIterator> Iterator for DoubleEndPeekable<T> {
    type Item = T::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.temp.take(){
            result
        } else {
            self.iter.next()
        }
    }
}
impl<T: DoubleEndedIterator+ExactSizeIterator> ExactSizeIterator for DoubleEndPeekable<T> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for DoubleEndPeekable<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.temp_back.take(){
            result
        } else {
            self.iter.next_back()
        }
    }
}


impl<T: DoubleEndedIterator> DoubleEndPeekable<T> {
    #[inline]
    pub fn peek(&mut self) -> Option<&T::Item> {
        let iter = &mut self.iter;
        self.temp.get_or_insert_with(|| iter.next()).as_ref()
    }
    #[inline]
    pub fn peek_back(&mut self) -> Option<&T::Item> {
        let iter = &mut self.iter;
        self.temp_back.get_or_insert_with(|| iter.next_back()).as_ref()
    }
    #[inline]
    pub fn new(iter : T) -> DoubleEndPeekable<T> {
        Self {
            iter,
            temp: None,
            temp_back: None,
        }
    }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut iter = DoubleEndPeekable::new(height.iter().enumerate());
        let mut max_ret = 0i32;

        loop {
            let (i, v1) = if let Some(v1) = iter.peek() {*v1} else {
                break;
            };
            let (j, v2) = if let Some(v2) = iter.peek_back() {v2} else {
                break;
            };
            // dbg!(i, j, v1, v2);

            let area = (j - i) as i32 * std::cmp::min(v1, v2);
            max_ret = std::cmp::max(max_ret, area);

            if v1 > v2 {
                iter.next_back();
            } else {
                iter.next();
            }
        }

        max_ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test1() {
        assert_eq!{
            Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),
            49
        }
        assert_eq!{
            Solution::max_area(vec![2,3,4,5,18,17,6]),
            17
        }
        assert_eq!{
            Solution::max_area((1..=15000).collect()),
            56250000
        }
    }
}