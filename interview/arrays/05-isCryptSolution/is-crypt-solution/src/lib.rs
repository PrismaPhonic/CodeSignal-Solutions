use std::collections::HashMap;

fn is_crypt_solution(crypt: Vec<String>, solution: Vec<Vec<char>>) -> bool {
    let map = matrix_into_hashmap(solution);
    let mut numbers: Vec<i64> = Vec::new();

    for word in crypt.iter() {
        let mut num_str = String::new();
        for (i, c) in word.chars().enumerate() {
            let num_char = *map.get(&c).unwrap();

            // fail fast - check for leading zero
            if i == 0 && num_char == '0' && word.len() > 1 {
                return false;
            }

            num_str.push(num_char);
        }
        numbers.push(num_str.parse::<i64>().unwrap());
    }

    numbers[0] + numbers[1] == numbers[2]
}

fn matrix_into_hashmap(matrix: Vec<Vec<char>>) -> HashMap<char, char> {
    let mut map = HashMap::new();

    for pair in matrix {
        map.insert(pair[0], pair[1]);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let crypt: Vec<String> = vec!["SEND".to_string(), "MORE".to_string(), "MONEY".to_string()];
        let solution: Vec<Vec<char>> = vec![
            vec!['O', '0'],
            vec!['M', '1'],
            vec!['Y', '2'],
            vec!['E', '5'],
            vec!['N', '6'],
            vec!['D', '7'],
            vec!['R', '8'],
            vec!['S', '9'],
        ];
        assert!(is_crypt_solution(crypt, solution));
    }
}
