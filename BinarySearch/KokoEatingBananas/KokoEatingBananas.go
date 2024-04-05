package kokoeatingbananas

import (
	"fmt"
	"math"
	"slices"
)

func MinEatingSpeed(piles []int, h int) int {

	min := int(math.Ceil(float64(sliceSum(piles)) / float64(h))) 
	max := slices.Max(piles) // right

	best := max

	for k := min; k <= max; k++ {

		hours := 0

		for _, pile := range piles {
			hours += int(math.Ceil(float64(pile) / float64(k)))
		}

		if hours <= h {
			best = int(math.Min(float64(best), float64(k)))
		}

		fmt.Println(h, k, hours, best)
	}

	return 0
}

func sliceSum(slice []int) int {
	sum := 0
	for _, val := range slice {
		sum += val
	}
	return sum
}
