# Created by Olgierd Palasz at 2026/08/21 20:43
# leetgo: dev
# https://leetcode.com/problems/reorder-list/

from typing import *
from leetgo_py import *

# @lc code=begin

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        slow = fast = head

        while fast.next != None and fast.next.next != None:
            fast = fast.next.next
            slow = slow.next

        tmp = slow.next
        slow.next = None
        # slow is now mid
        curr = tmp
        prev = None

        #reverse second half
        while curr != None: 
            nextNode = curr.next
            curr.next = prev
            prev = curr
            curr = nextNode

        startFirst = head
        startSecond = prev


        while startSecond != None:
            t1 = startFirst.next
            t2 = startSecond.next

            startFirst.next = startSecond
            startSecond.next = t1

            startFirst = t1
            startSecond = t2
        

# @lc code=end

if __name__ == "__main__":
    head: ListNode = deserialize("ListNode", read_line())
    Solution().reorderList(head)
    ans = head
    print("\noutput:", serialize(ans, "ListNode"))
