struct Solution;

impl Solution {
    // This implementation should be optimzed
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return Default::default();
        }

        let mut res: Vec<char> = vec![];
        let mut chars = vec![];
        for str in strs.iter() {
            chars.push(str.chars());
        }

        loop {
            let first = &chars[0].next();
            if first.is_none() {
                break;
            }

            let c = first.unwrap();
            for char_iter in &mut chars[1..] {
                if !char_iter
                    .next()
                    .map(|inner_c| inner_c == c)
                    .unwrap_or(false)
                {
                    // return the result
                    return res.into_iter().collect();
                }
            }

            res.push(c);
        }

        res.into_iter().collect()

        // for char in chars  {

        // }
    }
}
