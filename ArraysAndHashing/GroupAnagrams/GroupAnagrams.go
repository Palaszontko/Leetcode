package GroupAnagrams

import (
	"sort"
	"strings"
)

func GroupAnagrams(strs []string) [][]string {

	hash_map := make(map[string][]string)

	for _, val := range strs {
		tmp := strings.Split(val, "")
		sort.Strings(tmp)
		key := strings.Join(tmp, "")

		hash_map[key] = append(hash_map[key], val)

	}

	result_array := make([][]string, 0)

	for _ , val := range hash_map {
		result_array = append(result_array, val)
	}

	return result_array
}
