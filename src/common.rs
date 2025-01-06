pub(crate) struct Ray<P, T: Ord + Copy> {
    pub(crate) point: P,
    pub(crate) edge: T,
}

impl<P, T: Ord + Copy> Ray<P, T> {
    pub fn new(point: P, edge: T) -> Self {
        Self { point, edge }
    }
}

pub(crate) trait IsGraph<P: Copy + Eq, T: Ord + Copy> {
    fn add_bidirectional_edge(&mut self, i: P, j: P, val: T);
    fn add_directional_edge(&mut self, from: P, to: P, val: T);

    fn new(nodes: Box<dyn Iterator<Item = P>>) -> Self;
    fn is_cyclical(&self) -> bool;

    fn check_directional(&mut self);

    fn get_targets(&self, node: P) -> Vec<Ray<P, T>>;

    fn get_sources(&self, node: P) -> Vec<Ray<P, T>>;

    fn remove_bidirectional_edge(&mut self, i: P, j: P);
    fn remove_directional_edge(&mut self, from: P, to: P);

    fn add_node(&mut self, node: P);

    fn get_nodes(&self) -> Vec<P>;
}
