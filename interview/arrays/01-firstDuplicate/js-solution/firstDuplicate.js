function firstDuplicate(a) {
    let occurance = new Set();
    
    for (let num of a) {
        if (!occurance.has(num)) {
            occurance.add(num)
        } else {
            return num;
        }
    }
    
    return -1;
}
