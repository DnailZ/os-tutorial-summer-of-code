struct Solution{}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        if strs.len() == 0{
            return prefix;
        }
        for c in strs[0].chars() {
            prefix.push(c);
            for i in 1..strs.len(){
                if !strs[i].starts_with(&prefix) {
                    prefix.pop();
                    return prefix;
                }
            }
        }
        prefix 
    }
}