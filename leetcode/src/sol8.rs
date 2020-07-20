
struct Solution{}


impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        let mut vec = Vec::new();
        if x < 0 { return false; }
        while x > 0 {
            vec.push(x % 10);
            x /= 10;
        }
        let mut iter = vec.iter();
        loop {
            if iter.len() <= 1 {
                return true;
            }
            if let (Some(v1), Some(v2)) = (iter.next(), iter.next_back()) {
                if v1 != v2 {
                    return false;
                }
            }
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
            Solution::is_palindrome(12321), true
        }
        assert_eq!{
            Solution::is_palindrome(-12321), false
        }
        assert_eq!{
            Solution::is_palindrome(10), false
        }
    }
}