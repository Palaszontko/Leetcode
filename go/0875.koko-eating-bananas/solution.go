// Created by Olgierd Palasz at 2026/05/06 21:34
// leetgo: dev
// https://leetcode.com/problems/koko-eating-bananas/

package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func minEatingSpeed(piles []int, h int) int {
	min := int(math.Ceil(float64(sliceSum(piles)) / float64(h))) // left (smallest)
	max := slices.Max(piles)                                     // right

	best := max

	for min <= max {
		k := int((min + max) / 2)
		hours := 0

		for _, pile := range piles {
			hours += int(math.Ceil(float64(pile) / float64(k)))
		}

		if hours <= h {
			best = int(math.Min(float64(best), float64(k)))
			max = k - 1
		} else {
			min = k + 1
		}
	}

	return best
}

func sliceSum(slice []int) int {
	sum := 0
	for _, val := range slice {
		sum += val
	}
	return sum
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	piles := Deserialize[[]int](ReadLine(stdin))
	h := Deserialize[int](ReadLine(stdin))
	ans := minEatingSpeed(piles, h)

	fmt.Println("\noutput:", Serialize(ans))
}
