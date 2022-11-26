use crate::*;

#[test]
fn flip() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        let original = std::fs::read_to_string(&path).unwrap();

        let mut parsed = parse(&original);

        {
            for row in &mut parsed.rows {
                row.tiles.reverse();
            }

            for prototype in &mut parsed.prototypes {
                for atom in &mut prototype.atoms {
                    // pixel shift
                    if let Some(VarVal::Int(pixel_y)) = atom.vars.get_mut("pixel_y") {
                        *pixel_y = *pixel_y * -1.0;
                    }

                    // cables
                    {
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "2-4" {
                                *icon_state = "1-4".into();
                                atom.vars["d1"] = VarVal::Int(1.0);
                            }
                        }
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "1-4" {
                                *icon_state = "2-4".into();
                                atom.vars["d1"] = VarVal::Int(2.0);
                            }
                        }
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "2-4" {
                                *icon_state = "1-4".into();
                                atom.vars["d1"] = VarVal::Int(2.0);
                            }
                        }
                        if let Some(VarVal::String(icon_state)) = atom.vars.get_mut("icon_state") {
                            if icon_state == "1-8" {
                                *icon_state = "2-8".into();
                                atom.vars["d1"] = VarVal::Int(2.0);
                            }
                        }
                    }
                }
            }
        }

        let printed = print(&parsed);

        std::fs::write(
            dbg!(format!(
                "data_flip/{}",
                path.file_name().unwrap().to_str().unwrap()
            )),
            printed,
        )
        .unwrap();
    }
}
