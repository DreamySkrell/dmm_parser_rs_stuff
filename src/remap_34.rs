#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

pub fn remap() {
    let map_dir = "D:/Git/Aurora.3.kyres1/maps/sccv_horizon".to_string();
    let map_name = "sccv_horizon-2_deck_2";
    let origin_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();
    let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_p.dmm").into();
    let result_path: std::path::PathBuf = format!("{map_dir}/{map_name}_r.dmm").into();

    let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
    let mut parsed = parse(&origin_map_str);

    let parsed_str = print(&parsed);
    std::fs::write(parsed_path, parsed_str.clone()).unwrap();
    for (i, diff) in diff::lines(&origin_map_str, &parsed_str).iter().enumerate() {
        match diff {
            diff::Result::Left(l) => println!("{} diff - : {}", i, l),
            diff::Result::Both(l, r) => {
                assert_eq!(l, r);
                //println!("{} diff   : {}", i, l);
            }
            diff::Result::Right(r) => println!("{} diff + : {}", i, r),
        }
    }
    assert!(origin_map_str == parsed_str);

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
            let atom_cloned = atom.clone();
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
            let is_directional_subtype = ["/east", "/west", "/south", "/north"]
                .iter()
                .any(|dir| atom.path.ends_with(dir));

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
            // directional stuff
            if !is_directional_subtype {
                // ----------------------------------------------
                // fix up APCs
                {
                    if atom_cloned.path.starts_with("/obj/machinery/power/apc") {
                        if atom_cloned.vars.contains_key("dir") {
                            if matches!(atom_cloned.vars["dir"], VarVal::Int(8.0)) {
                                atom.path = format!("{}/west", atom.path);
                                atom.vars.remove("dir");
                                atom.vars.remove("pixel_x");
                                atom.vars.remove("name");
                            } else if matches!(atom_cloned.vars["dir"], VarVal::Int(4.0)) {
                                atom.path = format!("{}/east", atom.path);
                                atom.vars.remove("dir");
                                atom.vars.remove("pixel_x");
                                atom.vars.remove("name");
                            } else if matches!(atom_cloned.vars["dir"], VarVal::Int(1.0)) {
                                atom.path = format!("{}/north", atom.path);
                                atom.vars.remove("dir");
                                atom.vars.remove("pixel_y");
                                atom.vars.remove("name");
                            } else if matches!(atom_cloned.vars["dir"], VarVal::Int(2.0)) {
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
                } // fix up APCs

                // ----------------------------------------------
                // fix up air alarms
                {
                    let atom_cloned = atom.clone();

                    if atom_cloned.path.starts_with("/obj/machinery/alarm") {
                        if pixel_x.is_some() && pixel_x.unwrap() < 0.0f64 {
                            atom.path = format!("{}/west", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_x.is_some() && pixel_x.unwrap() > 0.0f64 {
                            atom.path = format!("{}/east", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() > 0.0f64 {
                            atom.path = format!("{}/north", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() < 0.0f64 {
                            atom.path = format!("{}/south", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        }
                    }
                } // fix up air alarms

                // ----------------------------------------------
                // fire extinguisher cabinets
                {
                    let atom_cloned = atom.clone();

                    if atom_cloned
                        .path
                        .starts_with("/obj/structure/extinguisher_cabinet")
                    {
                        if pixel_x.is_some() && pixel_x.unwrap() < 0.0f64 {
                            atom.path = format!("{}/west", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_x.is_some() && pixel_x.unwrap() > 0.0f64 {
                            atom.path = format!("{}/east", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() > 0.0f64 {
                            atom.path = format!("{}/north", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() < 0.0f64 {
                            atom.path = format!("{}/south", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        }
                    }
                } // fire extinguisher cabinets

                // ----------------------------------------------
                // fireaxe cabinets
                {
                    let atom_cloned = atom.clone();

                    if atom_cloned
                        .path
                        .starts_with("/obj/structure/fireaxecabinet")
                    {
                        if pixel_x.is_some() && pixel_x.unwrap() < 0.0f64 {
                            atom.path = format!("{}/west", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_x.is_some() && pixel_x.unwrap() > 0.0f64 {
                            atom.path = format!("{}/east", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_x");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() > 0.0f64 {
                            atom.path = format!("{}/north", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        } else if pixel_y.is_some() && pixel_y.unwrap() < 0.0f64 {
                            atom.path = format!("{}/south", atom.path);
                            atom.vars.remove("dir");
                            atom.vars.remove("pixel_y");
                            atom.vars.remove("name");
                        }
                    }
                } // fireaxe cabinets
            } // end of directional stuff
        } // for atom in &mut prototype.atoms
    } // for prototype in &mut parsed.prototypes

    let result_str = print(&parsed);

    std::fs::write(result_path, result_str).unwrap();
}
