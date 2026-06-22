// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/min-stack/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

type MinStack struct {
	values    []int
	top       int
	minValues []int
}

func Constructor() MinStack {
	return MinStack{
		values:    make([]int, 0),
		top:       -1,
		minValues: make([]int, 0),
	}
}

//lint:file-ignore ST1006 leetcode use "this"
func (this *MinStack) Push(val int) {
	if this.top == -1 {
		this.values = append(this.values, val)
		this.minValues = append(this.minValues, val)
		this.top += 1
	} else {
		this.values = append(this.values, val)
		this.top += 1

		if val < this.GetMin() {
			this.minValues = append(this.minValues, val)
		} else {
			this.minValues = append(this.minValues, this.minValues[len(this.minValues)-1])
		}
	}
}

func (this *MinStack) Pop() {
	this.values = this.values[:len(this.values)-1]
	this.minValues = this.minValues[:len(this.minValues)-1]
	this.top -= 1
}

func (this *MinStack) Top() int {
	return this.values[len(this.values)-1]
}

func (this *MinStack) GetMin() int {
	return this.minValues[len(this.minValues)-1]
}

func (this *MinStack) Display() {
	fmt.Println(this.values)
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	ops := Deserialize[[]string](ReadLine(stdin))
	params := MustSplitArray(ReadLine(stdin))
	output := make([]string, 0, len(ops))
	output = append(output, "null")

	obj := Constructor()

	for i := 1; i < len(ops); i++ {
		switch ops[i] {
		case "push":
			methodParams := MustSplitArray(params[i])
			val := Deserialize[int](methodParams[0])
			obj.Push(val)
			output = append(output, "null")
		case "pop":
			obj.Pop()
			output = append(output, "null")
		case "top":
			ans := Serialize(obj.Top())
			output = append(output, ans)
		case "getMin":
			ans := Serialize(obj.GetMin())
			output = append(output, ans)
		}
	}
	fmt.Println("\noutput:", JoinArray(output))
}
