package main

import "fmt"

func main() {
	arr := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1}
	lenght := len(arr)
	for i := range lenght {
		for j := range lenght - i - 1 {
			if arr[j] > arr[j+1] {
				arr[j], arr[j+1] = arr[j+1], arr[j]
			}
		}
	}
	fmt.Println("Sorted Array: ", arr)
}
