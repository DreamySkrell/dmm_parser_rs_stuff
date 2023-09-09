#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod cerberus;
mod dungen;

use std::{collections::HashMap, ops::Deref};

use crate::dmmr::{self, pack, print, Atom, Prototype, Umm};
use grid::Grid;
use linked_hash_map::LinkedHashMap;
use petgraph::{
    stable_graph::{NodeIndex, StableGraph},
    visit::{EdgeRef, IntoEdgeReferences},
};
use rand::{thread_rng, Rng};

pub fn generate_dungen() {
    let dungeon_size = 100;
    let mut dungeon = dungen::Dungeon::new(dungeon_size, dungeon_size);
    let max_features = 10;
    dungeon.generate(max_features);

    let map_dir = "D:/Git/Aurora.3/maps/sccv_horizon".to_string();
    let map_name = map_dir + "/" + "dungeon_dungen" + ".dmm";

    let mut umm = Umm {
        comment:
            "//MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE"
                .into(),
        grid: Grid::new(dungeon_size as usize, dungeon_size as usize),
    };

    for x in 0..dungeon_size {
        for y in 0..dungeon_size {
            //
            //dungeon
            let dungen_tile = dungeon.get_tile(x, y);
            let umm_tile = umm.grid.get_mut(x as usize, y as usize).unwrap();
            umm_tile.id = "aaa".into();

            match dungen_tile {
                dungen::Tile::Unused => {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/template_noop".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::Floor => {
                    if rand::thread_rng().gen::<bool>() {
                        umm_tile.atoms.push(Atom {
                            path: "/turf/simulated/floor/tiled".into(),
                            vars: LinkedHashMap::new(),
                        });
                    } else {
                        umm_tile.atoms.push(Atom {
                            path: "/turf/simulated/floor/tiled/full".into(),
                            vars: LinkedHashMap::new(),
                        });
                    }
                }
                dungen::Tile::Corridor => {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/floor/plating".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::Wall => {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/wall".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::ClosedDoor => {
                    umm_tile.atoms.push(Atom {
                        path: "/obj/machinery/door/airlock/maintenance".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::OpenDoor => {
                    umm_tile.atoms.push(Atom {
                        path: "/obj/machinery/door/airlock/maintenance".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::Exit => {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/floor/tiled".into(),
                        vars: LinkedHashMap::new(),
                    });
                    umm_tile.atoms.push(Atom {
                        path: "/obj/effect/landmark".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
                dungen::Tile::Entrance => {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/floor/tiled".into(),
                        vars: LinkedHashMap::new(),
                    });
                    umm_tile.atoms.push(Atom {
                        path: "/obj/effect/shuttle_landmark".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
            };
        }
    }

    let repacked = dmmr::pack(&umm);
    let result_str = dmmr::print(&repacked);
    std::fs::write(map_name, result_str).unwrap();
}

fn make_graph() -> cerberus::graph::MapGraph {
    let mut graph = StableGraph::from_edges([
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 8),
        (8, 16),
        (16, 17),
        (17, 18),
        (18, 19),
        //
        (5, 9),
        (9, 10),
        (10, 11),
        (11, 12),
        //
        (0, 13),
        (13, 14),
        (14, 15),
        //
        (5, 20),
        (20, 21),
        //
        // (2, 22),
        // (4, 23),
        // (9, 24),
        // (13, 25),
        // (17, 26),
        // (6, 27),
        // //
        // (5, 28),
        // (5, 29),
        // (5, 30),
    ]);
    for (i, weight) in graph.node_weights_mut().enumerate() {
        *weight = i + 1;
    }
    let nodes: Vec<NodeIndex<u32>> = graph.node_indices().collect();
    let neighbour_map = cerberus::graph::create_neighbour_map((graph.clone(), nodes.clone()));
    cerberus::graph::MapGraph {
        graph,
        nodes,
        neighbour_map,
        node_forced_room_ids: HashMap::from([(5, 42), (19, 43)]),
    }
}

pub fn generate_cerberus() {
    let dungeon_size = 100;
    // let map_graph = cerberus::random_graph();
    let map_graph = make_graph();
    let mut dungeon = loop {
        let dungeon =
            cerberus::generate_map(&map_graph, (dungeon_size as i32, dungeon_size as i32));
        let edges_count = map_graph.graph.edge_count();
        let edges_satisfied = {
            map_graph
                .graph
                .edge_references()
                .map(|edge| (edge.source(), edge.target()))
                .map(|(a, b)| (a.index(), b.index()))
                .filter(|(a1, b1)| {
                    let room = dungeon.rooms.get(a1);
                    if room.is_none() {
                        false
                    } else {
                        room.unwrap()
                            .door_connections
                            .iter()
                            .map(|x| x.1.clone())
                            .map(|x| x)
                            .flatten()
                            .map(|d| (d.node_a_idx, d.node_b_idx))
                            .any(|(a2, b2)| (*a1 == a2 && *b1 == b2) || (*a1 == b2 && *b1 == a2))
                    }
                })
                .count()
        };
        dbg!(dungeon.rooms.len(), map_graph.nodes.len(),);
        if dungeon.rooms.len() >= map_graph.nodes.len() - 1 && edges_satisfied == edges_count {
            break dungeon;
        }
        println!("-------------------");
        println!("regenerating map...");
    };

    let map_dir = "D:/Git/Aurora.3/maps/sccv_horizon".to_string();
    let map_name = map_dir + "/" + "dungeon_cerberus" + ".dmm";

    let mut umm = Umm {
        comment:
            "//MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE"
                .into(),
        grid: Grid::new(dungeon_size as usize, dungeon_size as usize),
    };

    for x in 0..dungeon_size {
        for y in 0..dungeon_size {
            let ere_tile = dungeon.tiles[x + y * dungeon_size];
            let umm_tile = umm.grid.get_mut(x as usize, y as usize).unwrap();
            umm_tile.id = "aaa".into();

            if ere_tile == 0 {
                // SPACE: grey
                umm_tile.atoms.push(Atom {
                    path: "/turf/template_noop".into(),
                    vars: LinkedHashMap::new(),
                });
            } else if ere_tile == 9 {
                // CONFLICT: red
                umm_tile.atoms.push(Atom {
                    path: "/turf/simulated/floor/tiled".into(),
                    vars: LinkedHashMap::new(),
                });
                umm_tile.atoms.push(Atom {
                    path: "/obj/effect/shuttle_landmark".into(),
                    vars: LinkedHashMap::new(),
                });
            } else if ere_tile == 8 {
                // BG: black
                if rand::thread_rng().gen::<bool>() {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/floor/tiled".into(),
                        vars: LinkedHashMap::new(),
                    });
                } else {
                    umm_tile.atoms.push(Atom {
                        path: "/turf/simulated/floor/tiled/full".into(),
                        vars: LinkedHashMap::new(),
                    });
                }
            } else if ere_tile == 7 {
                // DOOR: blue
                umm_tile.atoms.push(Atom {
                    path: "/obj/machinery/door/airlock/maintenance".into(),
                    vars: LinkedHashMap::new(),
                });
            } else {
                // WALL: grey white
                umm_tile.atoms.push(Atom {
                    path: "/turf/simulated/wall".into(),
                    vars: LinkedHashMap::new(),
                });
            }
        }
    }

    let repacked = dmmr::pack(&umm);
    let result_str = dmmr::print(&repacked);
    std::fs::write(map_name, result_str).unwrap();
}
