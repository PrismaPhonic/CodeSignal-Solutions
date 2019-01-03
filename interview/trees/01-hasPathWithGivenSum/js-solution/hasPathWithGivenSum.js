//
// Definition for binary tree:
// function Tree(x) {
//   this.value = x;
//   this.left = null;
//   this.right = null;
// }
function hasPathWithGivenSum(root, sum) {
    const dfs = (node, sum, s) => {
        s += node.value;
        if (node.left !== null) {
            if (dfs(node.left, sum, s) === true) {
                return true;
            }
        }

        if (node.right !== null) {
            if (dfs(node.right, sum, s) === true) {
                return true;
            }
        }

        if (node.left === null && node.right === null) {
            if (sum === s) return true;
        }
        return false;
    }

    if (root !== null) return dfs(root, sum, 0);
    return false;
}
