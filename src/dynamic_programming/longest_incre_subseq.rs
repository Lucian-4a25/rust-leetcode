// 最长递增子序列，算法思维:
// - 逐步构建一个递增序列，从 0 开始为一个元素；在遍历之后的元素过程中，期望之前的序列中最慢的升序，也就是说，
// 将当前的元素值插入到之前有序的顺序中，找到其在其中最小的大于它的元素，然后替换，如果没有，则视为当前的元素最大，
// 将其 append 在子序列最右侧
// - 如果需要得知子序列的索引，或者子序列的元素值。需要进行一些技巧的判断，由于每次遍历只需要判断当前增长最缓慢的子序列，
// 所以只需要使用一个数组保存这些元素的索引。另外，由于我们并不知道哪些元素是最终会出现在最后的序列中的，所以需要记录当前
// 的元素出现在最长子序列时，它之前的元素下标是多少，这样我们可以依赖这个信息得知最长子序列的所有元素；
struct Solution;

// return the index to insert
pub fn find_insertion_pos(nums: &Vec<i32>, incre_seq_idx: &Vec<usize>, val: i32) -> usize {
    if incre_seq_idx.is_empty() {
        return 0;
    }

    let mut lo = 0;
    let mut hi = incre_seq_idx.len() - 1;

    while lo <= hi {
        let mid = (lo + hi) / 2;
        if nums[incre_seq_idx[mid]] < val {
            lo = mid + 1;
            continue;
        }

        if nums[incre_seq_idx[mid]] == val {
            return mid;
        }

        if mid == 0 {
            break;
        }
        hi = mid - 1;
    }

    lo
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let len = nums.len();
        let mut prev_idx = vec![0; len];
        let mut incre_seq_idx: Vec<usize> = vec![];

        let mut max_len = 0;
        for i in 0..len {
            let insert_pos = find_insertion_pos(&nums, &incre_seq_idx, nums[i]);

            if insert_pos + 1 > max_len {
                max_len = insert_pos + 1;
            }

            if insert_pos > 0 {
                prev_idx[i] = incre_seq_idx[insert_pos - 1];
            }
            if incre_seq_idx.len() > insert_pos {
                incre_seq_idx[insert_pos] = i;
            } else {
                incre_seq_idx.push(i);
            }
        }

        // construct increment sub sequence
        let mut idx = incre_seq_idx[max_len - 1];
        let mut result = vec![];
        for _ in 0..max_len {
            result.push(nums[idx]);
            idx = prev_idx[idx];
        }

        result.reverse();
        println!("The result sequence is: {:?}", result);

        max_len as i32
    }
}

#[test]
fn test_insertion_pos() {}
