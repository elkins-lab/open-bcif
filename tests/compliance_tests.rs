use std::path::Path;

#[test]
fn test_real_pdb_1crn() {
    let path = Path::new("tests/data/1crn.bcif");
    if path.exists() {
        let res = open_bcif::commands::validate::validate(path.to_str().unwrap());
        assert!(res.is_ok(), "Validation failed for 1CRN: {:?}", res.err());
    } else {
        println!("Skipping 1CRN test: file not found (tests/data/1crn.bcif)");
    }
}

#[test]
fn test_real_pdb_1ubq() {
    let path = Path::new("tests/data/1ubq.bcif");
    if path.exists() {
        let res = open_bcif::commands::validate::validate(path.to_str().unwrap());
        assert!(res.is_ok(), "Validation failed for 1UBQ: {:?}", res.err());
    } else {
        println!("Skipping 1UBQ test: file not found (tests/data/1ubq.bcif)");
    }
}
