function crossingSum(matrix, r, c) {
    return sumColumn(matrix, c) + sumRow(matrix, r) - matrix[r][c];
}

function sumColumn(matrix, c) {
    return matrix.map(row => row[c]).reduce((a, b) => a + b, 0);
}

function sumRow(matrix, r) {
    return matrix[r].reduce((a, b) => a + b, 0);
}
