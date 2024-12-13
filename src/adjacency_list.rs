use crate::common::IsGraph;

// a list of lists where the value of list[x] is a list of nodes connected to node x

struct UniqueVec<T: PartialEq>{
    vec: Vec<T>
}

impl<T: PartialEq> UniqueVec<T>{
    fn new() -> UniqueVec<T> {
        UniqueVec { vec: Vec::new() }
    }

    fn contains(&self, item: &T) -> bool{
        self.vec.contains(item)
    }

    fn push(&mut self, item: T) {
        if self.contains(&item){
            return;
        }
        self.vec.push(item);
    }

    fn remove(&mut self, item: &T){
        if let Some(index) = self.vec.iter().position(|&x| x == item){
            self.vec.swap_remove(index)
        }
    }
}

struct AdjacencyList<const N: usize> {
    lists: [UniqueVec<usize>;N],
    is_directional: bool,
    is_cyclic: bool,
}

impl<const N: usize> IsGraph for AdjacencyList<N> {
    fn add_bidirectional_edge(&mut self, i: usize, j: usize) {
        self.lists[i].push(j);
        self.lists[j].push(i);
    }

    fn add_directional_edge(mut self, from: usize, to: usize) {
        self.lists[from].push(to);
        self.is_directional = true;
    }

    fn new() -> AdjacencyList<N> {
        Self{
            lists: [UniqueVec::<usize>::new(); N],
            is_directional: false,
            is_cyclic: false,
        }
    }

    fn is_cyclical(&self) -> bool {
        todo!()
    }

    fn check_directional(&mut self) {
        for (i, node) in &self.lists.iter().enumerate() {
            if !self.lists[i].contains(node){
                self.is_directional = true;
                return;
            }
        }
        self.is_directional = false;
    }

    fn get_targets(&self, node: usize) -> Vec<usize> {
        self.lists[node].vec.clone()
    }

    fn get_sources(&self, node: usize) -> Vec<usize> {
        self.lists.iter().enumerate().filter_map(|(n, i)| if i.contains(&node){ Some(n) } else { None }).collect::<Vec<_>>()
    }

    fn remove_bidirectional_edge(&mut self, i: usize, j: usize) {
        self.lists[i].remove(&j);
        self.lists[j].remove(&i);
    }

    fn remove_directional_edge(&mut self, from: usize, to: usize) {
        self.lists[from].remove(&to);
        self.check_directional();
    }
}