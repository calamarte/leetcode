#[allow(unused_variables)]
fn main() {
    let (n, connections) = (
        6,
        vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
    );

    let (n, connections) = (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]);

    println!("Result -> {}", min_reorder(n, connections));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
/// WARN: It's a solution from leetcode :(
#[allow(unused_variables)]
fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    fn dfs(node: usize, parent: usize, edges: &Vec<Vec<(usize, bool)>>) -> i32 {
        let mut ans = 0;

        for &(neighbor, dir) in edges[node].iter() {
            if neighbor != parent {
                if dir {
                    ans += 1
                }
                ans += dfs(neighbor, node, edges);
            }
        }

        ans
    }

    let edges = {
        let mut ans = vec![Vec::new(); connections.len() + 1];
        for edge in connections {
            ans[edge[0] as usize].push((edge[1] as usize, true));
            ans[edge[1] as usize].push((edge[0] as usize, false));
        }
        ans
    };

    println!("Edges -> {edges:?}");

    dfs(0, n as usize, &edges)
}
