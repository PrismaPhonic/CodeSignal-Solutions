function areFollowingPatterns(strings, patterns) {
    let mapToPattern = {};
    let mapToString = {};
    
    for (let i = 0; i < strings.length; i++) {
        let string = strings[i];
        let pattern = patterns[i];
        if (mapToPattern[pattern]) {
            if (mapToPattern[pattern] !== string) return false;
        } else {
            if (mapToString[string]) return false;
        }        
        mapToPattern[pattern] = string;
        mapToString[string] = pattern;
    }
    
    return true;
}
