use std::collections::HashSet;

fn hasDeadlock(connections: Vec<Vec<i32>>) -> bool {
    
    for i in 0..connections.len() {
        if recurse(i, &mut HashSet::new(), &connections) {
            return true;
        }
    }
       
    false
}

fn recurse(idx: usize, seen: &mut HashSet<usize>, connections: &Vec<Vec<i32>>) -> bool {
    let top = &connections[idx];

    for process in top {
        let process = *process as usize;
        if !seen.insert(process) { return true };
        if recurse(process, seen, connections) {
            return true;
        };
        seen.remove(&process);
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        let processes = vec![
            vec![2,3,5], 
            vec![0,2,5,4,3], 
            vec![3], 
            vec![5], 
            vec![3,5], 
            vec![]];

        assert_eq!(hasDeadlock(processes), false);
    }
}
