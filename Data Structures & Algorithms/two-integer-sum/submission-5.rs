use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_d: Vec<(i32, i32)> = nums.into_iter()
        .enumerate()
        .map(|(index, item) | (item, index as i32))
        .collect(); // uggh, I don't like this fukin clone

        nums_d.sort_unstable_by(|a, b| a.0.cmp(&b.0)); // it is now fukin sorted

        let (mut l, mut r) = (0, nums_d.len() - 1);

        while l < r {
            if nums_d[l].0 + nums_d[r].0 == target {
                let mut v =vec![nums_d[l].1 as i32, nums_d[r].1 as i32];
                v.sort_unstable();
                return v;
            }

            if nums_d[l].0 + nums_d[r].0 < target {
                l += 1;
            } else {
                r -= 1;
            }
        }

        return vec![]
    }
}
