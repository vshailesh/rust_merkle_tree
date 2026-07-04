use sha2::{Digest, Sha256};

pub struct ArrayMT {
    merkle_tree: Vec<String>,
}

impl ArrayMT {
    // Construct Merkle Tree from String array
    pub fn new(arr: Vec<String>) -> Self {
        let arr_size = arr.len();
        let capacity = Self::calculate_array_capacity(arr_size as i32);
        let mut merkle_tree = Vec::with_capacity(capacity as usize);
        let mut index: usize = 0;
        merkle_tree = vec!["".to_string(); capacity as usize];
        for el in arr.iter() {
            let val = el.clone();
            let encrypted_val = Self::encrypt_sha256(val);
            merkle_tree[index] = encrypted_val;
            index += 1;
        }
        // eprintln!("{:?}", merkle_tree);
        let mut N = arr_size;

        //helps in calculating the real index of the elements to hash
        //while traversing over the array.
        let mut offset = 0;
        while N > 1 {
            for j in (0..N).step_by(2) {
                // calculates the real index in the array.
                let internal_index = offset + j;
                let concatenated_value = merkle_tree[internal_index].clone()
                    + merkle_tree[std::cmp::min(internal_index + 1, N - 1)]
                        .clone()
                        .as_str();
                merkle_tree[index] = Self::encrypt_sha256(concatenated_value);
                index += 1;
            }
            offset += N;
            N = N.div_ceil(2);
        }
        Self { merkle_tree }
    }

    fn encrypt_sha256(val: String) -> String {
        let final_encrypted_val = Sha256::digest(val.into_bytes());
        hex::encode(final_encrypted_val)
    }

    // fn decrypt_sha256(val: String) -> Vec<u8> {
    //
    // }

    fn calculate_array_capacity(arr_size: i32) -> i32 {
        let mut temp_arr_size = arr_size;
        let mut capacity = 0;
        while temp_arr_size > 1 {
            capacity += temp_arr_size;
            temp_arr_size = (temp_arr_size + 1) / 2;
        }
        capacity + temp_arr_size
    }

    pub fn get_capacity(&self) -> usize {
        self.merkle_tree.capacity()
    }
}
