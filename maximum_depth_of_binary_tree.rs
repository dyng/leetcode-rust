use std::cmp;

fn main() {
    let input = Some(box TreeNode {
        val: 0,
        left: None,
        right: None,
    });
    let output = max_depth(input);
    println!("{}", output);
}

fn max_depth(root : Option<Box<TreeNode<int>>>) -> int {
    match root {
        Some(node) => {
            let child_depth = cmp::max(max_depth(node.left), max_depth(node.right));
            child_depth + 1
        },
        None => 0,
    }
}

struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
