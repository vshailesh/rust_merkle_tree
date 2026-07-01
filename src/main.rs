// pub mod array_implementation;
// use array_implementation::*;

use rust_merkle_tree::*;

fn main() {}

#[cfg(test)]
mod test_array_impl {
    use crate::array_implementation;

    #[test]
    fn test_capacity_allocation_4_elements() {
        let arr_4_elements = ["1", "2", "3", "4"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_4_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_capacity(), 7);
    }

    #[test]
    fn test_capacity_allocation_5_elements() {
        let arr_5_elements = ["1", "2", "3", "4", "5"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_5_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_capacity(), 12);
    }

    #[test]
    fn test_capacity_allocation_6_elements() {
        let arr_6_elements = ["1", "2", "3", "4", "5", "6"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_6_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_capacity(), 13);
    }

    #[test]
    fn test_capacity_allocation_7_elements() {
        let arr_7_elements = ["1", "2", "3", "4", "5", "6", "7"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_7_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_capacity(), 14);
    }

    #[test]
    fn test_capacity_allocation_8_elements() {
        let arr_8_elements = ["1", "2", "3", "4", "5", "6", "7", "8"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_8_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_capacity(), 15);
    }
}
