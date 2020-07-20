struct Solution{}

use std::iter::Peekable;
use std::str::Chars;

fn roman_to(iter : &mut Peekable<Chars>, base: char, five: char, ten: char) -> i32 {
    if Some(&base) == iter.peek() {
        iter.next();
        if Some(&five) == iter.peek() {
            iter.next();
            return 4;
        } else if Some(&ten) == iter.peek() {
            iter.next();
            return 9;
        } else if Some(&base) == iter.peek() {
            iter.next();
            if Some(&base) == iter.peek() {
                iter.next();
                return 3;
            }
            return 2;
        }
        return 1;
    } else if Some(&five) == iter.peek() {
        iter.next();
        if Some(&base) == iter.peek() {
            iter.next();
            if Some(&base) == iter.peek() {
                iter.next();
                if Some(&base) == iter.peek() {
                    iter.next();
                    return 8;
                }
                return 7;
            }
            return 6;
        }
        return 5;
    }
    0
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut chars = s.chars().peekable();
        let thousands = roman_to(&mut chars, 'M', '\0', '\0') * 1000;
        let hundreds = roman_to(&mut chars, 'C', 'D', 'M') * 100;
        let tens = roman_to(&mut chars, 'X', 'L', 'C') * 10;
        let ones = roman_to(&mut chars, 'I', 'V', 'X');
        thousands + hundreds + tens + ones
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test1() {
        assert_eq!{
            Solution::roman_to_int("MCMXCIV".into()),
            1994
        }
    }
}