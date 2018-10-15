
use std::rc::Rc;

#[derive(Debug)]
pub struct DAG<T : Eq> {
    root: Rc<DAGNode<T>>,
}

impl<T:Eq> DAG<T> {
    pub fn new(value: T) -> DAG<T> {
        DAG {
            root: Rc::new(DAGNode{
                value,
                children: Vec::new(),
                parents: Vec::new(),
            })
        }
    }
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<Rc<DAGNode<T>>>,
    parents: Vec<Rc<DAGNode<T>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construction(){
        let a = DAG::new(30);
        println!("{:?}",a);
    }
}