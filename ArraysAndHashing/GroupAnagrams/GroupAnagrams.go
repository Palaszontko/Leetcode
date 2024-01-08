package GroupAnagrams

import (
	"reflect"
)

func generate_anagram_hash(word string) map[int]int {
	hash_map := make(map[int]int)

	for _, letter := range word {
		hash_map[int(letter)] += 1
	}

	return hash_map
}

func GroupAnagrams(strs []string) [][]string {
	anagrams := make(map[string]map[int]int)

	for _, word := range strs {
		anagrams[word] = generate_anagram_hash(word)
	}

	list := [][]string{}

	list = append(list, []string{strs[0]})

	for key, hash_map := range anagrams {

		for i := 0; i < len(list); i++ {
			if reflect.DeepEqual(anagrams[list[i][0]], hash_map) && list[i][0] != key {
				list[i] = append(list[i], key)
				break
			} else {
				list = append(list, []string{key})
				break
			}
		}
	}

	return list
}
