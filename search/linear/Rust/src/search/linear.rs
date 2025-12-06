use crate::search::SeachTrait;

#[derive(Debug)]
pub struct LinearSearch {
    pub list_of_numbers: Vec<i32>,
    pub seach_number: i32,
}

impl SeachTrait for LinearSearch {
    fn search(&self) {
        let data = self.list_of_numbers.clone();
        let mut found = false;
        for i in data {
            if self.seach_number == i {
                found = true;
            }
        }
        match found {
            true => println!("{} Number Found in the list", &self.seach_number),
            _ => println!("{} Number not Found in the list", &self.seach_number),
        }
    }
}
