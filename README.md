[![Build Status](https://travis-ci.org/eminence/libgit2.svg?branch=master)](https://travis-ci.org/eminence/libgit2-rs)


libgit2-rs
==========

**Notice**
You almost certainly don't want to use this.  Use https://github.com/alexcrichton/git2-rs instead


rust bindings for libgit2 (version 0.20, maybe newer)

Works with cargo and a recent Rust (0.12-pre)

This project is incomplete, but in progress.  Maybe of the "read-only" APIs are working, but none
of the methods for creating new commits/blobs/trees/etc are in place.

Pull Requests will be happily accepted


Documentation at: http://eminence.github.io/libgit2-rs/doc/git2/git2/index.html

Example
=======
See the `tests/` directory for more examples

~~~rust
let dir =  TempDir::new("git2_test1").unwrap();
println!("Cloning into {}", dir.path().display());

let result = git2::clone("git://github.com/eminence/libgit2-rs.git", dir.path(), None);
let repo = match result {
    Err(e) => fail!("Failed to clone! {}", e),
    Ok(r) => r
};

assert!(repo.is_bare() == false);
let commit = repo.lookup_commit("444ff52ddb1f2f4b6a5627d57a5dd6d5a9fc86fd").unwrap();
println!("commit.message() -->{}<--", commit.message());
assert!(commit.message().as_slice() == "Added capability and version check\n");
assert!(commit.author().name.as_slice() == "Andrew Chin");
assert!(commit.author().email.as_slice() == "achin@eminence32.net");
assert!(commit.parentcount() == 1);
let parent = match commit.parent(0) {
    Err(r) => fail!("Failed to get parent! {}", r),
    Ok(p) => p
};
assert!(parent.id().to_string().as_slice() == "e7abafb574c707ba9906e90c1dc03126bab98973");
assert!(parent.id() == "e7abafb574c707ba9906e90c1dc03126bab98973".to_oid().unwrap());

~~~


Thread Saftey
=============
It's not.  When running tests, set `RUST_TEST_TASKS=1`

