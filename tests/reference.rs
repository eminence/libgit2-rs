#![link(name="git2")]

extern crate git2;
use git2::git2;
use std::io::TempDir;

#[test]
fn test_lookup_reference() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/src/libgit2-rs/test_data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);
    let refe = match repo.lookup_reference("refs/heads/master") {
        Ok(r) => r,
        Err(e) => fail!("Failed to get reference:\n{}", e.message)
    };
    assert!(refe.is_remote() == false);
    assert!(refe.is_branch() == true);
    assert!(refe.get_type() == git2::reference::GIT_REF_OID);
    
    if repo.lookup_reference("refs/heads/noexist").is_ok() {
        fail!("Found repo, but wasn't supposed to");
    }
}
