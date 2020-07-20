struct Solution {}

impl Solution {
    pub fn my_atoi(s : String) -> i32 {
        let mut iter = s.bytes().peekable();

        // dealing with spaces.
        while let Some(b' ') = iter.peek() {
            iter.next();
        }

        let mut negative = false; 

        if let Some(ch) = iter.peek() {
            if *ch == b'-' {
                negative = true;
                iter.next();
            } else if *ch == b'+' {
                iter.next();
            }
        }

        let mut ret : i32 = 0;
        loop {
            if let Some(ch) = iter.next() {
                if (b'0'..=b'9').contains(&ch) {
                    let result = ret.checked_mul(10).and_then(|r|
                        r.checked_add((ch - b'0') as _)
                    );

                    if let Some(non_overflow) = result {
                        ret = non_overflow
                    } else {
                        return if negative { i32::MIN } else { i32::MAX };
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if negative { -ret } else { ret }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test1() {
        assert_eq!{
            Solution::my_atoi("   -42".into()),
            -42
        }
        assert_eq!{
            Solution::my_atoi("4193 with words".into()),
            4193
        }
        assert_eq!{
            Solution::my_atoi("4193 with words".into()),
            4193
        }
        assert_eq!{
            Solution::my_atoi("-91283472332".into()),
            i32::MIN
        }
    }
}