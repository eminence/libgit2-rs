extern crate libc;
use self::libc::{c_uchar, c_char, c_int, c_uint};

use std::rc::Rc;
use git2;
use git2::repository::GitRepo;
use git2::oid::GitOid;

extern {

    fn git_commit_free(obj: *mut GitCommit);
    fn git_commit_lookup(obj: *mut *mut GitCommit, repo: *mut GitRepo, oid: *const GitOid) -> c_int;
    fn git_commit_message(obj: *mut GitCommit) -> *const c_char;
    fn git_commit_message_encoding(obj: *mut GitCommit) -> *const c_char;
    fn git_commit_parentcount(obj: *mut GitCommit) -> c_uint;
    fn git_commit_time_offset(obj: *mut GitCommit) -> c_int;
    fn git_commit_time(obj: *mut GitCommit) -> i64;
    fn git_commit_author(obj: *mut GitCommit) -> *const GitSignature;
    fn git_commit_committer(obj: *mut GitCommit) -> *const GitSignature;

}

#[deriving(Show)]
pub struct GitTime {
    time: i64,
    offset: i32
}

pub struct GitSignature {
    name: *mut c_char,
    email: *mut c_char,
    when: GitTime
}

#[deriving(Show)]
pub struct Signature {
    name: String,
    email: String,
    when: GitTime
}

pub struct GitCommit;

enum TriState {
    True,
    False,
    Unknown
}

struct GitCommitPtr {
    _val: *mut GitCommit
}
    
#[deriving(Clone)]
pub struct Commit {
    _ptr: Rc<GitCommitPtr>,
    _num_parents: Option<uint>,
    _parents: Vec<Rc<Commit>>

}

impl Commit {
    pub fn _new(p: *mut GitCommit) -> Commit {
        Commit{
            _ptr: Rc::new(GitCommitPtr{_val:p}),
            _num_parents: None,
            _parents: vec![]
        }
    }
    pub fn _get_ptr(&self) -> *mut GitCommit { self._ptr.deref()._val }

    pub fn message(&self) -> String {
        unsafe {
            let _msg = git_commit_message(self._get_ptr());
            ::std::str::raw::from_c_str(_msg)
        }
    }
    pub fn message_encoding(&self) -> Option<String> {
        unsafe {
            let _msg = git_commit_message_encoding(self._get_ptr());
            if _msg.is_null() { return None }
            Some(::std::str::raw::from_c_str(_msg))
        }
    }
    pub fn parentcount(&self) -> uint {
        unsafe {git_commit_parentcount(self._get_ptr()) as uint}
    }

    pub fn time_offset(&self) -> i32 {
        unsafe {git_commit_time_offset(self._get_ptr()) as i32}
    }
    pub fn time(&self) -> i64 {
        unsafe {git_commit_time(self._get_ptr()) as i64}
    }
    pub fn author(&self) -> Signature {
        unsafe {
            let _sig: *const GitSignature = git_commit_author(self._get_ptr());
            let _sig2 : GitSignature =  *_sig ;
            let name = ::std::string::raw::from_buf(_sig2.name as *const u8);
            let email = ::std::string::raw::from_buf(_sig2.email as *const u8);
            Signature{name: name, email: email, when: _sig2.when}
        }
    }
    pub fn committer(&self) -> Signature {
        unsafe {
            let _sig: *const GitSignature = git_commit_committer(self._get_ptr());
            let _sig2 : GitSignature =  *_sig ;
            let name = ::std::string::raw::from_buf(_sig2.name as *const u8);
            let email = ::std::string::raw::from_buf(_sig2.email as *const u8);
            Signature{name: name, email: email, when: _sig2.when}
        }
    }
}

impl Drop for GitCommitPtr {
    fn drop(&mut self) {
        println!("dropping this commit!");
        unsafe { git_commit_free(self._val)}
    }
}
