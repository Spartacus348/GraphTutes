use crate::common::IsGraph;

// a matrix where the value of (x,y) says if there is an edge from x to y
#[derive(Debug)]
struct AdjacencyMatrix<const N: usize, T: Ord+Copy>{
    matrix: [[T; N]; N],
    is_directional: bool,
    is_cyclical: bool,
}

impl<const N: usize, T: Ord+Copy> IsGraph<P, T> for AdjacencyMatrix<N, T>{
    fn add_bidirectional_edge(&mut self: Self, i:usize, j:usize, val: T){
        self.matrix[i][j] = val;
        self.matrix[j][i] = val;
    }

    fn add_directional_edge(&mut self: Self, from:usize, to:usize, val: T){
        self.matrix[from][to] = val;
        self.is_directional = val;
    }

    fn new() -> Self {
        Self{
            matrix:[[false; N]; N],
            is_cyclical: false,
            is_directional: false,
        }
    }

    fn is_cyclical(&self) -> bool {
        todo!()
    }

    fn check_directional(&mut self){
        for i in 0..N{
            for j in 0..i{
                if self.matrix[i][j] != self.matrix[j][i]{
                    self.is_directional = true;
                    return;
                }
            }
        }
        self.is_directional = false;
    }

    fn get_targets(&self, node:usize) -> Vec<usize>{
        self.matrix[node].iter().enumerate().filter_map(|i: usize,target: bool| if *target { Some(i) } else { None }).collect()
    }

    fn get_sources(&self, node:usize)-> Vec<usize>{
        self.matrix.iter().enumerate().filter_map(|i: usize, source:&[bool;N]| if *source[node] { Some(i) } else { None }).collect()
    }

    fn remove_bidirectional_edge(&mut self, i:usize, j:usize) {
        self.matrix[i][j] = false;
        self.matrix[j][i] = false;
    }

    fn remove_directional_edge(&mut self, from: usize, to: usize) {
        self.matrix[from][to] = false;
        self.check_directional()
    }
}
