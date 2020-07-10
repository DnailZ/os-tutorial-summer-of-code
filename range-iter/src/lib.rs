use std::iter::Iterator;
use std::marker::Sized;

pub trait RangeIterator: Sized + Iterator {
    // type Item= <Self as Iterator>::Item;
    fn head_next(&mut self) -> Option<Self::Item>;
    fn tail_next(&mut self) -> Option<Self::Item>;

    fn head_iter(&mut self) -> HeadIter<Self> {
        HeadIter(self)
    }
    fn tail_iter(&mut self) -> TailIter<Self> {
        TailIter(self)
    }
}

pub struct HeadIter<'a, I : RangeIterator>(&'a mut I);
pub struct TailIter<'a, I : RangeIterator>(&'a mut I);

impl<'a, I:RangeIterator> Iterator for HeadIter<'a, I> {
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head_next()
    }
}

impl<'a, I:RangeIterator> Iterator for TailIter<'a, I> {
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.tail_next()
    }
}

trait RangeIter : Iterator + Clone + Eq {
    fn range_iter(self) -> HeadTailIter<Self>{
        HeadTailIter{
            head: self.clone(),
            tail: self,
        }
    }

    fn range_iter_with_len(self) -> HeadTailLenIter<Self>{
        HeadTailLenIter{
            head: self.clone(),
            tail: self,
            len: 0,
        }
    }
}
impl<T: ?Sized> RangeIter for T where T: Iterator + Clone + Eq {}


pub struct HeadTailIter<I : Iterator + Eq> {
    head : I,
    tail : I,
}

impl<I : Iterator + Eq> RangeIterator for HeadTailIter<I> {

    fn head_next(&mut self) -> Option<Self::Item>{
        self.head.next()
    }

    fn tail_next(&mut self) -> Option<Self::Item> {
        if self.head == self.tail{
            None
        } else {
            self.tail.next()
        }
    }
}

impl<I : Iterator + Eq> Iterator for HeadTailIter<I>{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.tail_next()
    }
}

pub struct HeadTailLenIter<I : Iterator + Eq> {
    head : I,
    tail : I,
    len: usize,
}

impl<I : Iterator + Eq> RangeIterator for HeadTailLenIter<I> {
    
    fn head_next(&mut self) -> Option<Self::Item>{
        self.len += 1;
        self.head.next()
    }

    fn tail_next(&mut self) -> Option<Self::Item> {
        self.len -= 1;
        if self.head == self.tail{
            None
        } else {
            self.tail.next()
        }
    }
}

impl<I : Iterator + Eq> Iterator for HeadTailLenIter<I>{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.tail_next()
    }
}

impl<I : Iterator + Eq> HeadTailLenIter<I>{
    pub fn len(&self) -> usize {
        self.len
    }
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
