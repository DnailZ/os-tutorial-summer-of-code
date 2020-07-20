
struct Solution{}


impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut vec = Vec::new();
        while x != 0 {
            vec.push(x % 10);
            x /= 10;
        }
        let mut ret : i32 = 0;
        for digit in vec {
            let result = ret.checked_mul(10).and_then(|r| r.checked_add(digit) );
            if let Some(non_overflow) = result {
                ret = non_overflow;
            } else {
                return 0;
            }
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test1() {
        assert_eq!{
            Solution::reverse(123),
            321
        }
        assert_eq!{
            Solution::reverse(120),
            21
        }
        assert_eq!{
            Solution::reverse(-123),
            -321
        }
        assert_eq!{
            Solution::reverse(2147483647),
            0
        }
        assert_eq!{
            Solution::reverse(1231234),
            4321321
        }

    }
}

