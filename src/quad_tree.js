// Definition for a QuadTree node.
function _Node(val, isLeaf, topLeft, topRight, bottomLeft, bottomRight) {
  this.val = val;
  this.isLeaf = isLeaf;
  this.topLeft = topLeft;
  this.topRight = topRight;
  this.bottomLeft = bottomLeft;
  this.bottomRight = bottomRight;
}

/**
 * Time complexity: O(n^2 log n)
 * Space complexity: O(n^2)
 * @param {number[][]} grid
 * @return {_Node}
 */
var construct = function (grid) {
  const dfs = (startY, startX, len) => {
    if (len == 1) {
      return new _Node(grid[startY][startX], true);
    }

    const yIndexs = Array.from({ length: len }, (_, i) => startY + i);
    const xIndexs = Array.from({ length: len }, (_, i) => startX + i);

    const template = grid[startY][startX];

    let areSame = true;
    for (const y of yIndexs) {
      for (const x of xIndexs) {
        if (grid[y][x] != template) {
          areSame = false;
          break;
        }
      }
    }

    if (areSame) {
      return new _Node(template, true);
    }

    const node = new _Node(0, false);
    const mid = len / 2;

    node.topLeft = dfs(startY, startX, mid);
    node.topRight = dfs(startY, startX + mid, mid);
    node.bottomLeft = dfs(startY + mid, startX, mid);
    node.bottomRight = dfs(startY + mid, startX + mid, mid);

    return node;
  };

  return dfs(0, 0, grid.length);
};

// prettier-ignore
let grid = [
  [0, 1],
  [1, 0]
];

grid = [
  [1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 1, 1, 1, 1],
  [1, 1, 1, 1, 1, 1, 1, 1],
  [1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0],
  [1, 1, 1, 1, 0, 0, 0, 0],
];

const result = construct(grid);
console.log(result);
