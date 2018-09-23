pub mod tree {
    #[derive(Debug)]
    pub struct BTree<T: PartialOrd> {
        root: Option<Box<BNode<T>>>,
    }

    impl<T: PartialOrd> BTree<T> {
        pub fn new(new: Option<T>) -> BTree<T> {
            match new {
                None => BTree { root: None },
                Some(value) => BTree {
                    root: Some(Box::new(BNode {
                        value,
                        left_child: None,
                        right_child: None,
                    }))
                },
            }
        }

        pub fn insert(&mut self, value: T) {
            match self.root {
                None => {self.root = Some(Box::new(BNode {
                    value,
                    left_child: None,
                    right_child: None,
                }))}
                Some(ref mut x) => {x.insert(value);}
            }
        }
    }

    #[derive(Debug)]
    struct BNode<T: PartialOrd> {
        value: T,
        left_child: Option<Box<BNode<T>>>,
        right_child: Option<Box<BNode<T>>>,
    }

    impl<T: PartialOrd> BNode<T>{
        fn insert(&mut self, value: T){
            if self.value == value {
                return;
            }
            let current_node = if value < self.value {&mut self.left_child} else {&mut self.right_child};
            match current_node {
                &mut Some(ref mut subnode) => subnode.insert(value),
                &mut None => {
                    let new_node = BNode {value, left_child: None, right_child: None};
                    let boxed_node = Some(Box::new(new_node));
                    *current_node = boxed_node;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tree::*;

    #[test]
    fn test_construction() {
        let a: BTree<i32> = BTree::new(None);
        let mut b: BTree<i32> = BTree::new(Some(5));
        println!("{:?}\n{:?}", a, b);
        b.insert(3);
        b.insert(7);
        b.insert(2);
        b.insert(4);
        b.insert(6);
        b.insert(8);
        println!("{:?}", b);
    }
}
