package main

import "fmt"

func merge(left []int, right []int) []int {
	var result []int
	i := 0
	j := 0
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			result = append(result, left[i])
			i += 1
		} else {
			result = append(result, right[j])
			j += 1
		}
	}
	result = append(result, left[i:]...)
	result = append(result, right[j:]...)
	return result
}

func merge_sort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}
	mid := len(arr) / 2
	fmt.Println(mid)
	left := arr[mid:]
	right := arr[:mid]
	left_sorted := merge_sort(left)
	right_sorted := merge_sort(right)
	return merge(left_sorted, right_sorted)
}

func main() {
	unsortedList := []int{3, 54, 66, 324, 6, 34, 46, 63, 3243, 7, 34, 234, 5645}
	sorted_arr := merge_sort(unsortedList)
	fmt.Println(sorted_arr)

}
