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
                None => {
                    self.root = Some(Box::new(BNode {
                        value,
                        left_child: None,
                        right_child: None,
                    }))
                }
                Some(ref mut x) => { x.insert(value); }
            }
        }
    }

    use std::fmt;

    impl<T: PartialOrd + fmt::Display> fmt::Display for BTree<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.root {
                None => write!(f, "()"),
                Some(ref x) => write!(f, "{}", x.construct_disp("")),
            }
        }
    }

    #[derive(Debug)]
    struct BNode<T: PartialOrd> {
        value: T,
        left_child: Option<Box<BNode<T>>>,
        right_child: Option<Box<BNode<T>>>,
    }

    impl<T: PartialOrd> BNode<T> {
        fn insert(&mut self, value: T) {
            if self.value == value {
                return;
            }
            let current_node = if value < self.value { &mut self.left_child } else { &mut self.right_child };
            match current_node {
                &mut Some(ref mut subnode) => subnode.insert(value),
                &mut None => {
                    let new_node = BNode { value, left_child: None, right_child: None };
                    let boxed_node = Some(Box::new(new_node));
                    *current_node = boxed_node;
                }
            }
        }
    }


    //  ├ │ ─ └

    impl<T: PartialOrd + fmt::Display> BNode<T> {
        fn construct_disp(&self, prefix: &str) -> String {
            let mut holder = String::new();
            holder.push_str(&prefix);
            holder.push('─');
            holder.push_str(&self.value.to_string());
            holder.push('\n');
            let mut new_prefix = String::from(prefix);
            new_prefix = new_prefix.replace("├", "│");
            new_prefix = new_prefix.replace("└", " ");

            match (&self.left_child, &self.right_child) {
                (None, None) => (),
                (Some(l), None) => {
                    new_prefix.push_str(" └");
                    holder.push_str(&l.construct_disp(&new_prefix));
                }
                (None, Some(r)) => {
                    new_prefix.push_str(" └");
                    holder.push_str(&r.construct_disp(&new_prefix));
                }
                (Some(l), Some(r)) => {
                    new_prefix.push_str(" ├");
                    holder.push_str(&l.construct_disp(&new_prefix));
                    let newlen = new_prefix.len() - " ├".len();
                    new_prefix.truncate(newlen);
                    new_prefix.push_str(" └");
                    holder.push_str(&r.construct_disp(&new_prefix));
                }
            }
            holder
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
        println!("{}", b);
    }
}
