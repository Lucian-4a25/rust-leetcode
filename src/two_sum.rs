// 两数之和 II - 输入有序数组
struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 2 {
            return vec![];
        }

        let mut s = 0;
        let mut e = numbers.len() - 1;
        while s < e {
            let sum = numbers[s] + numbers[e] - target;
            if sum > 0 {
                e -= 1;
            } else if sum < 0 {
                s += 1;
            } else {
                return vec![(s + 1) as i32, (e + 1) as i32];
            }
        }

        return vec![];
    }
}
