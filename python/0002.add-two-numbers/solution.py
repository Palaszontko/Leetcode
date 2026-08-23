# Created by Olgierd Palasz at 2026/08/23 13:18
# leetgo: dev
# https://leetcode.com/problems/add-two-numbers/

from typing import *
from leetgo_py import *

# @lc code=begin
from math import ceil
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0,None)
        head = dummy
        leading = 0
        while l1 or l2: 
            if l1 and l2:
                sum = l1.val + l2.val + leading
            elif l1 and not l2:
                sum = l1.val + leading
            elif not l1 and l2:    
                sum = l2.val + leading

            if sum >= 10:
                leading = sum / 10
                sum %= 10
            else:
                leading = 0

            dummy.next = ListNode(int(sum), None)
            dummy = dummy.next

            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        if leading != 0:
            dummy.next = ListNode(int(leading), None)
        
        return head.next

            
            


        

# @lc code=end

if __name__ == "__main__":
    l1: ListNode = deserialize("ListNode", read_line())
    l2: ListNode = deserialize("ListNode", read_line())
    ans = Solution().addTwoNumbers(l1, l2)
    print("\noutput:", serialize(ans, "ListNode"))
