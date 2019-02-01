function sudoku2(grid) {
  if (!checkRows(grid)) return false;
  if (!checkColumns(grid)) return false;
  if (!checkSubGrids(grid)) return false;

  return true;
}

function checkRows(grid) {
  for (let row of grid) {
    let nums = new Set();
    for (let e of row) {
      if (e === '.') continue;
      if (nums.has(e)) return false;
      nums.add(e);
    }
  }
  return true;
}

function checkColumns(grid) {
  for (let i = 0; i < grid[0].length; i++) {
    let nums = new Set();
    for (let j = 0; j < grid.length; j++) {
      let e = grid[j][i];
      if (e === '.') continue;
      if (nums.has(e)) return false;
      nums.add(e);
    }
  }
  return true;
}

function checkSubGrids(grid) {
  // generate starting coordinate for each subgrid and run
  // each through a sudoku check
  for (let row = 0; row < grid.length; row += 3) {
    for (let column = 0; column < grid[0].length; column += 3) {
      if (!checkSubGrid(row, column, grid)) return false;
    }
  }
  return true;
}

// takes starting coordinates
function checkSubGrid(row, column, grid) {
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
