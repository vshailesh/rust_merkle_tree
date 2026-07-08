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
        let mut N = arr_size;

        //helps in calculating the real index of the elements to hash
        //while traversing over the array.
        let mut offset = 0;
        while N > 1 {
            for j in (0..N).step_by(2) {
                // calculates the real index in the array.
                let internal_index = offset + j;
                let last_index = offset + N - 1;
                let left = merkle_tree[internal_index].clone();
                let right = merkle_tree[std::cmp::min(internal_index + 1, last_index)].clone();
                let concatenated_value = left + right.as_str();
                merkle_tree[index] = Self::encrypt_sha256(concatenated_value);
                index += 1;
            }
            offset += N;
            N = N.div_ceil(2);
        }
        Self { merkle_tree }
    }

    pub fn get_capacity(&self) -> usize {
        self.merkle_tree.capacity()
    }

    pub fn get_size(&self) -> usize {
        self.merkle_tree.len()
    }

    pub fn add_one_new_element(&mut self, element: String) {
        let len = self.merkle_tree.len();
        let original_elements = Self::find_original_number_of_elements(len);
        let mut temp_merkle_tree: Vec<String> = Vec::with_capacity(original_elements as usize + 1);
        let old_slice = &self.merkle_tree[0..original_elements as usize];
        temp_merkle_tree.extend_from_slice(old_slice);
        temp_merkle_tree.push(Self::encrypt_sha256(element));
        Self::build_merkle_tree(&mut temp_merkle_tree);
        self.merkle_tree = temp_merkle_tree;
    }

    pub fn add_array_of_new_elements(&mut self, array: Vec<String>) {
        let len = self.merkle_tree.len();
        let original_elements = Self::find_original_number_of_elements(len);
        let mut temp_merkle_tree: Vec<String> = Vec::with_capacity(original_elements as usize + 1);
        let old_slice = &self.merkle_tree[0..original_elements as usize];
        temp_merkle_tree.extend_from_slice(old_slice);
        for el in array {
            temp_merkle_tree.push(Self::encrypt_sha256(el));
        }
        Self::build_merkle_tree(&mut temp_merkle_tree);
        self.merkle_tree = temp_merkle_tree;
    }

    fn encrypt_sha256(val: String) -> String {
        let final_encrypted_val = Sha256::digest(val.into_bytes());
        hex::encode(final_encrypted_val)
    }

    fn calculate_array_capacity(arr_size: i32) -> i32 {
        let mut temp_arr_size = arr_size;
        let mut capacity = 0;
        while temp_arr_size > 1 {
            capacity += temp_arr_size;
            temp_arr_size = (temp_arr_size + 1) / 2;
        }
        capacity + temp_arr_size
    }

    fn find_original_number_of_elements(num: usize) -> i32 {
        let mut lo: i32 = 0;
        let mut hi: i32 = 1_000_000_007;

        while lo < hi {
            let mid = hi - (hi - lo) / 2;
            let temp_capacity = Self::calculate_array_capacity(mid);
            if temp_capacity as usize > num {
                hi = mid - 1;
            } else {
                lo = mid;
            }
        }
        lo
    }

    fn build_merkle_tree(mtree: &mut Vec<String>) {
        let mut N = mtree.len();

        //helps in calculating the real index of the elements to hash
        //while traversing over the array.
        let mut offset = 0;
        while N > 1 {
            for j in (0..N).step_by(2) {
                // calculates the real index in the array.
                let internal_index = offset + j;
                let concatenated_value = mtree[internal_index].clone()
                    + mtree[std::cmp::min(internal_index + 1, N - 1)]
                        .clone()
                        .as_str();
                mtree.push(Self::encrypt_sha256(concatenated_value));
            }
            offset += N;
            N = N.div_ceil(2);
        }
    }

    fn generate_merkle_proof(&self, index: usize) -> Vec<String> {
        let mut index = index;
        let mut N = Self::find_original_number_of_elements(self.merkle_tree.len());
        let mut offset = 0;
        let mut proof: Vec<String> = vec![];

        while N > 1 {
            if index % 2 == 0 {
                let k = std::cmp::min(offset + index + 1, offset + N as usize - 1);
                proof.push(self.merkle_tree[k].clone());
            } else {
                let k = offset + index - 1;
                proof.push(self.merkle_tree[k].clone());
            }
            index /= 2;
            offset += N as usize;
            N = (N + 1) / 2;
        }
        proof
    }

    fn verify_merkle_proof(&self, index: usize, proof: Vec<String>) -> bool {
        let mut index = index;
        let merkle_root = self.merkle_tree[self.merkle_tree.len() - 1].clone();
        let mut hash = self.merkle_tree[index].clone();
        for p in proof.iter() {
            let left: String;
            let right: String;
            if index % 2 == 0 {
                left = hash;
                right = p.clone();
            } else {
                left = p.clone();
                right = hash;
            }
            hash = Self::encrypt_sha256(left + right.as_str());
            index /= 2;
        }
        merkle_root == hash
    }

    pub fn prove_if_exists(&self, data: String) -> bool {
        let encrypted_data = Self::encrypt_sha256(data);
        let num_of_elements = Self::find_original_number_of_elements(self.merkle_tree.len());
        let mut index: usize = 0;
        for idx in 0..num_of_elements as usize {
            if self.merkle_tree[idx] == encrypted_data {
                index = idx;
                break;
            }
        }
        let merkle_proof = self.generate_merkle_proof(index);
        self.verify_merkle_proof(index, merkle_proof)
    }
}
