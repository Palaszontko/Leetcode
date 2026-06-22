// Created by Olgierd Palasz at 2026/05/06 21:33
// leetgo: dev
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func evalRPN(tokens []string) int {
	stack := NewStack()

	for _, symbol := range tokens {
		value, err := strconv.Atoi(symbol)
		if err == nil {
			stack.Push(value)
		} else {
			a := stack.Pop()
			b := stack.Pop()
			stack.Push(Operation(b, a, symbol))
		}
	}
	return stack.Pop()
}

func Operation(val1 int, val2 int, operator string) int {
	if operator == "+" {
		return val1 + val2
	} else if operator == "-" {
		return val1 - val2
	} else if operator == "*" {
		return val1 * val2
	} else if operator == "/" {
		return val1 / val2
	} else {
		return 0
	}
}

type Stack struct {
	tokens []int
	top    int
}

func NewStack() Stack {
	return Stack{
		tokens: make([]int, 0),
		top:    -1,
	}
}

func (stack *Stack) Push(value int) {
	stack.top += 1
	stack.tokens = append(stack.tokens, value)
}

func (stack *Stack) Pop() int {
	value := stack.tokens[stack.top]
	stack.tokens = stack.tokens[:stack.top]
	stack.top -= 1
	return value
}

func (stack *Stack) IsEmpty() bool {
	return stack.top == -1
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	tokens := Deserialize[[]string](ReadLine(stdin))
	ans := evalRPN(tokens)

	fmt.Println("\noutput:", Serialize(ans))
}
