pub mod tree {
    #[derive(Debug)]
    pub struct BTree<T: PartialOrd> {
        root: Option<Box<BNode<T>>>,
    }

    impl<T: PartialOrd> BTree<T> {
        pub fn new(value: Option<T>) -> BTree<T> {
            match value {
                None => BTree { root: None },
                Some(x) => BTree {
                    root: Some(Box::new(BNode {
                        parent: None,
                        value: x,
                        left_child: None,
                        right_child: None,
                    }))
                },
            }
        }
    }

    #[derive(Debug)]
    struct BNode<T: PartialOrd> {
        parent: Option<Box<BNode<T>>>,
        value: T,
        left_child: Option<Box<BNode<T>>>,
        right_child: Option<Box<BNode<T>>>,
    }
}

#[cfg(test)]
mod tests {
    use tree::*;

    #[test]
    fn test_construction() {
        let a: BTree<i32> = BTree::new(None);
        let b: BTree<i32> = BTree::new(Some(5));
        println!("{:?}\n{:?}", a, b);
    }
}
