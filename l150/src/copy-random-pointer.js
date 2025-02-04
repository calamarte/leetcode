/**
 * // Definition for a _Node.
 * function _Node(val, next, random) {
 *    this.val = val;
 *    this.next = next;
 *    this.random = random;
 * };
 */

/**
 * @param {_Node} head
 * @return {_Node}
 */
var copyRandomList = function (head) {
  const map = new Map();
  let tail = head;
  while (tail) {
    map.set(tail, new _Node(tail.val, null, null));
    tail = tail.next;
  }

  tail = head;
  while (tail) {
    const new_node = map.get(tail);

    if (tail.next) {
      const new_next_node = map.get(tail.next);
      new_node.next = new_next_node;
    }

    if (tail.random) {
      const new_random_node = map.get(tail.random);
      new_node.random = new_random_node;
    }

    tail = tail.next;
  }

  return map.get(head);
};
