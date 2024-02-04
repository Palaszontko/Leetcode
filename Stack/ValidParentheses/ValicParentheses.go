package ValidParentheses

func IsValid(s string) bool {

	stack := []rune{}

	hash_map_open_to_close := map[rune]rune{
		'(': ')',
		'{': '}',
		'[': ']',
	}

	for _, val := range s {
		if val == '(' || val == '{' || val == '[' {
			stack = append(stack, val)
		} else {
			if len(stack) == 0 {
                return false
            }
			lastest_parenthesis := stack[len(stack) - 1]
			stack = stack[:len(stack) - 1]

			if hash_map_open_to_close[lastest_parenthesis] != val {
				return false
			}
		}

	}

	return len(stack) == 0
}
