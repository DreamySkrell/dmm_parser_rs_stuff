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
        let origin_path: std::path::PathBuf = format!("{map_name}").into();
        println!("processing map: {}", origin_path.to_str().unwrap());
        // let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_p.dmm").into();
        let result_path: std::path::PathBuf = format!("{map_name}").into();

        let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
        let parsed = parse(&origin_map_str);

        let parsed_str = print(&parsed);
        // std::fs::write(parsed_path, parsed_str.clone()).unwrap();
        // for (i, diff) in diff::lines(&origin_map_str, &parsed_str).iter().enumerate() {
        //     match diff {
        //         diff::Result::Left(l) => println!("{} diff - : {}", i, l),
        //         diff::Result::Both(l, r) => {
        //             assert_eq!(l, r);
        //         }
        //         diff::Result::Right(r) => println!("{} diff + : {}", i, r),
        //     }
        // }
        // assert!(origin_map_str == parsed_str);

        let mut umm = unpack(&parsed);

        let rows = umm.grid.rows();
        let cols = umm.grid.cols();
        for row in 0..rows {
            for col in 0..cols {
                if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
                    continue;
                }

                let prototypes = umm.grid.get_mut(row, col).unwrap();

                // per atom changes
                for atom in prototypes.atoms.iter_mut() {
                    // floors
                    if ["/turf/unsimulated/floor"].iter().any(|x| *x == atom.path) {
                        if let Some(dmmr::VarVal::String(icon_state)) = atom.vars.get("icon_state")
                        {
                            let mut new_icon_state = None;
                            for (a, b) in
                                [("tiles", "tiled_preview"), ("steel_dirty", "tiled_preview")]
                            {
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
                }
            }
        }

        let repacked = pack(&umm);

        let result_str = print(&repacked);

        std::fs::write(result_path, result_str).unwrap();
    }
}
