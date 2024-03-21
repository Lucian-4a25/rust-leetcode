use std::collections::HashSet;

// 无重复字符的最长子串
struct Solution;

impl Solution {
    // 使用滑动窗口，类似于，使用一个索引下标表示当前队列中最左侧位置，这样我们可以使用 Set
    // 取代 Vec 来存储不重复的子串（对查询更加友好）
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut max = 0;
        let mut subseq = HashSet::<char>::new();
        let chars = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        for (i, &c) in chars.iter().enumerate() {
            if subseq.contains(&c) {
                max = std::cmp::max(max, i - left);
                loop {
                    let removed = &chars[left];
                    left += 1;
                    subseq.remove(removed);
                    if removed == &c {
                        break;
                    }
                }
            }

            subseq.insert(c);
        }
        if chars.len() - left > max {
            max = chars.len() - left;
        }

        max as i32
    }

    // normal mind version
    pub fn length_of_longest_substring_normal(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut max = 0;
        let mut subseq = vec![];
        let chars = s.chars().collect::<Vec<char>>();
        for &c in chars.iter() {
            let pos_op = subseq.iter().position(|&s| s == c);
            if !pos_op.is_some() {
                subseq.push(c);
                continue;
            }
            if subseq.len() > max {
                max = subseq.len();
            }

            let pos = pos_op.unwrap();
            subseq.drain(0..pos + 1);
            subseq.push(c);
        }
        if subseq.len() > max {
            max = subseq.len()
        }

        return max as i32;
    }
}
