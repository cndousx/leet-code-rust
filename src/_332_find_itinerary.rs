use std::collections::HashMap;

///
/// [332. 重新安排行程](https://leetcode.cn/problems/reconstruct-itinerary/?envType=problem-list-v2&envId=sorting)
///
struct Solution;
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for ticket in &tickets {
            graph
                .entry(ticket[0].clone())
                .or_default()
                .push(ticket[1].clone());
        }
        for dests in graph.values_mut() {
            dests.sort_unstable();
            dests.reverse();
        }

        let mut result = Vec::with_capacity(tickets.len() + 1);
        Self::dfs(&mut graph, "JFK".to_string(), &mut result);
        result.reverse();
        result
    }

    fn dfs(graph: &mut HashMap<String, Vec<String>>, curr: String, result: &mut Vec<String>) {
        while let Some(next) = graph.get_mut(&curr).and_then(|d| d.pop()) {
            Self::dfs(graph, next, result);
        }
        result.push(curr);
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t1() {
        let ans = Solution::find_itinerary(vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ]);
        assert_eq!(
            vec![
                "JFK".to_string(),
                "MUC".to_string(),
                "LHR".to_string(),
                "SFO".to_string(),
                "SJC".to_string()
            ],
            ans
        );
    }
}
