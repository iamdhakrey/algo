package main

import (
	"fmt"
	"math/rand"
)

type SearchInterface interface {
	Print()
	Search()
}

type LinearSeach struct {
	numbers_list  []int64
	search_nubmer int64
	found         *bool
}

func GenerateNumbersList() []int64 {
	var numbers_list []int64

	for range 100 {
		randomNumber := rand.Int63n(100)
		numbers_list = append(numbers_list, randomNumber)
	}
	return numbers_list
}

func (l *LinearSeach) Search() {
	for _, v := range l.numbers_list {
		if l.search_nubmer == v {
			found := true
			l.found = &found
			break
		}
	}
	l.Print()
}

func (l *LinearSeach) Print() {
	if l.found != nil {
		if *l.found {
			fmt.Printf("Number %d found in list\n", l.search_nubmer)
		} else {
			fmt.Printf("Number %d not exists in the list\n", l.search_nubmer)
		}
	} else {
		fmt.Printf("Number %d not exists in the list\n", l.search_nubmer)
	}
}

func main() {
	fmt.Println("Go Lang Linear Search")

	numbersList := GenerateNumbersList()
	fmt.Println(numbersList)
	l := LinearSeach{
		numbers_list:  numbersList,
		search_nubmer: 101,
	}
	l.Search()
}
