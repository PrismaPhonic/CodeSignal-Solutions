type Formation = Vec<Vec<String>>;

fn volleyball_positions(mut form: Formation, k: i32) -> Formation {
    for _ in 0..k%6 {
        let temp = form[0][1].clone();
        form[0][1] = form[1][2].clone();
        form[1][2] = form[3][2].clone();
        form[3][2] = form[2][1].clone();
        form[2][1] = form[3][0].clone();
        form[3][0] = form[1][0].clone();
        form[1][0] = temp;
    }
    
    form
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let formation = vec![
            vec!["empty".to_string(),"Player5".to_string(),"empty".to_string()], 
            vec!["Player4".to_string(),"empty".to_string(),"Player2".to_string()], 
            vec!["empty".to_string(),"Player3".to_string(),"empty".to_string()], 
            vec!["Player6".to_string(),"empty".to_string(),"Player1".to_string()]];

        let output = vec![
			 vec!["empty".to_string(), "Player1".to_string(), "empty".to_string()], 
			 vec!["Player2".to_string(), "empty".to_string(), "Player3".to_string()], 
			 vec!["empty".to_string(), "Player4".to_string(), "empty".to_string()], 
			 vec!["Player5".to_string(), "empty".to_string(), "Player6".to_string()]];

        assert_eq!(volleyball_positions(formation, 2), output);
    }
}
