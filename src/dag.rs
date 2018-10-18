
#[derive(Debug)]
struct DAG<T: Eq> {
    structure: Vec<DAGNode<T>>,
}

impl<T: Eq> DAG<T> {
    pub fn new(value: T) -> DAG<T> {
        let mut new_dag = DAG {
            structure: Vec::new()
        };
        new_dag.structure[0] = DAGNode{
            value,
            children: Vec::new(),
            parents: Vec::new(),
        };
        new_dag
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        if self.structure[0].value == *value {
            Some(&self.structure[0].value)
        } else {
            match self.structure[0].find(self,value) {
                None => None,
                Some(node) => Some(&node.value)
            }
        }
    }

    //pub fn add_new(&mut self, _parent: &T, _value: T) {

    //}
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl<T: Eq> DAGNode<T> {
    fn find<'a>(&'a self, holder :&'a DAG<T>, value: &T) -> Option<&'a DAGNode<T>> {
        if self.value == *value {
            Some(&self)
        } else {
            let mut ret = None;
            for i_child in &self.children {
                match ret {
                    None => {
                        ret = holder.structure[*i_child].find(holder,value);
                    },
                    Some(_) => break,
                }
            }
            ret
        }
    }

    //fn add_new(&mut self, parent: &T, value: T) {

    //}
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