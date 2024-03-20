use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = vec![];
        nums.sort();
        let num_len = nums.len();
        let mut prev = None;
        for i in 0..num_len - 2 {
            let v = nums[i];
            // skip checked num
            if prev.map(|p| p == v).unwrap_or(false) {
                continue;
            }
            let mut left = i + 1;
            let mut right = num_len - 1;
            // let mut left_vals = HashSet::new();
            while left < right {
                let sum = nums[left] + nums[right] + v;
                if sum > 0 {
                    right -= 1;
                    continue;
                }
                if sum < 0 {
                    left += 1;
                    continue;
                }

                // need to check duplication
                // let mut exist = false;
                // for res in result.iter() {
                //     if res[0] == v && res[1] == nums[left] {
                //         exist = true;
                //         break;
                //     }
                // }
                // if !exist {
                //     result.push(vec![v, nums[left], nums[right]]);
                // }
                result.push(vec![v, nums[left], nums[right]]);
                let old_l = nums[left];
                let old_r = nums[right];
                loop {
                    left += 1;
                    if nums[left] != old_l || left >= right {
                        break;
                    }
                }
                loop {
                    right -= 1;
                    if nums[right] != old_r || left >= right {
                        break;
                    }
                }
            }

            prev = Some(v);
        }

        result
    }
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (pos, &num) in nums.iter().enumerate() {
            map.entry(num).or_default().push(pos);
        }

        let mut result: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                let two_sum = nums[i] + nums[j];
                if let Some(v) = map.get(&(-two_sum)) {
                    for &index in v.iter() {
                        if index != i && index != j {
                            let mut exist = false;
                            // duplicate the result
                            for r in result.iter() {
                                if r.contains(&nums[i]) && r.contains(&nums[j]) {
                                    if nums[i] != nums[j] {
                                        exist = true;
                                        break;
                                    }

                                    if r[0] == nums[i] && r[1] == nums[i]
                                        || r[1] == nums[i] && r[2] == nums[i]
                                        || r[0] == nums[i] && r[2] == nums[i]
                                    {
                                        exist = true;
                                        break;
                                    }
                                }
                            }
                            if !exist {
                                result.push(vec![nums[i], nums[j], nums[index]]);
                            }
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

#[test]
fn test_three_sum() {
    let i: i32 = 3;

    println!("value: {}", -i);
    println!("equal: {}", 0 == -0);
}
