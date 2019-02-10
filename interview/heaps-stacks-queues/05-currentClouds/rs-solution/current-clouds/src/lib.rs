use std::collections::HashSet;

type SkyMap = Vec<Vec<char>>;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point(usize, usize);

fn countClouds(skyMap: SkyMap) -> i32 {    
    let height = skyMap.len();
    
    if height == 0 { return 0 };
    
    let width = skyMap[0].len();

    let mut seen: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = Vec::new();
    let mut count = 0;
    
    // dfs closure for use later when triggering dfs from cloud find
    let mut dfs = |stack: &mut Vec<Point>, seen: &mut HashSet<Point>| {
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let Point(x, y) = top;

            if x > 0 && skyMap[y][x-1] == '1' {
                let left = Point(x-1, y);
                if seen.insert(left.clone()) {
                    stack.push(left);
                }
            }
            if x < width-1 && skyMap[y][x+1] == '1' {
                let right = Point(x + 1, y);
                if seen.insert(right.clone()) {
                    stack.push(right);
                }
            }
            if y > 0 && skyMap[y-1][x] == '1' {
                let up = Point(x, y-1);
                if seen.insert(up.clone()) {
                    stack.push(up);
                }
            }
            if y < height-1 && skyMap[y+1][x] == '1' {
                let down = Point(x, y+1);
                if seen.insert(down.clone()) {
                    stack.push(down);
                }
            }
        }
    };
    
    // If we have already seen the point, continue looping, otherwise
    // (haven't seen it yet), see if it's a 1. if so, we found a cloud. increment count and 
    // trigger depth first search, updating seen with each neighbor '1' until we exhaust search and pop
    // back out
    for y in 0..height {
        for x in 0..width {
            let point = Point(x, y); 
            if !seen.insert(point.clone()) {
                continue;
            } else {
                if skyMap[y][x] == '1' {
                    count += 1;
                    // trigger search
                    stack.push(point);
                    
                    dfs(&mut stack, &mut seen);
                }
            }
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
