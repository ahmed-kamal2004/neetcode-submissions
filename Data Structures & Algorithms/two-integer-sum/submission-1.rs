use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mm = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            if let Some(jdx) = mm.get(&(target - num)) {
                return vec![*jdx, idx as i32];
            }
            mm.insert(num, idx as i32);
        }

        return vec![]
    }
}
