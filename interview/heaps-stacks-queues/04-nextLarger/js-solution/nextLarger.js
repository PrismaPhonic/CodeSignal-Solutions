function nextLarger(a) {
    let len = a.length;
    
    let output = Array(len).fill(-1);
    let stack = [];
    
    for (let i = len-1; i >= 0; i--) {
        while (stack.length > 0) {
            let num = stack.pop();
            if (num > a[i]) {
                output[i] = num;
                stack.push(num);
                break;
            }
        }
        stack.push(a[i]);
    }
    
    return output;
}
