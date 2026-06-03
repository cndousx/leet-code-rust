///
/// [85. 最大矩形](https://leetcode.cn/problems/maximal-rectangle/)
///
struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = (&matrix[0]).len();
        if n == 0 {
            return 0;
        }
        fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            // nested functions
            let mut heights = heights;
            heights.insert(0, 0);
            heights.push(0);
            // 单调栈
            let mut stack = vec![];
            let mut max_area = 0;
            for i in 0..heights.len() {
                while !stack.is_empty() && heights[*stack.last().unwrap()] > heights[i] {
                    let h = heights[stack.pop().unwrap()];
                    let w = if stack.is_empty() {
                        i
                    } else {
                        i - *stack.last().unwrap() - 1
                    } as i32;
                    max_area = max_area.max(h * w);
                }
                stack.push(i);
            }
            max_area
        }

        let mut heights = vec![0; n];
        let mut max_area = -1;
        for i in 0..m {
            for j in 0..n {
                heights[j] = if matrix[i][j] == '1' {
                    heights[j] + 1
                } else {
                    0
                };
            }
            max_area = max_area.max(largest_rectangle_area(heights.clone()));
        }
        return max_area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::maximal_rectangle(vec![vec!['1']]);
        assert_eq!(ans, 1);
    }

    #[test]
    fn t2() {
        let ans = Solution::maximal_rectangle(vec![vec!['0']]);
        assert_eq!(ans, 0);
    }

    #[test]
    fn t3() {
        let ans = Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ]);
        assert_eq!(ans, 6);
    }
    #[test]
    fn t4() {
        let ans = Solution::maximal_rectangle(vec![vec!['0', '1'], vec!['1', '0']]);
        assert_eq!(ans, 1);
    }
}
