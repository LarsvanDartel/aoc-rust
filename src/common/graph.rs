use std::collections::VecDeque;
use std::iter;

use hashbrown::HashMap;
use rustworkx_core::petgraph::graphmap::NodeTrait;
pub use rustworkx_core::petgraph::prelude::*;

pub fn iter_cliques<N: NodeTrait, E>(g: &UnGraphMap<N, E>) -> impl Iterator<Item = Vec<N>> + '_ {
    let mut index = HashMap::new();
    let mut neighbors = HashMap::new();

    for node in g.nodes() {
        index.insert(node, index.len());
        neighbors.insert(
            node,
            g.neighbors(node)
                .filter(|n| !index.contains_key(n))
                .collect::<Vec<_>>(),
        );
    }

    let mut q = VecDeque::new();
    for node in g.nodes() {
        let mut neighbors = neighbors[&node].clone();
        neighbors.sort_by_key(|n| index[n]);
        q.push_back((vec![node], neighbors));
    }

    iter::from_fn(move || {
        let (base, common_neighbors) = q.pop_front()?;
        for (i, &node) in common_neighbors.iter().enumerate() {
            let mut new_base = base.clone();
            new_base.push(node);
            let mut new_common_neighbors = common_neighbors[i + 1..].to_vec();
            new_common_neighbors.retain(|n| g.neighbors(node).any(|m| m == *n));
            new_common_neighbors.sort_by_key(|n| index[n]);
            q.push_back((new_base, new_common_neighbors));
        }
        Some(base)
    })
}
