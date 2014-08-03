#![link(name="git2")]

extern crate git2;
use git2::git2;
use std::io::TempDir;

#[test]
fn test_blob() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/src/libgit2-rs/test_data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let obj = repo.lookup_blob("95d09f2b10159347eece71399a7e2e907ea3df4f").unwrap();

    assert!(obj.rawsize() == 11);
    assert!(obj.rawcontent().as_slice() == "hello world".as_bytes());
}
