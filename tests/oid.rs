#![link(name="git2")]

extern crate git2;
use git2::git2;
use git2::git2::{OID, ToOID};
use std::io::TempDir;

#[test]
fn test_oid() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/tests/data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let oid: OID = match "95d09f2b10159347eece71399a7e2e907ea3df4f".to_oid() {
        Ok(o) => o,
        Err(e) => fail!(e)
    };

    println!("OID is {}", oid);
    assert!(oid.to_string().as_slice() == "95d09f2b10159347eece71399a7e2e907ea3df4f");

}
