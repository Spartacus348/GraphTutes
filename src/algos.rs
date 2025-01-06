use crate::common::{IsGraph, Ray};

pub(crate) fn reverse_graph<P: Copy + Eq + 'static, T: Ord + Copy, G: IsGraph<P, T>>(
    target: G,
) -> G {
    let nodes = target.get_nodes();
    let mut reversed = G::new(Box::new(nodes.clone().into_iter()));
    for node in reversed.get_nodes() {
        for Ray {
            point: new_source,
            edge,
        } in target.get_targets(node)
        {
            reversed.add_directional_edge(new_source, node, edge);
        }
    }
    reversed
}
