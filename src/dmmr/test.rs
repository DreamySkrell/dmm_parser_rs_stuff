use crate::dmmr::*;
use crate::*;

#[test]
fn sanity() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let _ = std::fs::read_to_string(dmm.unwrap().path()).unwrap();
    }
}

#[test]
fn parse_compare() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        println!("-- {}", path.to_str().unwrap());
        let original = std::fs::read_to_string(path).unwrap();
        println!("   read");
        let parsed = parse(&original);
        println!("   parsed");
        let printed = print(&parsed);
        println!("   printed");

        // println!("{}", original);
        // println!("{}", printed);

        if original != printed {
            let left = &original.clone().replace("\r\n", "\n");
            let right = &original.clone().replace("\r\n", "\n");

            for (i, diff) in diff::lines(left, right).iter().enumerate() {
                match diff {
                    diff::Result::Left(l) => println!("{} diff - : {}", i, l),
                    diff::Result::Both(l, r) => {
                        assert_eq!(l, r);
                        //println!("{} diff   : {}", i, l);
                    }
                    diff::Result::Right(r) => println!("{} diff + : {}", i, r),
                }
            }
            assert_eq!(left, right);
        }

        println!("   ok");
    }
}

#[test]
fn unpack_repack() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        println!("-- {}", path.to_str().unwrap());
        let original = std::fs::read_to_string(path.clone()).unwrap();
        println!("   read");
        let parsed = parse(&original);
        println!("   parsed");
        let unpacked = unpack(&parsed);
        println!("   unpacked");
        let repacked = pack(&unpacked);
        println!("   repacked");
        let printed = print(&repacked);
        println!("   printed");

        if original != printed && false {
            let left = &original.clone().replace("\r\n", "\n");
            let right = &printed.clone().replace("\r\n", "\n");

            // println!("{}", left);
            // println!("{}", right);

            for (i, diff) in diff::lines(left, right).iter().enumerate() {
                match diff {
                    diff::Result::Left(l) => println!("{} diff - : {}", i, l),
                    diff::Result::Both(l, r) => {
                        assert_eq!(l, r);
                        //println!("{} diff   : {}", i, l);
                    }
                    diff::Result::Right(r) => println!("{} diff + : {}", i, r),
                }
            }
            // assert_eq!(left, right);
        }

        // std::fs::write(path, printed).unwrap();

        println!("   ok");
    }
}

#[test]
fn ordering() {
    assert!("a" < "b");
    assert!("a" > "B");
}
