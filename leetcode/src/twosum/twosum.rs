impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            for (j, m) in nums[i + 1..].iter().enumerate() {
                if n + m == target {
                    indices.push(i as i32);
                    indices.push((i + 1 + j) as i32);
                    return indices;
                }
            }
        }
        indices
    }
}
