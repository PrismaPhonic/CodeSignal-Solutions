## Simplify Path

## JS Solution

This solution has a few gotchas to catch some really bizarre test cases that get
thrown at us.  Fundamentally it works by creating a stack by using a simple
regex `/\/[^/]+/g` - this regex basically starts a capture at a literal forward
slash and then captures any non-forward slash repeatedly until (and not
including) the next forward slash.  This gives us an array of matches.  We then
increment through this array of matches and if we get a `/..` that means go back
a directory so we pop off our output array.  if we get `/.` then we do nothing,
and otherwise we push onto our output array.  In the end we join and then run a
simple replace to clean up any duplicate forward slashes

solution:

```javascript
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
```
