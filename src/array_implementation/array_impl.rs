use crate::common::{find_power, power_of_2, power2_just_smaller_than_num};
pub struct ArrayMT {
    merkle_tree: Vec<String>,
}

impl ArrayMT {
    // Construct Merkle Tree from String array
    pub fn new(arr: Vec<String>) -> Self {
        let arr_size = arr.len();
        let capacity = Self::calculate_array_capacity(arr_size as i32);
        let merkle_tree = Vec::with_capacity(capacity as usize);
        Self { merkle_tree }
    }

    fn calculate_array_capacity(arr_size: i32) -> i32 {
        let mut capacity = 0;
        if arr_size % 2 == 0 {
            if power_of_2(arr_size as usize) {
                let power = find_power(arr_size as usize);
                capacity = (1 << power) - 1;
            } else {
                let one_smaller_power = power2_just_smaller_than_num(arr_size as usize);
                eprintln!("one_smaller_power = {}", one_smaller_power);
                capacity = (1 << (one_smaller_power + 1)) - 1 + arr_size;
            }
        } else {
            if power_of_2(arr_size as usize + 1) {
                let power = find_power(arr_size as usize + 1);
                capacity = (1 << power) - 2;
            } else {
                let one_smaller_power = power2_just_smaller_than_num(arr_size as usize);
                capacity = (1 << (1 + one_smaller_power)) - 1 + arr_size;
            }
        }
        capacity
    }

    pub fn get_capacity(&self) -> usize {
        self.merkle_tree.capacity()
    }
}
