/// To hold each node of the Tree
pub struct MerkleTree {
    node: Box<Option<Node>>,
}

/// Node struct
/// where each node may contain its own MerkleTree
pub struct Node {
    parent: Box<Option<Node>>,
    left: Box<Option<MerkleTree>>,
    right: Box<Option<MerkleTree>>,
    data: String,
}

/// check if a number is a power of 2 or not
pub fn power_of_2(num: usize) -> bool {
    let mut count: usize = 0;
    let mut mut_num = num;
    while mut_num > 0 {
        count += mut_num & 1;
        // eprintln!("mut_num = {}", mut_num);
        mut_num >>= 1;
        // eprintln!("mut_num = {}", mut_num);
    }
    count == 1
}

pub fn power2_just_smaller_than_num(num: usize) -> i32 {
    let mut count_power = 1;
    let mut val = 1 << count_power;

    while val < num {
        count_power += 1;
        val = 1 << count_power;
    }
    if val == num {
        count_power
    } else {
        count_power - 1
    }
}
pub fn find_power(num: usize) -> usize {
    let mut power = 0;
    let mut mut_num = num;
    while mut_num > 0 {
        power += 1;
        mut_num >>= 1;
    }
    power
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_2() {
        let is_power_of_2 = 512;
        assert!(power_of_2(is_power_of_2));
    }
    #[test]
    fn test_is_not_power_of_2() {
        let is_not_power_of_2 = 5;
        assert!(!power_of_2(is_not_power_of_2));
    }
}
