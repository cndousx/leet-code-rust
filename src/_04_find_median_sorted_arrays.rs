///
/// [4. 寻找两个正序数组的中位数](https://leetcode.cn/problems/median-of-two-sorted-arrays/)
///
struct Solution;
impl Solution {
    /// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
    ///
    ///算法的时间复杂度应该为 O(log (m+n))
    ///
    ///解题思路 用归并合并两个数组
    ///
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let mut nums: Vec<i32> = Vec::new();
        let len = m + n;
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;
        let mid = len / 2;
        // 不用全部都填加到新数组
        while count != (mid + 1) {
            if i == m {
                while j != n {
                    nums.insert(count, *nums2.get(j).unwrap());
                    j += 1;
                    count += 1;
                }
                break;
            };
            if j == n {
                while i != m {
                    nums.insert(count, *nums1.get(i).unwrap());
                    i += 1;
                    count += 1;
                }
                break;
            };
            let ins: i32;
            if nums1.get(i) < nums2.get(j) {
                ins = *nums1.get(i).unwrap();
                i += 1;
            } else {
                ins = *nums2.get(j).unwrap();
                j += 1;
            }
            nums.insert(count, ins);
            count += 1;
        }
        if len % 2 == 0 {
            (*nums.get(mid).unwrap() as f64 + *nums.get(mid - 1).unwrap() as f64) / 2.0
        } else {
            *nums.get(mid).unwrap() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
