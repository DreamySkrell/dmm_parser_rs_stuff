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
            let left = &original;
            let right = &printed;

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
        }

        if original != printed {
            panic!();
        }

        // assert_eq!(original, printed);
        println!("   ok");
    }
}
