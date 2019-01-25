## Simplify Path

## JS Solution

After coming up with a simpler solution in Rust, I refactored my javascript
solution which is quitte simple.  We just split on `/`.  If our iterable is 
a blank string or a single `.` then we don't do anything. If it's a `..` then
we pop off our output stack.  Lastly in any other case we push onto our output
stack and prepend with a forward slash.

We have one small case check at the end if we ended up with nothing in our stack
then we return a single forward slash.

solution:

```javascript
function simplifyPath(path) {
  let output = [];
  for (let dir of path.split("/")) {
    switch (dir) {
      case '.':
      case '':
        break;
      case '..':
        output.pop();
        break;
      default:
        output.push("/" + dir);
    }
  }
  if (output.length === 0) return "/";
  return output.join("");
}
```
