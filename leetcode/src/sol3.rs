struct Solution ();

use std::collections::HashMap;
use std::cmp::{min, max};


use std::iter::Iterator;
use std::marker::Sized;

pub trait RangeIterator: Sized + Iterator {
    // type Item= <Self as Iterator>::Item;
    fn head_next(&mut self) -> Option<Self::Item>;
    fn tail_next(&mut self) -> Option<Self::Item>;
}


trait TraceingRangeIter : Iterator + Clone {
    fn range_iter_with_trace(self) -> TracingHeadTailIter<Self>{
        TracingHeadTailIter{
            head: self.clone(),
            tail: self,
            head_ind: 0,
            tail_ind: 0,
        }
    }
}

impl <T: Iterator + Clone> TraceingRangeIter for T {}

pub struct TracingHeadTailIter<I : Iterator> {
    head : I,
    tail : I,
    pub head_ind: usize,
    pub tail_ind: usize,
}

impl<I : Iterator> RangeIterator for TracingHeadTailIter<I> {
    
    fn head_next(&mut self) -> Option<Self::Item>{
        self.head_ind += 1;
        self.head.next()
    }

    fn tail_next(&mut self) -> Option<Self::Item> {
        self.tail_ind += 1;
        if self.head_ind == self.tail_ind{
            None
        } else {
            self.tail.next()
        }
    }
}

impl<I : Iterator> Iterator for TracingHeadTailIter<I>{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.tail_next()
    }
}

impl<I : Iterator> TracingHeadTailIter<I>{
    pub fn len(&self) -> usize {
        self.head_ind - self.tail_ind
    }
}


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = s.char_indices().range_iter_with_trace();
        let mut maps = HashMap::<char, usize>::new();

        let mut ret = 0;
        while let Some((i, ch_i)) = chars.head_next() {
            if let Some(next) = maps.get(&ch_i) {
                let next = *next;

                while let Some((j, ch_j)) = chars.tail_next() {
                    if j == next {
                        break;
                    }
                    maps.remove(&ch_j);
                }
            }
            maps.insert(ch_i, i);
            ret = max(ret, chars.len())
        }

        ret as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!{
            Solution::length_of_longest_substring("asdfasdg".into()),
            5
        }
        assert_eq!{
            Solution::length_of_longest_substring("abcabcbb".into()),
            3
        }
    }
}