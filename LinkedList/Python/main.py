class Node:
    def __init__(self,data):
        self.data = data
        self.next: Node| None = None

class LinkedList:
    def __init__(self) -> None:
        self.head =  None
        pass

    def add(self,data: int):
        node = Node(data)
        if not self.head:
           self.head = node
           return
        last_head = self.head
        while last_head.next:
            last_head = last_head.next

        last_head.next= node

    def add_head(self,data: int):
        node = Node(data)
        next = self.head
        node.next = next
        self.head = node

    def add_middle(self,data:int,order_form_start:int):
        node = Node(data)
        if self.head is None:
            return node

        if order_form_start == 0:
            old_head = self.head
            node.next = old_head
            self.head = node
            return

        start = 0
        next_node = self.head
        while next_node.next:
            start +=1
            if start == order_form_start -1 :
                break
            else:
                next_node = next_node.next

        node.next = next_node.next
        next_node.next = node

    def delete(self,location: int):
        curr_head =self.head
        if curr_head is None:
            return
        if location == 0:
            self.head = curr_head.next
            return
        start = 0
        while curr_head.next:
            start = +1
            if start == location - 1:
                curr_head.next = curr_head.next



    def search(self,number):
        curr_head = self.head
        while curr_head.next:
            if number == curr_head.data:
                print(f"{number} found")
                break
            curr_head = curr_head.next
        else:
            print(f"{number} not found")


    def len(self):
        if not self.head:
            return 0
        curr_head = self.head
        len_start = 0
        while curr_head.next:
            len_start +=1
            curr_head = curr_head.next
        return len_start

    def __len__(self):
        return self.len()

    def print(self):
        next_head = self.head
        while next_head:
            print(next_head.data)
            next_head = next_head.next
        print("\n")

l = LinkedList()
l.add(10)
l.add(20)
l.add_head(60)
l.add(30)
# l.print()
l.add(40)
l.add(50)
l.add_middle(200, -1)
l.add(50)
l.print()
l.search(20)
l.delete(20)
l.print()
print(l.len(), l.__len__(), len(l) )
