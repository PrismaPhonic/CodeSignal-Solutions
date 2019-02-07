function swapDiagonals(matrix) {

    for (let i = 0; i < matrix.length; i++) {
        [matrix[i][i], matrix[i][matrix.length-1-i]] = [matrix[i][matrix.length-1-i], matrix[i][i]];
    }     
    
    return matrix;
}
