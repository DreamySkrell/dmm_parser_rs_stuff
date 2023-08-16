#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::*;

pub fn remap() {
    let dir = "D:/Git/Aurora.3.kyres1/maps/sccv_horizon".to_string();
    let origin_path: std::path::PathBuf = format!("{dir}/sccv_horizon-3_deck_3.dmm").into();
    let result_path: std::path::PathBuf = format!("{dir}/sccv_horizon-3_deck_3_z.dmm").into();

    let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
    let mut parsed = parse(&origin_map_str);

    for prototype in &mut parsed.prototypes {
        let atoms_cloned = prototype.atoms.clone();

        let has_dark_tile = atoms_cloned.iter().any(|atom| {
            [
                "/turf/simulated/floor/tiled",
                "/turf/simulated/floor/tiled/full",
                "/turf/simulated/floor/tiled/dark",
                "/turf/simulated/floor/tiled/dark/full",
            ]
            .iter()
            .any(|p| p == &atom.path)
        });

        for atom in &mut prototype.atoms {
            // ----------------------------------------------
            // replace light floor decals with dark
            // on dark floor tiles
            {
                if has_dark_tile {
                    if atom
                        .path
                        .starts_with("/obj/effect/floor_decal/corner/green")
                    {
                        atom.path = atom.path.replace("green", "dark_green");
                    }
                    if atom.path.starts_with("/obj/effect/floor_decal/corner/blue") {
                        atom.path = atom.path.replace("blue", "dark_blue");
                    }
                }
            }

            // ----------------------------------------------
            // fix up APCs
            {
                let atom_cloned = atom.clone();
                if atom_cloned.path.starts_with("/obj/machinery/power/apc") {
                    if atom_cloned.vars.contains_key("dir") {
                        if matches!(atom_cloned.vars["dir"], VarVal::Int(8.0)) {
                            atom.path = format!("{}/west", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        }

                        if matches!(atom_cloned.vars["dir"], VarVal::Int(4.0)) {
                            atom.path = format!("{}/east", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        }

                        if matches!(atom_cloned.vars["dir"], VarVal::Int(1.0)) {
                            atom.path = format!("{}/north", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        }

                        if matches!(atom_cloned.vars["dir"], VarVal::Int(2.0)) {
                            atom.path = format!("{}/south", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        }
                    } else {
                        atom.path = format!("{}/south", atom.path);
                        atom.vars.remove("dir");
                        atom.vars.remove("pixel_y");
                        atom.vars.remove("name");
                    }
                }
            }

            // ----------------------------------------------
            // fix up air alarms
            {
                let atom_cloned = atom.clone();

                if atom_cloned.path.starts_with("/obj/machinery/alarm") {
                    let pixel_x = if atom_cloned.vars.contains_key("pixel_x") {
                        if let VarVal::Int(pixel_x) = atom_cloned.vars["pixel_x"] {
                            Some(pixel_x)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let pixel_y = if atom_cloned.vars.contains_key("pixel_y") {
                        if let VarVal::Int(pixel_y) = atom_cloned.vars["pixel_y"] {
                            Some(pixel_y)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if pixel_x.is_some() && pixel_x.unwrap() < 0.0f64 {
                        atom.path = format!("{}/west", atom.path);
                        atom.vars.remove("dir");
                        atom.vars.remove("pixel_x");
                        atom.vars.remove("name");
                    }

                    // if matches!(atom_cloned.vars["dir"], VarVal::Int(4.0)) {
                    //     atom.path = format!("{}/east", atom.path);
                    //     atom.vars.remove("dir");
                    //     atom.vars.remove("pixel_x");
                    //     atom.vars.remove("name");
                    // }

                    // if matches!(atom_cloned.vars["dir"], VarVal::Int(1.0)) {
                    //     atom.path = format!("{}/north", atom.path);
                    //     atom.vars.remove("dir");
                    //     atom.vars.remove("pixel_y");
                    //     atom.vars.remove("name");
                    // }

                    // if matches!(atom_cloned.vars["dir"], VarVal::Int(2.0)) {
                    //     atom.path = format!("{}/south", atom.path);
                    //     atom.vars.remove("dir");
                    //     atom.vars.remove("pixel_y");
                    //     atom.vars.remove("name");
                    // }
                }
            }
        }
    }

    let printed = print(&parsed);

    std::fs::write(result_path, printed).unwrap();
}
