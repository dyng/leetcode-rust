use std::cmp;

fn main() {
    let left = Some(box TreeNode {
        val: 1,
        left: None,
        right: None,
    });
    let input = Some(box TreeNode {
        val: 0,
        left: left,
        right: None,
    });

    let output = max_depth(input);
    println!("{}", output);
}

fn max_depth(root : Tree<int>) -> int {
    match root {
        Some(node) => {
            let child_depth = cmp::max(max_depth(node.left), max_depth(node.right));
            child_depth + 1
        },
        None => 0,
    }
}

type Tree<T> = Option<Box<TreeNode<T>>>;

struct TreeNode<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}
