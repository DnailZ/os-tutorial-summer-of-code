struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for x in s.chars() {
            match x {
                '('| '[' | '{' => v.push(x),
                ')' => if v.pop() != Some('(') { return false; },
                ']' => if v.pop() != Some('[') { return false; },
                '}' => if v.pop() != Some('{') { return false; },
                _ => (),
            }
        }
        v.len() == 0
    }
}

