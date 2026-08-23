# Created by Olgierd Palasz at 2026/08/22 10:49
# leetgo: dev
# https://leetcode.com/problems/remove-nth-node-from-end-of-list/

from typing import *
from leetgo_py import *

# @lc code=begin

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        left = head #prev remove
        right = head


        for _ in range(n):
            if right:
                right = right.next

        if right is None:
            head = head.next
            return head

        if right.next is None:
            left.next = None
            return head
            

        while right.next:
            left = left.next
            right = right.next

        left.next = right
        return head

        

            

        

# @lc code=end
if __name__ == "__main__":
    head: ListNode = deserialize("ListNode", read_line())
    n: int = deserialize("int", read_line())
    ans = Solution().removeNthFromEnd(head, n)
    print("\noutput:", serialize(ans, "ListNode"))
