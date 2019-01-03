// Definition for binary tree:
struct Tree<T> {
    value: T,
    left: TreeNode<T>,
    right: TreeNode<T>
}
impl<T> Tree<T> {
    fn new(value: T) -> Tree<T> {
        Tree { value, left: None, right: None }
    }
}
type TreeNode<T> = Option<Box<Tree<T>>>;

fn hasPathWithGivenSum(t: TreeNode<i32>, s: i32) -> bool {
    
    fn dfs(node: &Box<Tree<i32>>, mut s: i32, sum: i32) -> bool {
        s += node.value;
        if let Some(n1) = &node.left {
            if dfs(n1, s, sum) == true {
                return true;
            };
        }
        
        if let Some(nr) = &node.right {
            if dfs(nr, s, sum) == true {
                return true;
            };
        }
        
        if let None = node.left {
            if let None = node.right {
                if s == sum {
                    return true;
                }
            }
        }
        false
    }
    
    if let Some(rn) = &t {
        return dfs(rn, 0, s)
    }
    
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
