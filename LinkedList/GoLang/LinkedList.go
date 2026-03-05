package main

import "fmt"

type Node struct {
	data int
	next *Node
}

type LinkedList struct {
	head *Node
}

func (l *LinkedList) Add(data int) {
	node := Node{
		data: data,
		next: nil,
	}
	if l.head == nil {
		l.head = &node
		return
	}
	curr := l.head
	for curr.next != nil {
		curr = curr.next
	}
	curr.next = &node
}

func (l *LinkedList) AddAt(data int, index int) {
	node := Node{
		data: data,
		next: nil,
	}
	if l.head == nil {
		fmt.Println("No Item in linked list Adding at index 0")
		l.head = &node
		return
	}
	length := l.Len()
	if length < index {
		fmt.Println("Index is greater then the Linked list length Adding not at the end")
		l.Add(data)
	}

	loop_index := 0

	curr := l.head
	for curr.next != nil {
		if loop_index == index {
			node.next = curr.next
			curr.next = &node
			break
		}
		curr = curr.next
		loop_index += 1
	}

}

func (l *LinkedList) Len() int {
	if l.head == nil {
		return 0
	}
	var length int
	curr := l.head
	for curr.next != nil {
		length += 1
		curr = curr.next
	}
	length += 1
	return length
}

func (l *LinkedList) Print() {
	if l.head == nil {
		fmt.Println("No Item in linked list")
		return
	}
	curr := l.head
	for curr.next != nil {
		fmt.Printf("%d ", curr.data)
		curr = curr.next
	}
	fmt.Print(curr.data)

}

func main() {
	l := LinkedList{head: nil}
	l.Print()
	l.AddAt(5555, 10)
	l.Add(2)
	l.Add(1)
	l.Add(3)
	l.Add(5)
	l.AddAt(3431, 7)
	l.Add(3422)
	l.Add(3423)
	l.Add(5122)
	l.Print()

}
