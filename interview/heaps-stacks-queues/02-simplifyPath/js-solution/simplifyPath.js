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
