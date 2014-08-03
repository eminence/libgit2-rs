

use std::rc::Rc;
use git2;
use std::libc;


#[deriving(ToStr)]
pub struct GitTime {
    time: i64,
    offset: i32
}

pub struct GitSignature {
    name: *libc::c_char,
    email: *libc::c_char,
    when: GitTime
}

#[deriving(ToStr)]
pub struct Signature {
    name: ~str,
    email: ~str,
    when: GitTime
}

pub struct GitCommit;

enum TriState {
    True,
    False,
    Unknown
}

priv struct GitCommitPtr {
    priv _val: *GitCommit
}
    
#[deriving(Clone)]
pub struct Commit {
    priv _ptr: Rc<GitCommitPtr>,
    priv _num_parents: Option<uint>,
    priv _parents: ~[Rc<Commit>]

}

impl Commit {
    pub fn _new(p: *GitCommit) -> Commit {
        Commit{
            _ptr: Rc::new(GitCommitPtr{_val:p}),
            _num_parents: None,
            _parents: ~[]
        }
    }
    pub fn _get_ptr(&self) -> *GitCommit { self._ptr.borrow()._val }

    pub fn message(&self) -> ~str {
        unsafe {
            let _msg = git2::git_commit_message(self._get_ptr());
            ::std::str::raw::from_c_str(_msg)
        }
    }
    pub fn message_encoding(&self) -> Option<~str> {
        unsafe {
            let _msg = git2::git_commit_message_encoding(self._get_ptr());
            if _msg.is_null() { return None }
            Some(::std::str::raw::from_c_str(_msg))
        }
    }
    pub fn parentcount(&self) -> uint {
        unsafe {git2::git_commit_parentcount(self._get_ptr()) as uint}
    }

    pub fn time_offset(&self) -> i32 {
        unsafe {git2::git_commit_time_offset(self._get_ptr()) as i32}
    }
    pub fn time(&self) -> i64 {
        unsafe {git2::git_commit_time(self._get_ptr()) as i64}
    }
    pub fn author(&self) -> Signature {
        unsafe {
            let _sig: *GitSignature = git2::git_commit_author(self._get_ptr());
            let _sig2 : GitSignature =  *_sig ;
            let name = ::std::str::raw::from_c_str(_sig2.name);
            let email = ::std::str::raw::from_c_str(_sig2.email);
            Signature{name: name, email: email, when: _sig2.when}
        }
    }
    pub fn committer(&self) -> Signature {
        unsafe {
            let _sig: *GitSignature = git2::git_commit_committer(self._get_ptr());
            let _sig2 : GitSignature =  *_sig ;
            let name = ::std::str::raw::from_c_str(_sig2.name);
            let email = ::std::str::raw::from_c_str(_sig2.email);
            Signature{name: name, email: email, when: _sig2.when}
        }
    }
}

impl Drop for GitCommitPtr {
    fn drop(&mut self) {
        println!("dropping this commit!");
        unsafe { git2::git_commit_free(self._val)}
    }
}
