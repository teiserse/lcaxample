use std::rc::Rc;

#[derive(Debug)]
pub struct DAG<T: Eq> {
    root: Rc<DAGNode<T>>,
}

impl<T: Eq> DAG<T> {
    pub fn new(value: T) -> DAG<T> {
        DAG {
            root: Rc::new(DAGNode {
                value,
                children: Vec::new(),
                parents: Vec::new(),
            })
        }
    }

    pub fn find<>(&self, value: &T) -> Option<&T> {
        let node = self.root.find(value);
        match node {
            None => None,
            Some(loc) => Some(&loc.value)
        }
    }
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<Rc<DAGNode<T>>>,
    parents: Vec<Rc<DAGNode<T>>>,
}

impl<T: Eq> DAGNode<T> {
    fn find(&self, value: &T) -> Option<&Rc<DAGNode<T>>> {
        let mut current = None;
        for child in &self.children {
            if child.value == *value {
                current = Some(child);
            } else {
                match current {
                    Some(_) => (),
                    None => {
                        current = child.find(value);
                    }
                }
            }
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let a = DAG::new(30);
        println!("{:?}", a);
    }
}