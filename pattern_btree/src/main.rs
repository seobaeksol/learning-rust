// An ordered collection of `T`s.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// tells Rust that we're defining a method on `BinaryTree`s of ordered types.
impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        let mut place = self;
        while let BinaryTree::NonEmpty(node) = place {
            if value <= node.element {
                place = &mut node.left;
            } else {
                place = &mut node.right;
            }
        }

        *place = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");

    let mut todo = Vec::new();
    if let BinaryTree::NonEmpty(ref x) = tree {
        todo.push(x);
    }

    while let Some(x) = &todo.pop() {
        if let BinaryTree::NonEmpty(ref left) = x.left {
            todo.push(left);
        }
        if let BinaryTree::NonEmpty(ref right) = x.right {
            todo.push(right);
        }
        println!("{}", x.element);
    }
}
