function rotateImage(a) {
    let len = a.length;
    for (let layer = 0; layer < len/2; layer++) {
        let first = layer;
        let last = len - 1 - layer;
        for (let i = first; i < last; i++) {
            let offset = i - first;
            let temp = a[first][i];
            
            a[first][i] = a[last-offset][first];
            a[last-offset][first] = a[last][last-offset];
            a[last][last-offset] = a[i][last];
            a[i][last] = temp;
        }
    }
    return a;
}
