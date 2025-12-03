use std::fs::read_to_string;

/*

i [0, 12]

j [0, str.len)

a(i, j) = max(str[j] + a(i-1, j-1), a(i, j-1));
 */

const SIZE: usize = 12;

pub fn day3(filename: &str) -> u64 {
    let mut ans = 0;
    for x in read_to_string(filename).unwrap().lines() {
        let mut dp1: [u64; SIZE] = [0; SIZE];
        let mut dp: [u64; SIZE] = [0; SIZE];

        for j in (0..x.len()).rev() {
            let c: u64 = x.chars().nth(j).unwrap().to_digit(10).unwrap().into();
            for i in 0..dp.len().min(x.len() - j) {
                if i == 0 {
                    dp[i] = dp1[i].max(c);
                } else {
                    dp[i] = dp1[i].max(c * 10u64.pow(i as u32) + dp1[i - 1]);
                }
            }
            dp1.clone_from_slice(&dp);
        }
        //println!("{}", dp[dp.len() - 1]);

        ans += dp[dp.len() - 1];
    }
    ans
}