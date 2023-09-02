mod dungen;

use crate::dmmr::{self, pack, Atom, Prototype, Umm};
use grid::Grid;
use linked_hash_map::LinkedHashMap;
use rand::{thread_rng, Rng};

pub fn generate_1() {
    let dungeon_size = 50;
    let mut dungeon = dungen::Dungeon::new(dungeon_size, dungeon_size);
    let max_features = 10;
    dungeon.generate(max_features);
    // for y in 1..d.height {
    //     for x in 1..d.width {
    //         print!("{}", self._get_tile_icon(self.get_tile(x, y)));
    //     }
    //     println!("");
    // }

    let map_dir = "D:/Git/Aurora.3/maps/sccv_horizon".to_string();
    let map_name = map_dir + "/" + "dungeon" + ".dmm";

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
