struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len() - 1;
        // start from 0
        let mut curr_node = vec![0];
        let mut res: Vec<Vec<i32>> = vec![];

        Solution::dfs(n, 0, &mut res, &mut curr_node, &graph);

        res
    }

    pub fn dfs(
        n: usize,
        cn: usize,
        res: &mut Vec<Vec<i32>>,
        curr_node: &mut Vec<i32>,
        graph: &Vec<Vec<i32>>,
    ) {
        if cn == n {
            res.push(curr_node.clone());
            return;
        }

        for (i, _) in graph[cn].iter().enumerate() {
            // push current node
            curr_node.push(graph[cn][i]);
            // recursion
            /*
            check current node, the logic is following
            if the current node is not equal to node, we continue search (marking as visited) until reach (or not) the target node
            then we unmark nodes as visited (pop from the stack) and starting our search from the next vertex
             */
            Solution::dfs(n, graph[cn][i] as usize, res, curr_node, graph);
            let _ = curr_node.pop();
        }
    }
}

mod tests {
    use crate::all_paths_from_source_to_target::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![vec![0, 1, 3], vec![0, 2, 3]],
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]])
        );
    }
}
