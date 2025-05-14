///
/// # [22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
///
fn main() {
    let n = 3;

    println!("Result -> {:?}", generate_parenthesis(n));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(4^n / \sqrt n)$$
/// - Space complexity: $$O(4^n / \sqrt n)$$
fn generate_parenthesis(n: i32) -> Vec<String> {
    fn dfs(drill: i32, (open, close): (i32, i32), prefix: &mut String, storage: &mut Vec<String>) {
        if open == close && open == drill {
            storage.push(prefix.clone());
            return;
        }

        if open < drill {
            prefix.push('(');
            dfs(drill, (open + 1, close), prefix, storage);
            prefix.pop();
        }

        if close < open {
            prefix.push(')');
            dfs(drill, (open, close + 1), prefix, storage);
            prefix.pop();
        }
    }

    let mut result = Vec::new();
    dfs(n, (0, 0), &mut String::new(), &mut result);
    result
}
