
struct Ray<P,T: Ord+Copy>{
    point:P,
    edge: T
}

pub(crate) trait IsGraph<P, T: Ord+Copy>{
    fn add_bidirectional_edge(&mut self: Self, i:P, j:P, val: T);
    fn add_directional_edge(&mut self: Self, from:P, to:P, val:T);

    fn new() -> Self;
    fn is_cyclical(&self) -> bool;

    fn check_directional(&mut self);

    fn get_targets(&self, node: P) -> Vec<Ray<P,T>>;

    fn get_sources(&self, node:P)-> Vec<Ray<P,T>>;

    fn remove_bidirectional_edge(&mut self, i:P, j:P);
    fn remove_directional_edge(&mut self, from:P, to:P);
}