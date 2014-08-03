#![link(name="git2")]

extern crate git2;
use git2::git2;
use git2::git2::{OID, ToOID};

#[test]
fn test_blob() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/tests/data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let oid: OID = match "95d09f2b10159347eece71399a7e2e907ea3df4f".to_oid() {
        Ok(o) => o,
        Err(e) => fail!(e)
    };
    let obj = repo.lookup_blob(oid).unwrap();

    assert!(obj.rawsize() == 11);
    assert!(obj.rawcontent().as_slice() == "hello world".as_bytes());

    let owner = obj.owner();
    assert!(owner.path() == repo.path());

    let id = obj.id();
    assert!(id == oid);
 
    assert!(obj.is_binary() == false);
}
