fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut counter: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for value in nums {
        *counter.entry(value).or_insert(0) += 1;
    }

    let mut uniq_nums: Vec<i32> = vec![];
    for key in counter.keys() {
        uniq_nums.push(*key);
    }

    uniq_nums.sort();

    for i in 0..uniq_nums.len() {
        if uniq_nums[i] * 3 == 0 && counter[&uniq_nums[i]] >= 3 {
            res.push(vec![uniq_nums[i], uniq_nums[i], uniq_nums[i]]);
        }
        for j in i + 1..uniq_nums.len() {
            if uniq_nums[i] * 2 + uniq_nums[j] == 0 && counter[&uniq_nums[i]] > 1 {
                res.push(vec![uniq_nums[i], uniq_nums[i], uniq_nums[j]]);
            }
            if uniq_nums[j] * 2 + uniq_nums[i] == 0 && counter[&uniq_nums[j]] > 1 {
                res.push(vec![uniq_nums[i], uniq_nums[j], uniq_nums[j]]);
            }
            let c = 0 - uniq_nums[i] - uniq_nums[j];
            if c > uniq_nums[j] && counter.contains_key(&c) {
                res.push(vec![uniq_nums[i], uniq_nums[j], c]);
            }
        }
    }

    res
}
