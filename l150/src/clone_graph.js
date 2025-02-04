class _Node {
  constructor(val, neighbors) {
    this.val = val == undefined ? 0 : val;
    this.neighbors = neighbors == undefined ? [] : neighbors;
  }
}

/**
 * Time complexity: O(n)
 * Space complexity: O(n)
 * @param {_Node} node
 * @return {_Node}
 */
function cloneGrap(node) {
  const visited = new Map();

  const dfs = (node) => {
    if (!node) {
      return null;
    }

    if (visited.has(node)) {
      return visited.get(node);
    }

    const cnode = new Node(node.val);
    visited.set(node, cnode);

    cnode.neighbors = node.neighbors.map(dfs);
    return cnode;
  };

  return dfs(node);
}

cloneGrap();
