use std::collections::{hash_map::Entry, HashMap, VecDeque};

// 最小覆盖子串
struct Solution;

// 计算当前新的子串范围，并返回第一个值
fn cal_cur_range(r: &mut HashMap<char, (usize, VecDeque<usize>)>) -> (char, (usize, usize)) {
    let mut first_c: char = ' ';
    let mut min = usize::MAX;
    let mut max = 0;
    for (k, (_, q)) in r {
        let f = *q.front().unwrap();
        let e = *q.back().unwrap();
        if f < min {
            min = f;
            first_c = *k;
        }
        if e > max {
            max = e;
        }
    }

    (first_c, (min, max))
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return "".to_string();
        }

        // store the value of charactor of the target string
        let mut records = HashMap::new();
        for c in t.chars() {
            let counter = records.entry(c).or_insert(0);
            *counter += 1;
        }
        let mut r: HashMap<char, (usize, VecDeque<usize>)> = records
            .into_iter()
            .map(|(k, v)| (k, (v, VecDeque::with_capacity(v))))
            .collect();

        // 首先，从头开始向尾部遍历，出现多余的重复值则直接选择后面出现的
        let mut chars = s.chars().enumerate();
        let mut remaining_chars = r.len();
        while let Some((i, c)) = chars.next() {
            match r.entry(c) {
                Entry::Occupied(mut e) => {
                    let counter = (*e.get()).0;
                    let q = &mut (*e.get_mut()).1;
                    if counter > q.len() {
                        q.push_back(i);
                        if counter == q.len() {
                            remaining_chars -= 1;
                        }
                    } else {
                        q.push_back(i);
                        q.pop_front();
                    }
                }
                Entry::Vacant(_) => {}
            }
            if remaining_chars == 0 {
                break;
            }
        }

        // find the min and max offset
        if remaining_chars != 0 {
            return "".to_string();
        }

        let (mut first_c, mut min_range) = cal_cur_range(&mut r);
        // 从当前的位置继续向后遍历，找到其它的可能更短的子串;
        // 找到的条件是，当前最早匹配的元素得到的新的匹配，这样可能会有一个更短的可能；
        // 当然，这个过程中可能出现中间元素的更后位置，如果有可应该尽量使用靠后的位置，这样匹配的子串才有可能最短;
        // 这个过程也可以使用反向的思路:
        // 在继续向后遍历的过程中，记录不同字符出现的累计次数，在第二次子串满足的情况，
        // 从左向右进行收缩，如果是目标字符，则减去次数，如果减去之后小于需要满足的次数，那么这个字符就是接下来
        // 向后匹配过程中匹配的元素。否则，可以继续收缩整体的大小。这样的好处是，不需要遍历整个 map 获取当前
        // 最左节点，坏处是每次从左向右需要重复的 hash，hash 也是一个开销不小的操作。
        for (i, c) in chars {
            let mut recal = false;
            match r.entry(c) {
                Entry::Occupied(mut e) => {
                    let q = &mut (*e.get_mut()).1;
                    q.push_back(i);
                    q.pop_front();
                    if c == first_c {
                        recal = true;
                    }
                }
                _ => {}
            }

            if recal {
                let (new_first_c, new_range) = cal_cur_range(&mut r);
                first_c = new_first_c;
                if new_range.1 - new_range.0 < min_range.1 - min_range.0 {
                    min_range = new_range;
                }
            }
        }

        s[min_range.0..min_range.1 + 1].to_string()
    }
}
