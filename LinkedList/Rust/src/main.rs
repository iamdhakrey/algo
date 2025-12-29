#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Add a new node with the given data to the front of the list
    pub fn add(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node)
    }

    pub fn remove(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    /// remove at position
    pub fn remove_at(&mut self, location: i32) -> Option<i32> {
        if location < 0 {
            return None;
        }
        if location == 0 {
            return self.remove();
        }
        let mut current = &mut self.head;
        for _ in 0..location - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return None;
            }
        }
        if let Some(node) = current {
            if let Some(removed_node) = node.next.take() {
                node.next = removed_node.next;
                return Some(removed_node.data);
            }
        }
        None
    }

    pub fn len(&mut self) -> i32 {
        let mut length = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }
        length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn print(&mut self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.data);
            current = &node.next;
        }
    }

    pub fn search(&mut self, target: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == target {
                return true;
            }
            current = &node.next;
        }
        false
    }
}

fn main() {
    let mut l = LinkedList::new();
    l.add(4);
    l.add(2);
    l.add(3);
    l.add(1);
    let length = l.len();
    println!("Length of Linked  List: {}", length);
    let search = l.search(2);
    println!("Search 2 in Linked List: {}", search);
    let is_empty = l.is_empty();
    println!("Is Linked List Empty: {}", is_empty);
    l.remove_at(0);
    l.print();
}
