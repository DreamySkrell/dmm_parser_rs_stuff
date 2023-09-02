#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use walkdir::WalkDir;

use crate::dmmr::*;
use crate::*;

pub fn remap() {
    let map_dir = "D:/Git/Aurora.3/maps".to_string();

    for entry in WalkDir::new(map_dir.clone())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".dmm"))
    {
        let map_name = entry.path().to_str().unwrap(); //"sccv_horizon-2_deck_2";

        if ["multi_zas_test"].iter().any(|x| map_name.contains(x)) {
            continue;
        }
        if !["horizon", "deck"].iter().all(|x| map_name.contains(x)) {
            continue;
        }

        let origin_path: std::path::PathBuf = format!("{map_name}").into();
        println!("processing map: {}", origin_path.to_str().unwrap());
        // let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_p.dmm").into();
        let result_path: std::path::PathBuf = format!("{map_name}").into();

        let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
        let parsed = parse(&origin_map_str);

        let parsed_str = print(&parsed);
        // std::fs::write(parsed_path, parsed_str.clone()).unwrap();
        for (i, diff) in diff::lines(&origin_map_str, &parsed_str).iter().enumerate() {
            match diff {
                diff::Result::Left(l) => println!("{} diff - : {}", i, l),
                diff::Result::Both(l, r) => {
                    assert_eq!(l, r);
                }
                diff::Result::Right(r) => println!("{} diff + : {}", i, r),
            }
        }
        assert!(origin_map_str == parsed_str);

        let mut umm = unpack(&parsed);

        let rows = umm.grid.rows();
        let cols = umm.grid.cols();
        for row in 0..rows {
            for col in 0..cols {
                if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
                    continue;
                }

                let prototypes = umm.grid.get_mut(row, col).unwrap();

                let atoms_cloned = prototypes.atoms.clone();

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

                // per atom changes
                for atom in prototypes.atoms.iter_mut() {
                    // floors
                    if ["/turf/unsimulated/floor"].iter().any(|x| *x == atom.path) {
                        if let Some(dmmr::VarVal::String(icon_state)) = atom.vars.get("icon_state")
                        {
                            let mut new_icon_state = None;
                            for (a, b) in [
                                ("tiles", "tiled_preview"),
                                ("steel_dirty", "tiled_preview"),
                                ("platingdmg1", "tiled_preview"),
                                ("platingdmg2", "tiled_preview"),
                                ("platingdmg3", "tiled_preview"),
                                ("rampbottom", "tiled_preview"),
                                ("carpet5-1", "tiled_preview"),
                                ("floorscorched1", "tiled_preview"),
                                ("new_reinforced", "tiled_preview"),
                                ("bar", "tiled_preview"),
                                ("dark2", "dark_preview"),
                                ("wood_light", "wood_preview"),
                                ("carpet13-5", "carpet"),
                                ("lino_grey", "tiled_preview"),
                                ("brown", "tiled_preview"),
                                ("platingdmg2", "tiled_preview"),
                                ("freezerfloor", "freezer"),
                                ("new_white", "white"),
                                ("carpetside", "carpet"),
                                ("carpet6-2", "carpet"),
                                ("carpetnoconnect", "carpet"),
                                ("wood-broken", "wood_preview"),
                                ("cafeteria", "wood_preview"),
                                ("floor", "tiled_preview"),
                                ("floor3", "tiled_preview"),
                                ("engine", "tiled_preview"),
                                ("platebot", "tiled_preview"),
                            ] {
                                if icon_state == a {
                                    new_icon_state = Some(b);
                                }
                            }
                            if new_icon_state.is_some() {
                                atom.vars.insert(
                                    "icon_state".to_string(),
                                    dmmr::VarVal::String(new_icon_state.unwrap().into()),
                                );
                            }
                        }
                    }

                    // light
                    if ["/obj/machinery/light"].iter().any(|x| *x == atom.path) {
                        if let Some(dmmr::VarVal::String(icon_state)) = atom.vars.get("icon_state")
                        {
                            let mut new_icon_state = None;
                            for (a, b) in [("tube1", "tube_empty")] {
                                if icon_state == a {
                                    new_icon_state = Some(b);
                                }
                            }
                            if new_icon_state.is_some() {
                                atom.vars.insert(
                                    "icon_state".to_string(),
                                    dmmr::VarVal::String(new_icon_state.unwrap().into()),
                                );
                            }
                        }
                    }

                    // various fixes from remap_34
                    {
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
                                if atom
                                    .path
                                    .starts_with("/obj/effect/floor_decal/corner/paleblue")
                                {
                                    atom.path = atom.path.replace("paleblue", "dark_blue");
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
                                        } else if matches!(
                                            atom_cloned.vars["dir"],
                                            VarVal::Int(4.0)
                                        ) {
                                            atom.path = format!("{}/east", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_x");
                                            atom.vars.remove("name");
                                        } else if matches!(
                                            atom_cloned.vars["dir"],
                                            VarVal::Int(1.0)
                                        ) {
                                            atom.path = format!("{}/north", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_y");
                                            atom.vars.remove("name");
                                        } else if matches!(
                                            atom_cloned.vars["dir"],
                                            VarVal::Int(2.0)
                                        ) {
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
                            // fix up a whole lot of stuff
                            {
                                let blacklist = ["/obj/machinery/ringer_button"];
                                for fixable in [
                                    "/obj/machinery/alarm",
                                    "/obj/structure/extinguisher_cabinet",
                                    "/obj/structure/fireaxecabinet",
                                    "/obj/item/device/radio/intercom",
                                    "/obj/machinery/ringer",
                                    "/obj/machinery/requests_console",
                                    "/obj/machinery/newscaster",
                                ] {
                                    let atom_cloned = atom.clone();

                                    if blacklist.iter().any(|a| *a == atom_cloned.path) {
                                        continue;
                                    }

                                    if atom_cloned.path.starts_with(fixable) {
                                        if pixel_x.is_some() && pixel_x.unwrap() < 0.0f64 {
                                            atom.path = format!("{}/west", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_x");
                                        } else if pixel_x.is_some() && pixel_x.unwrap() > 0.0f64 {
                                            atom.path = format!("{}/east", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_x");
                                        } else if pixel_y.is_some() && pixel_y.unwrap() > 0.0f64 {
                                            atom.path = format!("{}/north", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_y");
                                        } else if pixel_y.is_some() && pixel_y.unwrap() < 0.0f64 {
                                            atom.path = format!("{}/south", atom.path);
                                            atom.vars.remove("dir");
                                            atom.vars.remove("pixel_y");
                                        }
                                    }
                                }
                            } // fix up a whole lot of stuff
                        } // end of directional stuff
                    }
                }

                // door dirs
                {
                    let prototypes_cloned = umm.grid.get(row, col).unwrap().clone();

                    let is_turf_solid = |a: &Atom| {
                        a.path.starts_with("/obj/machinery/door/airlock")
                            || a.path.starts_with("/obj/machinery/door/firedoor")
                            || a.path.starts_with("/obj/effect/map_effect/window_spawner")
                            || a.path.starts_with("/obj/structure/grille")
                            || a.path.starts_with("/turf/simulated/wall")
                    };
                    let north_solid = umm
                        .grid
                        .get(row + 1, col)
                        .unwrap()
                        .atoms
                        .iter()
                        .any(is_turf_solid);
                    let south_solid = umm
                        .grid
                        .get(row - 1, col)
                        .unwrap()
                        .atoms
                        .iter()
                        .any(is_turf_solid);
                    let east_solid = umm
                        .grid
                        .get(row, col + 1)
                        .unwrap()
                        .atoms
                        .iter()
                        .any(is_turf_solid);
                    let west_solid = umm
                        .grid
                        .get(row, col - 1)
                        .unwrap()
                        .atoms
                        .iter()
                        .any(is_turf_solid);

                    let prototypes = umm.grid.get_mut(row, col).unwrap();

                    for atom in prototypes.atoms.iter_mut() {
                        if atom.path.starts_with("/obj/machinery/door/airlock")
                            && !atom.path.contains("multi_tile")
                        {
                            if east_solid || west_solid {
                                atom.vars
                                    .insert("dir".to_string(), dmmr::VarVal::Int(4.0f64));
                            } else if north_solid || south_solid {
                                atom.vars
                                    .insert("dir".to_string(), dmmr::VarVal::Int(1.0f64));
                            }
                        }
                    }
                }

                // add firedoor to doors that don't have one
                // {
                //     let airlock = atoms_cloned.iter().find(|a| {
                //         a.path.starts_with("/obj/machinery/door/airlock")
                //             && !a.path.contains("/external")
                //     });
                //     let no_firedoor = !atoms_cloned
                //         .iter()
                //         .any(|a| a.path.starts_with("/obj/machinery/door/firedoor"));
                //     if airlock.is_some() && no_firedoor {
                //         let mut map = LinkedHashMap::new();
                //         map.insert("dir".to_string(), dmmr::VarVal::Int(4.0f64));
                //         prototypes.atoms.push(Atom {
                //             path: "/obj/machinery/door/firedoor".into(),
                //             vars: map,
                //         });
                //     }
                // }
            }
        }

        let repacked = pack(&umm);

        let result_str = print(&repacked);

        std::fs::write(result_path, result_str).unwrap();
    }
}
