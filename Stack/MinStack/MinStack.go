package MinStack

import "fmt"

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
