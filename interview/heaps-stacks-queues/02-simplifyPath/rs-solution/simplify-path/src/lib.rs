fn simplifyPath(path: String) -> String {
    let mut output: Vec<String> = Vec::new();
    for dir in path.split("/") {
        match dir {
            "." | "" => (),
            ".." => {
                output.pop();
            }
            d => output.push(format!("/{}", d)),
        }
    }
    if output.len() == 0 {
        return "/".to_string();
    };
    output.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let path = "/home/a/./x/../b//c/".to_string();
        assert_eq!(simplifyPath(path), "/home/a/b/c".to_string());
    }

    #[test]
    fn test_20() {
        let path = "meh/moh/beh/././././././././../././././././texbehbeh".to_string();
        assert_eq!(simplifyPath(path), "/meh/moh/texbehbeh".to_string());
    }
}
