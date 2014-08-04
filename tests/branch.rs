#![link(name="git2")]

extern crate git2;
use std::io::TempDir;
use git2::git2;


#[test]
fn test_branch_lookup() {

    let dir =  TempDir::new("git2_test1").unwrap();
    println!("Cloning into {}", dir.path().display());

    let result = git2::clone("git://github.com/eminence/libgit2-rs.git", dir.path(), None);
    let repo = match result {
        Err(e) => fail!("Failed to clone! {}", e),
        Ok(r) => r
    };

    let refe = match repo.lookup_branch("master", git2::branch::GIT_BRANCH_LOCAL) {
        Ok(r) => r,
        Err(e) => fail!("Failed to look up branch! {}", e)
    };

    assert!(refe.is_some());
    let r = refe.unwrap();
    assert!(r.is_branch());
    println!("OID: {}", r.target().unwrap());
    println!("Name: {}", r.name());

}

#[test]
fn test_branch_iter() {

    let dir =  TempDir::new("git2_test1").unwrap();
    println!("Cloning into {}", dir.path().display());

    let result = git2::clone("git://github.com/eminence/libgit2-rs.git", dir.path(), None);
    let repo = match result {
        Err(e) => fail!("Failed to clone! {}", e),
        Ok(r) => r
    };

    for refe in repo.branch_iterator(git2::branch::GIT_BRANCH_LOCALREMOTE).unwrap() {
        println!("refe is {}", refe.name());
    }

}
