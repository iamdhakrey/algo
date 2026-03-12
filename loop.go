package main

import "fmt"

func factorial(n int) int {
	if n == 1 {
		return 1
	}
	return n * factorial(n-1)
}

func main() {
	// fmt.Println("Test")
	var test_list []string
	test_list = []string{
		"heelp", "gesg",
	}
	fmt.Println(test_list)
	for li, fsd := range test_list {
		fmt.Println(li, fsd)
	}

	var test2 map[string]string
	test2 = map[string]string{
		"eter": "tret",
		"tret": "trete",
	}
	fmt.Println(test2)
	for li, fsd := range test2 {
		fmt.Println(li, fsd)
	}

	z := 20
	for i := 0; i < z; i += 10 {
		fmt.Println(i)
	}

	for i := 5; i > 0; i-- {

		for j := 1; j >= i; j-- {
			if i == j {
				fmt.Println("*")

			} else {
				fmt.Print("*")
			}
		}
	}

	fec := factorial(5)
	fmt.Println("gfe")
	fmt.Println(fec)
}
