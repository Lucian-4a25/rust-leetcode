//  三角形最小路径和

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }

        let mut sum_result = vec![triangle[0][0]];
        for level in triangle.iter().skip(1) {
            println!(
                "level len: {}, sum_result_len: {}",
                level.len(),
                sum_result.len()
            );
            let mut next_sum = vec![];
            for (i, &sum) in sum_result.iter().enumerate() {
                let first_sum = level[i] + sum;
                let second_sum = level[i + 1] + sum;

                if i == 0 {
                    next_sum.push(first_sum);
                } else if next_sum[i] > first_sum {
                    next_sum[i] = first_sum;
                }

                next_sum.push(second_sum);
            }

            sum_result = next_sum;
        }

        sum_result.into_iter().min().unwrap()
    }
}
