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

        for row in &mut parsed.rows {
            row.tiles.reverse();
        }

        for prototype in &mut parsed.prototypes {
            for atom in &mut prototype.atoms {
                // ----------------------------------------------
                // flags
                if atom.path.contains("flag/hegemony") {
                    if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                        *pixel_y = *pixel_y * -1.0;
                        continue;
                    }
                }

                // ----------------------------------------------
                // cables
                if atom.path.contains("structure/cable") {
                    if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                        if icon_state == "1-4" {
                            *icon_state = "2-4".into();
                            atom.vars["d1"] = VarVal::Int(2.0);
                            continue;
                        }
                    }
                    if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                        if icon_state == "2-4" {
                            *icon_state = "1-4".into();
                            atom.vars["d1"] = VarVal::Int(1.0);
                            continue;
                        }
                    }

                    if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                        if icon_state == "1-8" {
                            *icon_state = "2-8".into();
                            atom.vars["d1"] = VarVal::Int(2.0);
                            continue;
                        }
                    }
                    if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                        if icon_state == "2-8" {
                            *icon_state = "1-8".into();
                            atom.vars["d1"] = VarVal::Int(1.0);
                            continue;
                        }
                    }

                    if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                        if icon_state == "0-2" {
                            *icon_state = "0-1".into();
                            atom.vars["d2"] = VarVal::Int(1.0);
                            continue;
                        }
                    }
                    if let None = atom.vars.get_mut("icon_state") {
                        atom.vars
                            .insert("icon_state".into(), VarVal::String("0-2".into()));
                        atom.vars.insert("d2".into(), VarVal::Int(2.0));
                        continue;
                    }
                }

                // ----------------------------------------------
                // pipes
                if atom.path.contains("pipe/manifold") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }
                if atom.path.contains("pipe/simple") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 9.0 {
                            *dir = 10.0;
                            continue;
                        }
                        if *dir == 10.0 {
                            *dir = 9.0;
                            continue;
                        }
                        if *dir == 5.0 {
                            *dir = 6.0;
                            continue;
                        }
                        if *dir == 6.0 {
                            *dir = 5.0;
                            continue;
                        }
                    }
                }

                // ----------------------------------------------
                // vents/scrubbers
                if atom.path.contains("atmospherics/unary") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }

                // ----------------------------------------------
                // air alarms
                if atom.path.contains("machinery/alarm") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                    if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                        *pixel_y = *pixel_y * -1.0;
                        continue;
                    }
                }

                // ----------------------------------------------
                // APCs
                if atom.path.contains("machinery/power/apc") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                    if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                        *pixel_y = *pixel_y * -1.0;
                        continue;
                    }
                }

                // ----------------------------------------------
                // lights
                if atom.path.contains("machinery/light") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }

                // ----------------------------------------------
                // window panes
                if atom.path.contains("structure/window") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }

                // ----------------------------------------------
                // window doors
                if atom.path.contains("machinery/door/window") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }

                // ----------------------------------------------
                // colored tiles
                if atom.path.contains("structure/window") {
                    if let Some(VarVal::Int(dir)) = atom.vars.get_mut("dir") {
                        if *dir == 1.0 {
                            *dir = 2.0
                        }
                        continue;
                    }
                    if let None = atom.vars.get_mut("dir") {
                        atom.vars.insert("dir".into(), VarVal::Int(1.0));
                        continue;
                    }
                }

                // pixel shift
                // if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                //     *pixel_y = *pixel_y * -1.0;
                //     println!("pixel shifted: {}", atom.path);
                //     continue;
                // }

                // ----------------------------------------------
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
