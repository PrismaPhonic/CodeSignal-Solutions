function firstNotRepeatingCharacter(s) {
    let occurances = new Set();
    let uniq = new Set();
    
    for (const char of s) {
        if (!occurances.has(char)) {
            occurances.add(char);
            uniq.add(char);
        } else {
            uniq.delete(char);
        }
    }
    
    if (uniq.size === 0) return '_';
    
    // we have at least one uniq so create
    // an iterator and immediately return first
    // char
    for (let char of uniq) {
        return char;
    }
}
