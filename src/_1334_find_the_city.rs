/// [1334. 阈值距离内邻居最少的城市](https://leetcode.cn/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/?envType=problem-list-v2&envId=shortest-path)
struct Solution;
impl Solution {
    /// 有 `n` 个城市，按从 `0` 到 `n-1` 编号。给你一个边数组`edges`，
    /// 其中 `edges[i] = [fromi, toi, weighti]` 代表 `fromi` 和 `toi` 两个城市之间的双向加权边，
    /// 距离阈值是一个整数 `distanceThreshold`。
    ///
    /// 返回在路径距离限制为 `distanceThreshold` 以内可到达城市最少的城市。如果有多个这样的城市，则返回编号最大的城市。
    ///
    ///注意，连接城市 `i` 和 `j` 的路径的距离等于沿该路径的所有边的权重之和。
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        // dist[i][j] 表示从 i 到 j 的最短距离
        let mut dist = vec![vec![usize::MAX; n]; n];
        for i in 0..n {
            // 自身到自身的距离为 0
            dist[i][i] = 0;
        }
        for edge in &edges {
            let [from, to, weight] = [edge[0] as usize, edge[1] as usize, edge[2] as usize];
            dist[from][to] = weight;
            dist[to][from] = weight;
        }
        for k in 0..n {
            // 最外层循环变量是 k，从 0 到 n，代表我们允许经过的“中间顶点”。
            for i in 0..n {
                // 第二层循环变量是 i，从 0 到 n，代表路径的“起点”
                for j in 0..n {
                    // 第三层循环变量是 j，从 0到 n，代表路径的“终点”

                    // i===>k===>j
                    if i == j {
                        continue;
                    }

                    if dist[i][k] == usize::MAX || dist[k][j] == usize::MAX {
                        // 如果i，k不通或者k，j不通，则不更新最短距离
                        continue;
                    }
                    dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
                }
            }
        }
        let dt = distance_threshold as usize;
        (0..n)
            .map(|i| {
                let count = dist[i].iter().filter(|&&num| num <= dt).count();
                (count, i)
            })
            .fold(
                (usize::MAX, 0),
                |ans, x| {
                    if ans.0 >= x.0 { (x.0, x.1) } else { ans }
                },
            )
            .1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            ),
            0
        );
    }
}
