struct Solution;

impl Solution {
    // 买卖股票的最佳时机 III (只能交易两次)
    // 和第二个版本的状态思维，两次交易机会的情况，每天结束交易后，会存下面几种状态:
    // - 没有进行任何交易 (对状态不影响，忽略)
    // - 买入了一次
    // - 买入卖出了一次
    // - 交易过一次，进行了第二次买入
    // - 交易过一次，进行了第二次卖出
    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut prev_state = [-prices[0], 0, -prices[0], 0];
        for i in 1..prices.len() {
            let buy1 = std::cmp::max(prev_state[0], -prices[i]);
            let sell1 = std::cmp::max(prev_state[1], prev_state[0] + prices[i]);
            let buy2 = std::cmp::max(prev_state[2], prev_state[1] - prices[i]);
            let sell2 = std::cmp::max(prev_state[3], prev_state[2] + prices[i]);
            prev_state = [buy1, sell1, buy2, sell2];
        }

        std::cmp::max(prev_state[1], prev_state[3])
    }

    // 买卖股票的最佳时机 II (可以无限次交易)
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut max_prifit = 0;

        for i in 0..prices.len() - 1 {
            if prices[i + 1] - prices[i] > 0 {
                max_prifit += prices[i + 1] - prices[i];
            }
        }

        max_prifit
    }

    // 买卖股票的最佳时机 II (可以无限次交易)，使用动态规划的方式
    pub fn max_profit_v2_dynamic(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        // 记录每天完成交易后的不同状态的最优值，0 位置存放没有股票的最优值， 1 存放有股票的最优值
        let mut prev_state = [0, -prices[0]];

        for i in 1..prices.len() {
            // 没有股票有两种情况，卖出了持有的，或者之前就没有进行买入
            let s1 = std::cmp::max(prev_state[0], prev_state[1] + prices[i]);
            // 持有股票有有两种情况，持有了旧有的状态，或者进行了买入
            let s2 = std::cmp::max(prev_state[1], prev_state[0] - prices[i]);
            prev_state = [s1, s2];
        }

        prev_state[0]
    }

    // 买卖股票的最佳时机 I (只能交易一次)
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = i32::MAX;

        for v in prices {
            if v <= min_price {
                min_price = v;
            }

            if v - min_price > max_profit {
                max_profit = v - min_price;
            }
        }

        max_profit
    }

    /* pub fn max_profit_v3_fail(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let len = prices.len();
        let mut trade_point = 0;
        let mut trade_vals = vec![];

        while trade_point < len - 1 {
            // 从递减序列中找到最小值；如果是递减的情况，那么大的值不可能比小的值是更优解
            // - 如果高的值是最优解，那么如果在上升前卖出，肯定是损失；而我们期望的是获取一个正值。
            // - 如果高的值是最优解，如果再上升后卖出，肯定比选择更低的点买入的收益要低。
            for i in trade_point..len {
                let trade_low = prices[i];
                if i == len - 1 {
                    trade_point = i;
                    break;
                }
                if trade_low >= prices[i + 1] {
                    continue;
                }

                // 从 i 开始后，如果处于递增的情况，那么当前最优卖出一定是在最高点
                // - 如果选择较低的点卖出，在之后在这个上升序列的其它点买入，那么接下来选择的买入点一定在
                //   当前卖出点的相邻位置；由于我们可以确认的是，在最高点后必然存在一个下降点，而这个下降点一定比
                //   当前的高点要低；那么，这两种情况对比来看，获得的收益差取决于这两个点的位置大小；
                //   如果这个点要比上升点的所有点都要低，显然选择高点卖出是更好的方案。如果这个点比当前的点要高，那么损失
                //   的部分在于这两个点的差值，而这个差值，一定是小于早卖出的点和高点的差值，因为这个点比如比最高值要低。
                //   所以，在升序的点中，选择最高点卖出必然是最优解。
                let mut trade_high = prices[i + 1];
                if i + 2 >= len {
                    trade_point = len;
                }
                for j in i + 2..len {
                    if j == len - 1 {
                        trade_point = j;
                    }
                    if trade_high > prices[j] {
                        trade_point = j;
                        break;
                    }
                    trade_high = prices[j];
                }

                // println!("trade_low: {}, trade high: {}", trade_low, trade_high);
                trade_vals.push((trade_low, trade_high));
                break;
            }
        }

        if trade_vals.is_empty() {
            return 0;
        }

        // let mut max_1 = 0;
        // let mut max_2 = 0;
        // for val in trade_vals.into_iter() {
        //     if val > max_2 {
        //         max_2 = val;
        //         if max_2 > max_1 {
        //             let temp = max_1;
        //             max_1 = max_2;
        //             max_2 = temp;
        //         }
        //     }
        // }

        0
    } */
}

#[test]
fn test_max_profile() {
    // Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]);
}
