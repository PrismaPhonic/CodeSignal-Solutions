function simplifyPath(path) {
    let customPath = path;
    if (path === '/') return path;
    if (path[0] !== '/') customPath = '/' + path; 
    let stack = customPath.match(/\/[^/]+/g);
    let output = [];
    for (let p = 0; p < stack.length; p++) {
        if (stack[p] === '/..') {
            if (output.length === 0) {
                output.push('/');
            } else {
                output.pop();
            }
            continue;
        }
        if (stack[p] !== '/.') output.push(stack[p]);
    }
    if (output.length === 0) return '/';
    return output.join("").replace("//", "/");   
}
