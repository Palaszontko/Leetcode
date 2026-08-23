# Created by Olgierd Palasz at 2026/08/23 21:00
# leetgo: dev
# https://leetcode.com/problems/copy-list-with-random-pointer/

from typing import *
from leetgo_py import *

# @lc code=begin

"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[ListNode]') -> 'Optional[ListNode]':
        copied = {None : None}

        node = head
        while node:
            copy = Node(node.val)
            copied[node] = copy
            node = node.next

        node = head
        while node:
            copy = copied[node]
            copy.next = copied[node.next]
            copy.random = copied[node.random]
            node = node.next

        return copied[head]

# @lc code=end

# Warning: this is a manual question, the generated test code may be incorrect.
if __name__ == "__main__":
    head: ListNode = deserialize("ListNode", read_line())
    ans = Solution().copyRandomList(head)
    print("\noutput:", serialize(ans, "ListNode"))
