///
/// [233. 数字1的个数](https://leetcode.cn/problems/number-of-digit-one/?envType=problem-list-v2&envId=dynamic-programming)
///
struct Solution;
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let mut result = 0;
        let mut num = n;
        let mut exp = 0;

        while num > 0 {
            let remainder = num % 10;
            num = num / 10;
            let p = 10i32.pow(exp);
            let count = if remainder > 1 {
                num * p + p
            } else if remainder == 1 {
                num * p + n % p + 1
            } else {
                // 当前位的数字是0
                num * p
            };
            result += count;
            exp += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // 1,10,11,12,13
        // 这些数字中11包含两个1，其他都是包含1个，总计6个
        assert_eq!(Solution::count_digit_one(13), 6);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::count_digit_one(0), 0);
    }

    #[test]
    fn t3() {
        // 从最低位到最高位
        // 2 > 1，故个位为1的次数为80818+1=80819；(个位固定为1(xxxxx1) 从000001到808181,总共80819个数字)
        // 8 > 1，故十位为1的个数为80810+10=80820；（十位固定为1(xxxx1x) 左边的四个x是从1到8081(共计8081种可能)，因为当前固定的是十位，所有个位还有十种可能所以乘上10，最后再加上左边都是0的情况共10种）
        // 1 = 1，故百位为1的个数为80800+82+1=80883；(百位固定为1(xxx1xx)左侧三个x是从1到808，攻击808种可能，当左边全是0, 右侧可以从0到82，所以是83种可能)
        // 8 > 1，故千位为1的个数为80000+1000=81000；(千位固定为1，其余同上,xx1xxx 1到80共80种可能，当左侧都为0,右侧xxx有1000种可能)
        // 0 < 1，故万位为1的个数为80000；(万位固定为1，x1xxxx，左侧x只能取0到7，如果取8就超过n了，左侧八种可能，右侧10000种可能，所以总共80000种可能)
        // 8 > 1，故十万位为1的个数为100000；(十万位固定为1,1xxxxx,共100000万可能)
        // 求和即得：503522
        assert_eq!(Solution::count_digit_one(808182), 503522);
    }
}
