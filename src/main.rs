use rust_merkle_tree::*;

fn main() {}

#[cfg(test)]
mod test_array_impl {
    use crate::array_implementation;
    #[test]
    fn test_capacity_allocation_1_elements() {
        let arr_4_elements = ["1"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_4_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 1);
    }
    #[test]
    fn test_capacity_allocation_2_elements() {
        let arr_4_elements = ["1", "2"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_4_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 3);
    }
    #[test]
    fn test_capacity_allocation_3_elements() {
        let arr_4_elements = ["1", "2", "3"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_4_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 6);
    }
    #[test]
    fn test_capacity_allocation_4_elements() {
        let arr_4_elements = ["1", "2", "3", "4"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_4_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 7);
    }

    #[test]
    fn test_capacity_allocation_5_elements() {
        let arr_5_elements = ["5", "6", "7", "8", "9"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_5_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 11);
    }

    #[test]
    fn test_capacity_allocation_6_elements() {
        let arr_6_elements = ["10", "11", "12", "13", "14", "15"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_6_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 12);
    }

    #[test]
    fn test_capacity_allocation_7_elements() {
        let arr_7_elements = ["16", "17", "18", "19", "20", "21", "22"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_7_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 14);
    }

    #[test]
    fn test_capacity_allocation_8_elements() {
        let arr_8_elements = ["23", "24", "25", "26", "27", "28", "29", "30"];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_8_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(new_merkle_tree.get_size(), 15);
    }

    #[test]
    fn test_add_one_element_to_merkle_tree() {
        let arr_6_elements = ["10", "11", "12", "13", "14", "15"];
        let mut obj = array_implementation::ArrayMT::new(
            arr_6_elements.iter().map(|el| el.to_string()).collect(),
        );
        // eprintln!("{:?}", ob);
        assert_eq!(obj.get_size(), 12);
        obj.add_one_new_element("16".to_string());
        assert_eq!(obj.get_size(), 14);
    }

    #[test]
    fn test_add_array_of_elements_to_merkle_tree() {
        let arr_6_elements = ["1", "2", "3", "4"];
        let mut obj = array_implementation::ArrayMT::new(
            arr_6_elements.iter().map(|el| el.to_string()).collect(),
        );
        assert_eq!(obj.get_size(), 7);
        let new_elements = ["5", "6", "7"];
        obj.add_array_of_new_elements(new_elements.iter().map(|el| el.to_string()).collect());
        assert_eq!(obj.get_size(), 14);
        let new_elements = ["8"];
        obj.add_array_of_new_elements(new_elements.iter().map(|el| el.to_string()).collect());
        assert_eq!(obj.get_size(), 15);
    }

    #[test]
    fn test_generate_merkle_proof_and_verify() {
        let arr_20_elements = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20",
        ];
        let new_merkle_tree = array_implementation::ArrayMT::new(
            arr_20_elements.iter().map(|el| el.to_string()).collect(),
        );

        let response = new_merkle_tree.prove_if_exists("3".to_string());
        assert!(response);
    }
}
