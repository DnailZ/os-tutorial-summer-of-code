struct Solution();


static mut stack: String = String::new();
static mut remain: usize = 0;
static mut level: usize = 0;
static mut result: Option<Vec<String>> = None;

unsafe fn gen() {
    if remain > 0 {
        remain -= 1;
        level += 1;
        stack.push('(');
        gen();
        stack.pop();
        remain += 1;
        level -= 1;
    }
    if level > 0 {
        level -= 1;
        stack.push(')');
        gen();
        stack.pop();
        level += 1;
    }
    if remain == 0 && level == 0 {
        result.as_mut().unwrap().push(stack.clone());
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        unsafe{
            result = Some(Vec::new());
            remain = n as _;
            gen();
            result.take().unwrap()
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
            Solution::generate_parenthesis(3),
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()"),
            ]
        }
    }
}