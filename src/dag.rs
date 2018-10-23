
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

    pub fn find(&self, value: &T) -> Option<&T> {
        if self.structure[0].value == *value {
            Some(&self.structure[0].value)
        } else {
            match self.structure[0].find(self,0,value) {
                None => None,
                Some(idx) => Some(&self.structure[idx].value)
            }
        }
    }

    pub fn add_new(&mut self, parent: &T, value: T) {
        let node_from = self.structure[0].find(self,0,parent);
        match node_from {
            Some(from) => {
                let location = self.structure.len();
                let mut new_node = DAGNode{
                    value,
                    children: Vec::new(),
                    parents: Vec::new(),
                };
                new_node.parents.push(from);
                self.structure.push(new_node);
                self.structure[from].children.push(location);
            }
            None => (),
        }
    }
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl<T: Eq> DAGNode<T> {
    fn find(&self, holder :&DAG<T>, self_idx: usize, value: &T) -> Option<usize> {
        if self.value == *value {
            Some(self_idx)
        } else {
            let mut ret = None;
            for &i_child in &self.children {
                match ret {
                    None => {
                        ret = holder.structure[i_child].find(holder,i_child,value);
                    },
                    Some(_) => break,
                }
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let mut a = DAG::new(30);
        println!("{:?}", a);
        a.add_new(&30, 5);
        a.add_new(&30, 10);
        a.add_new(&5, 15);
        println!("{:?}", a);
    }
}