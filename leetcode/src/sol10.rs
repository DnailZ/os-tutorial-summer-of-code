struct Solution{}

fn ones_to_roman(i : i32) -> &'static str {
    match i {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => "ERR",
    }
}

fn tens_to_roman(i : i32) -> &'static str {
    match i {
        0 => "",
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => "ERR",
    }
}

fn hundreds_to_roman(i : i32) -> &'static str {
    match i {
        0 => "",
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => "ERR",
    }
}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let ones = num % 10;
        num /= 10;
        let tens = num % 10;
        num /= 10;
        let hundreds = num % 10;
        num /= 10;
        let thousands = num as usize;

        let mut ret = "M".repeat(thousands);
        ret.push_str(hundreds_to_roman(hundreds));
        ret.push_str(tens_to_roman(tens));
        ret.push_str(ones_to_roman(ones));
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
            Solution::int_to_roman(1994),
            String::from("MCMXCIV")
        }
    }
}