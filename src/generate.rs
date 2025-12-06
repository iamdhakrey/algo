use rand::seq::SliceRandom;

pub fn search_list() -> Vec<i32> {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    return nums;
}
