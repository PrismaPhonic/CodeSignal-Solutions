fn reverseInParentheses(inputString: String) -> String {
    let mut output: Vec<char> = Vec::new();
    let inputString: Vec<char> = inputString.chars().collect();
    let mut i = 0;
    
    while i < inputString.len() {
        let c = inputString[i];
        
        if c == '(' {
            // substring recursion
            i += 1;
            output.extend(recurse(&mut i, vec![], &inputString));
        } else {
            output.push(c);
            i += 1;
        }
    }
    
    output.into_iter().collect()
}

fn recurse(idx: &mut usize, mut stack: Vec<char>, inputString: &Vec<char>) -> Vec<char> {
        let mut c = inputString[*idx];
        while c != ')' {
            // recurse again
            if c == '(' {
                *idx += 1;
                stack.extend(recurse(idx, vec![], inputString));
            } else {
                stack.push(c);
                *idx += 1;
            }            
            c = inputString[*idx];      
        }
        
        *idx += 1;
        stack.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_test() {
        let string = "foo(bar(baz))blim".to_string();
        assert_eq!(reverseInParentheses(string), "foobazrabblim".to_string());
    }
}
