## Simplify Path

## RS Solution

This solution seemed much easier than trying to do it in javascript and I can't
seem to tell you why I didn't run into nearly as many bugs using rust. All we
had to do here is do was split on a forward slash, and then match each iteration
of that split against a simple check - if it's a single `.` or nothing at all
(such as when we have `//` in the path) then don't do anything.  If we match
`..` then pop off our output.  Otherwise anything else we push onto our output,
and format it with a prepended forward slash.  

We simply case check if we ended up not pushing anything into our output vector,
and if so just return a single forward slash (root directory).  Otherwise just
join the vector into an owned String type and return.

solution:

```rust
fn simplifyPath(path: String) -> String {
    let mut output: Vec<String> = Vec::new();
    for dir in path.split("/") {
        match dir {
            "." | "" => (),
            ".." => { output.pop(); },
            d => output.push(format!("/{}", d)),
        }
    }
    if output.len() == 0 { return "/".to_string() };
    output.join("")
}
```
