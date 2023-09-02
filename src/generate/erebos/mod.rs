mod common;
mod crawler;
mod debug;
mod graph;
mod map;

use graph::MapGraph;
use map::room_templates;
use std::time::Instant;

pub fn random_graph() -> MapGraph {
  graph::random_graph()
}

pub fn generate_map(map_graph: &MapGraph, config: (i32, i32)) -> map::Map {
  let mut map = map::Map::new(config.0, config.1);
  let mut rng = rand::thread_rng();
  let mut templates = room_templates::RoomTemplates::new();

  for node in map_graph.nodes.iter() {
    let mut chain = Vec::from([node.clone()]);
    crawler::try_node_recursive(node, &map_graph, &mut map, &mut templates, &mut chain, &mut rng);
  }
  map
}
