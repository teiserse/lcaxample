use std::collections::VecDeque;

#[derive(Debug)]
struct DAG<T: Eq> {
    structure: Vec<DAGNode<T>>,
}

impl<T: Eq> DAG<T> {
    pub fn new(value: T) -> DAG<T> {
        let mut new_dag = DAG {
            structure: Vec::new()
        };
        new_dag.structure.push(DAGNode {
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
            match self.structure[0].find(self, 0, value) {
                None => None,
                Some(idx) => Some(&self.structure[idx].value)
            }
        }
    }

    pub fn add_new(&mut self, parent: &T, value: T) {
        if self.find(&value).is_some() {
            return;
        }
        let node_from = self.structure[0].find(self, 0, parent);
        match node_from {
            Some(from) => {
                let location = self.structure.len();
                let mut new_node = DAGNode {
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

    pub fn add_link(&mut self, parent: &T, child: &T) {
        if *parent == *child { return; }
        let from = match self.structure[0].find(self, 0, parent) {
            Some(index) => { index }
            None => { return; }
        };
        let to = match self.structure[0].find(self, 0, child) {
            Some(index) => { index }
            None => { return; }
        };
        if self.structure[to].find(self, to, parent).is_some() {
            panic!("DAG attempted to link into a cycle.");
        };
        self.structure[from].children.push(to);
        self.structure[to].parents.push(from);
    }

    pub fn lca(&self, a: &T, b: &T) -> Option<&T> {
        let a_idx = match self.structure[0].find(self, 0, a) {
            Some(index) => { index }
            None => { return None; }
        };
        let b_idx = match self.structure[0].find(self, 0, b) {
            Some(index) => { index }
            None => { return None; }
        };
        if a_idx == b_idx { return Some(&self.structure[a_idx].value); }
        let mut bfs_queue: VecDeque<usize> = VecDeque::new();
        for initial in &self.structure[a_idx].parents {
            bfs_queue.push_back(*initial);
        }
        while !bfs_queue.is_empty() {
            let node = match bfs_queue.pop_front() {
                Some(index) => index,
                None => break,
            };
            for deeper_node in &self.structure[node].parents {
                bfs_queue.push_back(*deeper_node);
            }
            match self.structure[node].find(self, node, b) {
                None => {}
                Some(_) => {
                    return Some(&self.structure[node].value);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct DAGNode<T: Eq> {
    value: T,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl<T: Eq> DAGNode<T> {
    fn find(&self, holder: &DAG<T>, self_idx: usize, value: &T) -> Option<usize> {
        if self.value == *value {
            Some(self_idx)
        } else {
            let mut ret = None;
            for &i_child in &self.children {
                match ret {
                    None => {
                        ret = holder.structure[i_child].find(holder, i_child, value);
                    }
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
        //println!("{:?}", a);
        a.add_new(&30, 5);
        a.add_new(&30, 10);
        a.add_new(&5, 15);
        //println!("{:?}", a);
        a.add_link(&10, &15);
        //println!("{:?}", a);
    }

    #[test]
    #[should_panic(expected = "DAG attempted to link into a cycle.")]
    fn test_acyclic() {
        // this test is designed to fail and should fail, as success creates an invalid situation
        let mut b = DAG::new(1);
        b.add_new(&1, 2);
        b.add_new(&2, 3);
        b.add_new(&3, 4);
        b.add_link(&4, &2);
    }

    #[test]
    fn test_lca() {
        // A
        // |\
        // B C
        // |/|
        // D E
        // | |\
        // F G H
        //  \|
        //   I
        let mut c = DAG::new('A');
        c.add_new(&'A', 'B');
        c.add_new(&'A', 'C');
        c.add_new(&'B', 'D');
        c.add_link(&'C', &'D');
        c.add_new(&'C', 'E');
        c.add_new(&'D', 'F');
        c.add_new(&'E', 'G');
        c.add_new(&'E', 'H');
        c.add_new(&'F', 'I');
        c.add_link(&'G', &'I');
        //println!("{:?}", c);
        assert_eq!(c.lca(&'B', &'C'), Some(&'A'));
        assert_eq!(c.lca(&'F', &'H'), Some(&'C'));
        assert_eq!(c.lca(&'I', &'E'), Some(&'E'));
        assert_eq!(c.lca(&'G', &'G'), Some(&'G'));
        assert_eq!(c.lca(&'J', &'C'), None);
    }
}