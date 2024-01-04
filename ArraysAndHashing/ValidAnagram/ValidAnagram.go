package ValidAnagram

func IsAnagram(s string, t string) bool {
	Hash_s := make(map[byte]int)
	Hash_t := make(map[byte]int)

	if len(s) != len(t) {
		return false
	}

	for i := range s {
		Hash_s[s[i]] += 1
		Hash_t[t[i]] += 1
	}

	for key := range Hash_s {
		if Hash_s[key] != Hash_t[key] {
			return false
		}
	}

	return true
}
