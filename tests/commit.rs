#![link(name="git2")]

extern crate git2;
use git2::git2;

#[test]
fn test_commit() {
    let repo = match git2::Repository::open(&Path::new("/storage/home/achin/devel/libgit2.rs/tests/data/repoA")) {
        Ok(r) => r,
        Err(e) => fail!("Failed to open repo:\n{}", e.message)
    };
    assert!(repo.is_empty() == false);

    let obj = repo.lookup_commit("b398025e497485e38f6b5c6a1d8d6de534642c20").unwrap();
    println!("Commit message is -->{}<--", obj.message());
    println!("Commit offset is -->{}<--", obj.time_offset());
    println!("Commit time is -->{}<--", obj.time());
    println!("Commit author is -->{}<--", obj.author());
    assert!(obj.parentcount() == 1);

    let obj = repo.lookup_commit("8717d8cfa524fd42ae3f8f3efc00bb5cace3af17").unwrap();
    println!("Commit message is -->{}<--", obj.message_encoding());
    assert!(obj.parentcount() == 0);



}
