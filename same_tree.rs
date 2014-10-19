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

    let output = is_same_tree(&input, &None);
    println!("{}", output);
}

fn is_same_tree(p: &Tree<int>, q: &Tree<int>) -> bool {
    match (p, q) {
        (&None, &None) => true,
        (&Some(ref a), &Some(ref b)) => {
            if a.val == b.val
            && is_same_tree(&a.left, &b.left)
            && is_same_tree(&a.right, &b.right){
                true
            } else {
                false
            }
        },
        (_, _) => false,
    }
}

type Tree<T> = Option<Box<TreeNode<T>>>;

struct TreeNode<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}
