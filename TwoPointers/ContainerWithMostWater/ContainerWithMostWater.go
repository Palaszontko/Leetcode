package containerwithmostwater

func MaxArea(height []int) int {

	l := 0
	r := len(height) - 1
	best := 0

	for l < r {

		area := min(height[l], height[r]) * (r - l)

		if height[l] < height[r] {
			l += 1
		} else {
			r -= 1
		}

		if area > best {
			best = area
		}
	}

	return best
}
