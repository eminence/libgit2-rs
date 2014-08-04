extern crate libc;
use self::libc::{c_char, c_int, c_uint};

use std::rc::Rc;
use std::ptr;
use git2::repository::{Repository};
use git2::repository;
use git2::oid::{OID, GitOid, ToOID};
use git2::error::{GitError, get_last_error};

// opaque pointerr classes
pub mod opaque {
    pub enum Commit {}     
}

extern {

    fn git_commit_free(obj: *mut self::opaque::Commit);
    fn git_commit_lookup(obj: *mut *mut self::opaque::Commit, repo: *mut repository::opaque::Repo, oid: *const GitOid) -> c_int;
    fn git_commit_message(obj: *mut self::opaque::Commit) -> *const c_char;
    fn git_commit_message_encoding(obj: *mut self::opaque::Commit) -> *const c_char;
    fn git_commit_parentcount(obj: *mut self::opaque::Commit) -> c_uint;
    fn git_commit_time_offset(obj: *mut self::opaque::Commit) -> c_int;
    fn git_commit_time(obj: *mut self::opaque::Commit) -> i64;
    fn git_commit_author(obj: *mut self::opaque::Commit) -> *const GitSignature;
    fn git_commit_committer(obj: *mut self::opaque::Commit) -> *const GitSignature;
    fn git_commit_parent(obj: *mut *mut self::opaque::Commit, me: *const opaque::Commit, n: c_uint) -> c_int;
    fn git_commit_id(obj: *const opaque::Commit) -> *const GitOid;
}

#[deriving(Show)]
/// Time in a signature
pub struct GitTime {
    time: i64,
    offset: i32
}

struct GitSignature {
    name: *mut c_char,
    email: *mut c_char,
    when: GitTime
}

#[deriving(Show)]
/// An action signature (e.g. for committers, taggers, etc)
pub struct Signature {
    pub name: String,
    pub email: String,
    pub when: GitTime
}


struct GitCommitPtr {
    _val: *mut self::opaque::Commit
}
    
#[deriving(Clone)]
pub struct Commit {
    _ptr: Rc<GitCommitPtr>,
    _num_parents: Option<uint>,
    _parents: Vec<Rc<Commit>>

}

impl Commit {
    fn _new(p: *mut self::opaque::Commit) -> Commit {
        Commit{
            _ptr: Rc::new(GitCommitPtr{_val:p}),
            _num_parents: None,
            _parents: vec![]
        }
    }

    /// Lookup a commit object from a repository.
    pub fn lookup<T: ToOID>(repo: &Repository, oid: T) -> Result<Commit, GitError> {
        let mut p: *mut self::opaque::Commit = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        let ret = unsafe { git_commit_lookup(&mut p, repo._get_ptr(), _oid._get_ptr()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        return Ok(Commit::_new(p));
    }
    pub fn _get_ptr(&self) -> *mut self::opaque::Commit { self._ptr.deref()._val }

    /// Get the full message of a commit.
    ///
    /// The returned message will be slightly prettified by removing any potential leading
    /// newlines.
    pub fn message(&self) -> String {
        unsafe {
            let _msg = git_commit_message(self._get_ptr());
            ::std::string::raw::from_buf(_msg as *const u8)
        }
    }

    /// Get the encoding for the message of a commit, as a string representing a standard encoding
    /// name.
    ///
    /// The encoding may be NULL if the encoding header in the commit is missing; in that case
    /// UTF-8 is assumed.
    pub fn message_encoding(&self) -> Option<String> {
        unsafe {
            let _msg = git_commit_message_encoding(self._get_ptr());
            if _msg.is_null() { return None }
            Some(::std::string::raw::from_buf(_msg as *const u8))
        }
    }

    /// Get the number of parents of this commit
    pub fn parentcount(&self) -> uint {
        unsafe {git_commit_parentcount(self._get_ptr()) as uint}
    }

    /// Get the parent commit
    ///
    /// idx must be less than parentcount
    pub fn parent(&self, idx: uint) -> Result<Commit,GitError> {
        let mut cmt: *mut opaque::Commit = ptr::mut_null();
        match unsafe { git_commit_parent(&mut cmt, self._get_ptr() as *const opaque::Commit, idx as c_uint) } {
            0 => Ok(Commit::_new(cmt)),
            _ => Err(get_last_error())
        }

    }
    
    /// Get the id of a commit
    pub fn id(&self) -> OID {
        unsafe {OID::_new(git_commit_id(self._get_ptr() as *const opaque::Commit))}
    }

    /// Get the commit timezone offset (i.e. committer's preferred timezone) of a commit.
    ///
    /// Returns positive or negative timezone offset, in minutes from UTC
    pub fn time_offset(&self) -> i32 {
        unsafe {git_commit_time_offset(self._get_ptr()) as i32}
    }

    /// Get the commit time (i.e. committer time) of a commit.
    pub fn time(&self) -> i64 {
        unsafe {git_commit_time(self._get_ptr()) as i64}
    }

    /// Get the author of a commit.
    pub fn author(&self) -> Signature {
        unsafe {
            let _sig: *const GitSignature = git_commit_author(self._get_ptr());
            let _sig2 : GitSignature =  *_sig ;
            let name = ::std::string::raw::from_buf(_sig2.name as *const u8);
            let email = ::std::string::raw::from_buf(_sig2.email as *const u8);
            Signature{name: name, email: email, when: _sig2.when}
        }
    }

    /// Get the committer of a commit.
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
