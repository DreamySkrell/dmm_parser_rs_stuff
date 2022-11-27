use crate::*;

pub fn flip() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        let original = std::fs::read_to_string(&path).unwrap();

        let mut parsed = parse(&original);
        println!(
            "-- -- flipping: {}",
            path.file_name().unwrap().to_string_lossy()
        );

        {
            for row in &mut parsed.rows {
                row.tiles.reverse();
            }

            for prototype in &mut parsed.prototypes {
                for atom in &mut prototype.atoms {
                    // // pixel shift
                    // if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                    //     *pixel_y = *pixel_y * -1.0;
                    //     println!("pixel shifted: {}", atom.path);
                    //     continue;
                    // }

                    // cables
                    if atom.path.contains("structure/cable") {
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "1-4" {
                                *icon_state = "2-4".into();
                                atom.vars["d1"] = VarVal::Int(2.0);
                            }
                            continue;
                        }
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "2-4" {
                                *icon_state = "1-4".into();
                                atom.vars["d1"] = VarVal::Int(1.0);
                            }
                            continue;
                        }

                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "1-8" {
                                *icon_state = "2-8".into();
                                atom.vars["d1"] = VarVal::Int(2.0);
                            }
                            continue;
                        }
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "2-8" {
                                *icon_state = "1-8".into();
                                atom.vars["d1"] = VarVal::Int(1.0);
                            }
                            continue;
                        }

                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "0-2" {
                                *icon_state = "0-1".into();
                                atom.vars["d2"] = VarVal::Int(1.0);
                            }
                            continue;
                        }
                        if let None = atom.vars.get_mut("icon_state") {
                            atom.vars
                                .insert("icon_state".into(), VarVal::String("0-2".into()));
                            atom.vars.insert("d2".into(), VarVal::Int(2.0));
                            continue;
                        }
                    }
                }
            }
        }

        let printed = print(&parsed);

        std::fs::write(
            format!("data_flip/{}", path.file_name().unwrap().to_str().unwrap()),
            printed,
        )
        .unwrap();
    }
}
