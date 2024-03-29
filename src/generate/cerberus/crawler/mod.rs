use super::{graph, map};
use map::{
  room,
  room::{Room, RoomType},
  room_templates,
  room_templates::RoomTemplates,
};
use petgraph::graph::NodeIndex;
use rand::prelude::{SliceRandom, ThreadRng};

pub fn try_node_recursive(
  node_a: &NodeIndex<u32>,
  map_graph: &graph::MapGraph,
  map: &mut map::Map,
  templates: &mut RoomTemplates,
  chain: &mut Vec<NodeIndex>,
  rng: &mut ThreadRng,
) {
  let graph::MapGraph {
    graph,
    nodes,
    neighbour_map,
    node_forced_room_ids,
  } = map_graph;

  // debug::print_map(map.tiles.clone(), map.width);

  // Should only create on first node
  let mut room_a = map::find_or_create_start_room(map, templates, &node_a.index());
  let neighbours = room::get_neighbours(&node_a.index(), neighbour_map);
  let is_detached = chain.len() == 1 && map.rooms.len() > 0 && !map.rooms.contains_key(&node_a.index());

  let weights = graph.node_weights().collect::<Vec<&usize>>();
  // let _weight = weights.get(node_a.index()).unwrap();

  // Place first room in the middle of the map
  if nodes.first().unwrap().index() == node_a.index() {
    let (centre_x, centre_y) = map::centre_coordinates(&mut room_a, map);
    room_a.x = centre_x;
    room_a.y = centre_y;
    map::add_or_update_room(map, node_a.index(), room_a.clone());
  }

  // Sometimes nodes are not yet connected to the map, this attempts existing neighbours
  if is_detached {
    println!("Room detached: {}, finding existing combination..", node_a.index());
    for (_, node_b, _) in neighbours.clone() {
      let is_existing_node = map.rooms.contains_key(&node_b.index());

      if is_existing_node {
        println!("Trying existing node: {}", &node_b.index());
        let mut chain = Vec::from([node_b.clone()]);
        try_node_recursive(&node_b, map_graph, map, templates, &mut chain, rng);
      }
    }
  }

  // For each node
  for (_, node_b, outbound) in &neighbours {
    let node_b_weight = weights.get(node_b.index()).unwrap();
    let room_b_type = if node_b_weight == weights.last().unwrap() {
      //RoomType::Boss
      RoomType::Normal
    } else {
      RoomType::Normal
    };

    // Stop at an existing node (TODO: try connect rooms?)
    let is_existing_node = map.rooms.contains_key(&node_b.index());

    let mut room_added = false;
    let mut template_idxs: Vec<usize> = room_a.template.valid_combinations.keys().cloned().collect();

    // randomise templates
    template_idxs.shuffle(rng);

    // Loop through randomised rooms until we find one that can fit into the map
    while !template_idxs.is_empty() && !room_added && !is_existing_node && !is_detached {
      let template_b_idx = template_idxs.pop().unwrap();
      let mut room_b = Room::new(room_templates::get(&template_b_idx, templates));

      // Prefer rooms that are designed to have minimum doors (eg: T shaped rooms = 3 minimum doors)
      let has_min_doors = room_b.template.min_doors <= (graph.edges(node_b.clone()).count() + 1) as u32;
      // Select Boss or Normal room based on node weight
      let room_is_correct_type = {
        //
        //(room_b.template.unique_id.is_none() || room_b.template.unique_id.unwrap() == node_b.index());
        // if true {};
        if let Some(node_forced_room_id) = node_forced_room_ids.get(&node_b.index()) {
          if room_b.template.tag.is_none() {
            false
          } else {
            let unique_id = room_b.template.tag.unwrap();
            unique_id == *node_forced_room_id
          }
        } else {
          room_b.template.room_type == room_b_type
        }
      };

      // Each room has a precalculated coordinates
      let mut room_combinations = if room_is_correct_type && has_min_doors {
        room_a.template.valid_combinations.get(&template_b_idx).unwrap().clone()
      } else {
        Vec::new()
      };

      room_combinations.shuffle(rng);

      for combination in room_combinations {
        let (_, door_a_type, door_a_xy, door_b_type, door_b_xy) = combination;
        let mut room_b_aligned = room::align_room_b(&room_a, &mut room_b, combination);

        if map::can_place_room(map, &room_b_aligned, door_b_type) {
          // Add door references to room
          // TODO: Traverse all existing rooms and attempt to optimise for more connections and more rooms?
          room_a.add_door(node_a.index(), node_b.index(), door_a_type, door_a_xy, *outbound);
          room_b_aligned.add_door(node_b.index(), node_a.index(), door_b_type, door_b_xy, !outbound);

          // update rooms with the new door
          map::add_or_update_room(map, node_b.index().clone(), room_b_aligned.clone());
          map::add_or_update_room(map, node_a.index(), room_a.clone());
          room_added = true;
          break;
        }
      }
    }

    // if this random room can be placed, try this room's connecting nodes
    if room_added {
      chain.push(node_b.clone());
      try_node_recursive(&node_b, map_graph, map, templates, chain, rng);
    }
  }
}
