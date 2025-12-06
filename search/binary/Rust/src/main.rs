fn generate_sorted_list(numbers_count: i64) -> Vec<i64> {
    let nums: Vec<i64> = (0..numbers_count).collect();
    return nums;
}

trait SearchTrait {
    fn search(&self) {}
}

struct BinarySearch {
    pub numbers_list: Vec<i64>,
    pub search_number: i64,
}

impl SearchTrait for BinarySearch {
    fn search(&self) {
        if self.numbers_list.len() <= 0 {
            println!("List have no numbers or empty list");
        } else {
            let mut start: i64 = 1;
            let mut end: i64 = self.numbers_list.len() as i64;
            let mut found: bool = false;
            while !found {
                if start > end {
                    let c = start;
                    start = end;
                    end = c;
                }
                let middle = start + (end - start) / 2;
                if middle >= self.numbers_list.len() as i64 || middle < 0 {
                    println!("number not found {}", middle);
                    break;
                }

                if self.numbers_list[middle as usize] > self.search_number {
                    end = middle - 1
                }
                if self.numbers_list[middle as usize] < self.search_number {
                    start = middle + 1
                }
                if self.numbers_list[middle as usize] == self.search_number {
                    found = true;
                    println!("number found {}", self.search_number);
                }
            }
        }
    }
}

fn main() {
    let num_list = generate_sorted_list(100);
    let b = BinarySearch {
        numbers_list: num_list,
        search_number: -1,
    };
    b.search();
}
