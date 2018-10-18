
#[derive(Debug)]
struct DAG<T: Eq> {
    structure: Vec<DAGNode<T>>,
}

impl<T: Eq> DAG<T> {
    pub fn new(value: T) -> DAG<T> {
        let mut new_dag = DAG {
            structure: Vec::new()
        };
        new_dag.structure.push(DAGNode{
            value,
            children: Vec::new(),
            parents: Vec::new(),
        });
        new_dag
    }

    pub fn find<>(&self, value: &T) -> Option<&T> {
        None
    }

    pub fn add_new(&mut self, _parent: &T, _value: T) {

    }
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl<T: Eq> DAGNode<T> {
    fn find(&self, value: &T) -> Option<&DAGNode<T>> {
        None
    }

    fn add_new(&mut self, parent: &T, value: T) {

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