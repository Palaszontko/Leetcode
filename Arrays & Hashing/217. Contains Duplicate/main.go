package main

import "fmt"

func containsDuplicate(nums []int) bool {
    Hash := make(map[int]bool)

    for _, i := range nums{
        if Hash[i]{
            return true
		}else {
            Hash[i] = true;
        }
	}
	return false
}


func main(){
	a := []int{1,2,3,4,5,6}
	fmt.Println(containsDuplicate(a))
}