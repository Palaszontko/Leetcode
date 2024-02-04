package ValidParentheses

import "fmt"

func IsValid(s string) bool {

	stack := NewStack()

	hash_map_open_to_close := map[rune]rune{
		'(': ')',
		'{': '}',
		'[': ']',
	}

	for _, val := range s {
		if val == '(' || val == '{' || val == '[' {
			stack.Push(val)
		} else {
			lastest_parenthesis := stack.Pop()

			if hash_map_open_to_close[lastest_parenthesis] != val {
				return false
			}
		}

	}

	return stack.IsEmpty()
}

type Stack struct {
	values []rune
	top    int
}

func NewStack() *Stack {
	return &Stack{
		values: make([]rune, 0),
		top:    -1,
	}
}

func (stack *Stack) Push(value rune) {
	stack.top += 1
	stack.values = append(stack.values, value)
}

func (stack *Stack) Pop() rune {
	if stack.IsEmpty() {
		fmt.Println("Cannot pop from empty stack")
	}

	value := stack.values[stack.top]
	stack.values = stack.values[:stack.top]
	stack.top -= 1

	return value
}

func (stack *Stack) IsEmpty() bool {
	return stack.top == -1
}
