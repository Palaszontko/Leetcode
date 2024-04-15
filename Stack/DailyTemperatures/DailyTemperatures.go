package dailytemperatures

import "fmt"

func DailyTemperatures(temperatures []int) []int {

	result := []int{}
	l := 0

	for l < len(temperatures) - 1 {
		fmt.Println(l)
		counter := 1
		r := l + 1
		for temperatures[l] >= temperatures[r] {
			counter += 1
			r += 1
			if r > len(temperatures) - 1 {
				if counter > 0 {
					counter = 0
				}
				break
			}
		}
		result = append(result, counter)
		l += 1
	}

	result = append(result, 0)

	return result


}
