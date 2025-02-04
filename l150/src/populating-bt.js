/**
 * Definition for a _Node.
 * function _Node(val, left, right, next) {
 *    this.val = val === undefined ? null : val;
 *    this.left = left === undefined ? null : left;
 *    this.right = right === undefined ? null : right;
 *    this.next = next === undefined ? null : next;
 * };
 */

/**
 * @param {_Node} root
 * @return {_Node}
 */
var connect = function (root) {
  const deq = [root];

  while (deq.length) {
    const len = deq.length;

    for (let i = 0; i < len; i++) {
      const node = deq.shift();

      if (node) {
        if (node.left) deq.push(node.left);
        if (node.right) deq.push(node.right);

        if (i < len - 1) {
          node.next = deq[0] || null;
        }
      }
    }
  }

  return root;
};
