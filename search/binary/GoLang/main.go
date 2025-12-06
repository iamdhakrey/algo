package main

import "fmt"

type BinarySearch struct {
	numbers_list  []int64
	search_number int64
}

func generateSortArray(numbers_count int64) []int64 {
	arr := make([]int64, numbers_count)
	for i := range numbers_count {
		arr[i] = i
	}
	return arr
}

func (b *BinarySearch) Search() {
	list_len := len(b.numbers_list)
	start := 1
	end := list_len
	found := false

	for !found {
		if start > end {
			start, end = end, start
		}
		mid_pos := start + (end-start)/2
		if mid_pos == list_len || mid_pos < 0 {
			fmt.Println("Not Found", b.search_number)
			break
		}
		println(mid_pos)
		if b.numbers_list[mid_pos] < b.search_number {
			start = mid_pos + 1
		}
		if b.numbers_list[mid_pos] > b.search_number {
			end = mid_pos - 1
		}

		if b.numbers_list[mid_pos] == b.search_number {
			found = true
			fmt.Printf("Found %d ", b.search_number)
		}
	}

}

func main() {
	num_list := generateSortArray(100)
	b := BinarySearch{
		numbers_list:  num_list,
		search_number: -1,
	}
	b.Search()
}
