function reverseOnDiagonals(matrix) {

    for (let i = 0; i < matrix.length/2; i++) {
        [matrix[i][i], matrix[matrix.length-1-i][matrix.length-1-i]] = [matrix[matrix.length-1-i][matrix.length-1-i], matrix[i][i]];
        
        [matrix[i][matrix.length-1-i], matrix[matrix.length-1-i][i]] = [matrix[matrix.length-1-i][i], matrix[i][matrix.length-1-i]];
    }
    
    return matrix;
}
