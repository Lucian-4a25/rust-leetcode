use core::slice::{self};
use std::ops::Range;

struct Solution;

impl Solution {
    // 移除数组中重复出现的数字，返回移除后的数组长度
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let raw_len = nums.len();
        if raw_len < 2 {
            return raw_len as i32;
        }

        let mut cur = nums[0];
        nums.push(cur);
        for i in 1..raw_len {
            if nums[i] as i32 == cur {
                continue;
            }
            cur = nums[i] as i32;
            nums.push(nums[i] as i32);
        }

        nums.drain(0..raw_len);

        nums.len() as i32
    }

    // 移除数组中重复出现次数超过两次的数字，返回数组的长度；注意不能申请额外的空间
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let raw_len = nums.len();
        if raw_len < 3 {
            return raw_len as i32;
        }

        let mut cur = nums[0];
        let mut couter = 1;
        let mut s = 0;
        let mut i = 1;
        loop {
            if i >= nums.len() {
                if couter > 2 {
                    nums.drain(s..);
                }
                break;
            }

            let num = nums[i];
            if num == cur {
                couter += 1;
                if couter == 3 {
                    s = i;
                }
                i += 1;
                continue;
            }

            if couter > 2 {
                nums.drain(s..i);
                i = s + 1;
            } else {
                i += 1;
            }
            cur = num;
            couter = 1;
        }

        nums.len() as i32
    }

    pub fn remove_duplicates2_2(nums: &mut Vec<i32>) -> i32 {
        let raw_len = nums.len();
        if raw_len < 3 {
            return raw_len as i32;
        }

        let mut i = 2;
        for j in 2..raw_len {
            let num = nums[j];
            if num != nums[i as usize - 2] {
                nums[i] = num;
                i += 1;
            }
        }

        i as i32
    }
}

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

    // let Range { start, end } = slice::range(0..3, ..5);
    // println!("start: {}, end: {}", start, end);
    // nums.drain(0..3);
    // println!("The value of nums: {:?}", nums);
    Solution::remove_duplicates2_2(&mut nums);
    println!("res: {:?}", nums);
}
