use std::collections::HashMap;

fn palindromeRearranging(inputString: String) -> bool {
    let mut freq = HashMap::new();
    let mut count = 0;
    for c in inputString.to_lowercase().chars() {
        if c != ' ' {
            count += 1;
            *freq.entry(c).or_insert(0) += 1;
        }
    }
    
    let odds = freq.values().filter(|v| *v % 2 != 0).count();

    if count % 2 == 0 { odds == 0 } else { odds == 1 }
}
