pub mod barbell_graph;
pub mod directed_binomial_tree_graph;
pub mod directed_gnp_random_graph;
pub mod directed_heavy_hex_graph;
pub mod directed_heavy_square_graph;
pub mod directed_hexagonal_lattice_graph;
pub mod generalized_petersen_graph;
pub mod random_matrix;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use rand::Rng;
use std::collections::HashMap;

pub type NeighbourMap = HashMap<usize, Vec<(NodeIndex<u32>, NodeIndex<u32>)>>;
pub type GraphResult = (StableGraph<usize, usize>, Vec<NodeIndex<u32>>);

#[derive(Debug)]
pub struct MapGraph {
  pub graph: StableGraph<usize, usize>,
  pub nodes: Vec<NodeIndex<u32>>,
  pub neighbour_map: NeighbourMap,
  pub node_forced_room_ids: HashMap<usize, usize>,
}

pub fn random_graph() -> MapGraph {
  let mut rng = rand::thread_rng();
  let selection: u32 = rng.gen_range(1..8);
  // let selection: u32 = 5;

  let (graph, nodes) = match selection {
    1 => {
      let mesh_nodes: usize = rng.gen_range(5..10);
      let path_nodes: usize = rng.gen_range(4..10);
      println!("barbell_graph - mesh_nodes: {} path_nodes: {}", mesh_nodes, path_nodes);
      barbell_graph::new(mesh_nodes, path_nodes)
    }
    2 => {
      let order: u32 = rng.gen_range(4..10);
      println!("directed_binomial_tree_graph - order:{}", order);
      directed_binomial_tree_graph::new(order, false)
    }
    3 => {
      let num_nodes: isize = rng.gen_range(8..30);
      let probability: f64 = rng.gen_range(0.1..0.3);
      println!("directed_gnp_random_graph");
      directed_gnp_random_graph::new(num_nodes, probability)
    }
    4 => {
      let mut distance: usize = rng.gen_range(3..10);

      if distance % 2 == 0 {
        distance -= 1; // force odd number
      }

      println!("directed_heavy_hex_graph");
      directed_heavy_hex_graph::new(distance, false)
    }
    5 => {
      let mut distance: usize = rng.gen_range(3..6);

      if distance % 2 == 0 {
        distance -= 1; // force odd number
      }

      println!("directed_heavy_square_graph");
      directed_heavy_square_graph::new(distance, false)
    }
    6 => {
      let rows: usize = rng.gen_range(4..6);
      let cols: usize = rng.gen_range(4..6);
      println!("directed_hexagonal_lattice_graph");
      directed_hexagonal_lattice_graph::new(rows, cols, false)
    }
    7 => {
      let shift: usize = rng.gen_range(3..6);
      let num: usize = shift * 2 + 1;
      println!("generalized_petersen_graph - nodes:{} shift:{}", num, shift);
      generalized_petersen_graph::new(num, shift)
    }
    _ => {
      println!("random_matrix");
      random_matrix::new()
    }
  };

  let neighbour_map = create_neighbour_map((graph.clone(), nodes.clone()));

  MapGraph {
    graph,
    nodes,
    neighbour_map,
    node_forced_room_ids: HashMap::new(),
  }
}

/// For each node, store the directional neighbours (incoming and outgoing)
pub fn create_neighbour_map((graph, nodes): GraphResult) -> NeighbourMap {
  let mut neighbour_map: NeighbourMap = HashMap::new();

  for node_a in nodes.iter() {
    for node_b in graph.neighbors(*node_a) {
      // A->B
      if let Some(neighbours) = neighbour_map.get_mut(&node_a.index()) {
        neighbours.push((*node_a, node_b));
      } else {
        neighbour_map.insert(node_a.index(), Vec::from([(*node_a, node_b)]));
      }

      // B->A
      if let Some(neighbours) = neighbour_map.get_mut(&node_b.index()) {
        neighbours.push((*node_a, node_b));
      } else {
        neighbour_map.insert(node_b.index(), Vec::from([(*node_a, node_b)]));
      }
    }
  }

  neighbour_map
}
