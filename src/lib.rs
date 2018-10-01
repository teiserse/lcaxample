pub mod tree {
    #[derive(Debug)]
    pub struct BTree<T: Ord> {
        root: Option<Box<BNode<T>>>,
    }

    use std::fmt;
    use std::mem;
    use std::cmp::Ordering;

    impl<T: Ord> BTree<T> {
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
                    }));
                }
                Some(ref mut x) => { x.insert(value); }
            }
        }

        pub fn remove(&mut self, value: T) {
            match self.root.take() {
                None => {}
                Some(x) => {
                    self.root = x.remove(value);
                }
            }
        }

        pub fn lca(&self, val1: &T, val2: &T) -> Option<&T> {
            match self.root {
                None => { None }
                Some(ref x) => {
                    if x.contains(val1) && x.contains(val2) {
                        x.lca(&val1, &val2)
                    } else { None }
                }
            }
        }

        pub fn contains(&self, value: &T) -> bool {
            match self.root {
                None => { false }
                Some(ref x) => { x.contains(value) }
            }
        }
    }

    impl<T: Ord + fmt::Display> fmt::Display for BTree<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.root {
                None => write!(f, "()"),
                Some(ref x) => write!(f, "{}", x.construct_disp("")),
            }
        }
    }

    #[derive(Debug)]
    struct BNode<T: Ord> {
        value: T,
        left_child: Option<Box<BNode<T>>>,
        right_child: Option<Box<BNode<T>>>,
    }

    impl<T: Ord> BNode<T> {
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
        fn remove(mut self, value: T) -> Option<Box<BNode<T>>> {
            match value.cmp(&self.value) {
                Ordering::Less => {
                    match self.left_child.take() {
                        None => Some(Box::new(self)),
                        Some(x) => {
                            self.left_child = x.remove(value);
                            Some(Box::new(self))
                        }
                    }
                }
                Ordering::Greater => {
                    match self.right_child.take() {
                        None => Some(Box::new(self)),
                        Some(x) => {
                            self.right_child = x.remove(value);
                            Some(Box::new(self))
                        }
                    }
                }
                Ordering::Equal => {
                    match (self.left_child.take(), self.right_child.take()) {
                        (None, None) => { None }
                        (Some(l), None) => { Some(l) }
                        (None, Some(r)) => { Some(r) }
                        (Some(mut l), Some(r)) => {
                            l.replace_with_successor(&mut self.value);
                            self.left_child = l.remove(value);
                            self.right_child = Some(r);
                            Some(Box::new(self))
                        }
                    }
                }
            }
        }

        fn replace_with_successor(&mut self, value: &mut T) {
            match self.right_child {
                Some(ref mut node) => node.replace_with_successor(value),
                None => mem::swap(&mut self.value, value),
            }
        }

        fn lca(&self, val1: &T, val2: &T) -> Option<&T> {
            if self.value > *val1 && self.value > *val2 {
                match self.left_child {
                    None => { None }
                    Some(ref x) => { x.lca(val1, val2) }
                }
            } else if self.value < *val1 && self.value < *val2 {
                match self.right_child {
                    None => { None }
                    Some(ref x) => { x.lca(val1, val2) }
                }
            } else { Some(&self.value) }
        }

        fn contains(&self, value: &T) -> bool {
            match value.cmp(&self.value) {
                Ordering::Equal => {
                    true
                }
                Ordering::Less => {
                    match self.left_child {
                        None => { false }
                        Some(ref x) => { x.contains(value) }
                    }
                }
                Ordering::Greater => {
                    match self.right_child {
                        None => { false }
                        Some(ref x) => { x.contains(value) }
                    }
                }
            }
        }
    }


    //  ├ │ ─ └

    impl<T: Ord + fmt::Display> BNode<T> {
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
        //let a: BTree<i32> = BTree::new(None);
        let mut b: BTree<i32> = BTree::new(Some(5));
        //println!("{:?}\n{:?}", a, b);
        b.insert(3);
        b.insert(7);
        b.insert(2);
        b.insert(4);
        b.insert(6);
        b.insert(8);
        println!("{}", b);
        assert_eq!(b.to_string(), "─5\n ├─3\n │ ├─2\n │ └─4\n └─7\n   ├─6\n   └─8\n");
        b.remove(8);
        assert_eq!(b.to_string(), "─5\n ├─3\n │ ├─2\n │ └─4\n └─7\n   └─6\n");
        b.remove(3);
        assert_eq!(b.to_string(), "─5\n ├─2\n │ └─4\n └─7\n   └─6\n");
    }

    #[test]
    fn test_contains() {
        let mut b: BTree<i32> = BTree::new(Some(5));
        b.insert(3);
        b.insert(7);
        b.insert(2);
        b.insert(4);
        b.insert(6);
        b.insert(8);
        assert_eq!(b.contains(&3), true);
        assert_eq!(b.contains(&9), false);
    }

    #[test]
    fn test_lca() {
        let mut b: BTree<i32> = BTree::new(Some(5));
        b.insert(3);
        b.insert(7);
        b.insert(2);
        b.insert(4);
        b.insert(6);
        b.insert(8);
        assert_eq!(b.lca(&2, &7), Some(&5));
        assert_eq!(b.lca(&3, &4), Some(&3));
        assert_eq!(b.lca(&1, &5), None);
    }
}
