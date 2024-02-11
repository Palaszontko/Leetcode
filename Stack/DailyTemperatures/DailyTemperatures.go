package dailytemperatures

import "fmt"

func DailyTemperatures(temperatures []int) []int {

	results := []int{}

	l := 0
	r := 1

	for r < len(temperatures) {
		if temperatures[l] < temperatures[r] {
			results = append(results, 1)
			l += 1
			r += 1
		} else {
			
			for r < len(temperatures) && temperatures[l] >= temperatures[r] {
				fmt.Println(temperatures[l], temperatures[r])
				r += 1
			}

			if r > len(temperatures)-1 {
				results = append(results, 0)
			} else {
				results = append(results, r-l)
			}

			l += 1
			r = l + 1
		}
	}

	results = append(results, 0)

	return results
}
