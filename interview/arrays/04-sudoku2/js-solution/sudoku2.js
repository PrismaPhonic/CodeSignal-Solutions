function sudoku2(grid) {
  return checkRowsAndColumns(grid) && checkSubgrids(grid)
}

function checkRowsAndColumns(grid) {
  for (let i = 0; i < 9; i++) {
    let rowCount = new Set();
    let colCount = new Set();
    for (let j = 0; j < 9; j++) {
      let e = grid[i][j];
      if (e !== '.') {
        if (rowCount.has(e)) return false;
        rowCount.add(e);
      }
      e = grid[j][i]
      if (e === '.') continue;
      if (colCount.has(e)) return false;
      colCount.add(e);
    }
  }
  return true;
}

function checkSubgrids(grid) {
  for (let row = 0; row < 9; row += 3) {
    for (let column = 0; column < 9; column += 3) {
      if (!checkSubGrid(row, column, grid)) return false;
    }
  }
  return true;
}

// takes starting coordinates
function checkSubgrid(row, column, grid) {
  let nums = new Set();

  for (let i = row; i < row + 3; i++) {
    for (let j = column; j < column + 3; j++) {
      let e = grid[i][j];
      if (e === '.') continue;
      if (nums.has(e)) return false;
      nums.add(e);
    }
  }
  
  return true;
}
