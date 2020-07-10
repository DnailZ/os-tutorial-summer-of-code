struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn newlist(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode{val, next}))
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        adds(l1, l2, 0)
    }
}

fn simple_div10(a: i32) -> (i32, i32) {
    if a >= 10 {
        (1, a - 10)
    } else {
        (0, a)
    }
}

pub fn adds(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    c: i32,
) -> Option<Box<ListNode>> {
    if l1 == None && l2 == None && c == 0 {
        return None
    }
    println!("{:?} {:?} {}", l1, l2, c);
    let mut val1 = 0;
    let mut next1 = None;
    let mut val2 = 0;
    let mut next2 = None;

    if let Some(n1) = l1 {
        let ListNode{val:v1, next:n1} = *n1;
        val1 = v1;
        next1 = n1;
    }

    if let Some(n2) = l2 {
        let ListNode{val:v2, next:n2} = *n2;
        val2 = v2;
        next2 = n2;
    }
    let (c, val) = simple_div10(val1 + val2 + c);
    let next = adds(next1, next2, c);

    Some(Box::new(ListNode{val, next}))
}

#[cfg(test)]
mod test{
    use super::*;
    use std::assert_eq;
    use std::assert;
    #[test]
    fn test1() {
        let l1 = newlist(4, newlist(0, newlist(8, None)));
        let l2 = newlist(1, newlist(2, newlist(6, None)));
        let l3 = newlist(5, newlist(2, newlist(4, newlist(1, None))));
        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            l3
        )
    }
}