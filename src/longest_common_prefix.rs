struct Solution;

impl Solution {
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
                    return String::from_iter(res.into_iter());
                }
            }

            res.push(c);
        }

        String::from_iter(res.into_iter())

        // for char in chars  {

        // }
    }
}
