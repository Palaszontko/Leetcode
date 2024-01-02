package main

import "fmt"

func isAnagram(s string, t string) bool {
    Hash_s := make(map[byte]int)
    Hash_t := make(map[byte]int)

    if len(s) != len(t){
        return false
    }

    for i := range s{
        Hash_s[s[i]] += 1
        Hash_t[t[i]] += 1
	}

	for key, _ := range Hash_s {
		if Hash_s[key] != Hash_t[key]{
			return false
		}
	}

	return true
}

func main(){
	word1 := "abcd"
	word2 := "dacb"

	fmt.Println(isAnagram(word1, word2))
}
