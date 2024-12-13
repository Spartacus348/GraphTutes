use std::collections::HashMap;
use std::hash::Hash;
use crate::common::{IsGraph, Ray};

pub struct AdjacencyMap<N: Copy+Eq+Hash, E: Ord+Copy> {
    graph: HashMap<N,HashMap<N,E>>,
    is_directional: bool,
    is_cyclical: bool
}

impl<N: Copy+Eq+Hash, E: Ord+Copy> IsGraph<N, E> for AdjacencyMap<N, E> {
    fn add_bidirectional_edge(&mut self, i: N, j: N, val: E) {
        if self.graph.contains_key(&i) && self.graph.contains_key(&j) {
            self.graph.get_mut(&i).unwrap().insert(j, val);
            self.graph.get_mut(&j).unwrap().insert(i, val);
        }
    }

    fn add_directional_edge(&mut self, from: N, to: N, val: E) {
        if let Some(x) = self.graph.get_mut(&from) {
            x.insert(to, val);
            self.is_directional = true;
        }
    }

    fn new(nodes: Box<dyn Iterator<Item=N>>) -> Self {
        let mut empty_map = HashMap::<N, HashMap<N,E>>::new();
        for n in nodes{
            empty_map.insert(n,HashMap::<N,E>::new());
        }
        Self{
            graph: empty_map,
            is_directional: false,
            is_cyclical: false
        }
    }

    fn is_cyclical(&self) -> bool {
        todo!()
    }

    fn check_directional(&mut self) {
        for i in self.graph.keys() {
            for j in self.graph.keys(){
                if self.graph[i][j] != self.graph[j][i]{
                    self.is_directional = true;
                    return;
                }
            }
        }
        self.is_directional = false
    }

    fn get_targets(&self, node: N) -> Vec<Ray<N, E>> {
        if let Some(targets) = self.graph.get(&node){
            targets.iter().map(|(&k,&v)| Ray::new(k,v)).collect()
        } else{
            Vec::new()
        }
    }

    fn get_sources(&self, node: N) -> Vec<Ray<N, E>> {
        self.graph.iter().filter_map(
            |(k,v)| if let Some(&s) = v.get(&node){
                Some(Ray::new(*k,s))
            } else {None})
            .collect()
    }

    fn remove_bidirectional_edge(&mut self, i: N, j: N) {
        if let Some(x) = self.graph.get_mut(&i){
            x.remove(&j);
        }
        if let Some(y) = self.graph.get_mut(&j){
            y.remove(&i);
        }
    }

    fn remove_directional_edge(&mut self, from: N, to: N) {
        if let Some(x) = self.graph.get_mut(&from){
            x.remove(&to);
        }
    }
    
    fn add_node(&mut self, n: N) {
        self.graph.insert(n, HashMap::new());
    }
}

