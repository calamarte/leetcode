/*/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

function ListNode(val) {
  this.val = val;
  this.next = null;
}

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycle = function (head) {
  let slow = head,
    fast = head;

  while (slow && fast?.next) {
    slow = slow.next;
    fast = fast.next.next;

    if (slow == fast) {
      return true;
    }
  }

  return false;
};

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycleSet = function (head) {
  const set = new Set();
  set.add(head);

  while (head?.next) {
    if (set.has(head.next)) {
      return true;
    }

    head = head.next;
    set.add(head);
  }

  return false;
};
