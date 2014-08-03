extern crate libc;

use std::rc::Rc;
use std::vec;
use std::ptr;
use self::libc::{c_int};
use git2;
use git2::repository::{Repository};
use git2::repository;
use git2::oid::{GitOid,ToOID};
use git2::error::{GitError, get_last_error};

pub type GitOff = i64;

pub mod opaque {
    pub enum Blob {}
}

extern {
    fn git_blob_free(obj: *mut self::opaque::Blob);
    fn git_blob_lookup(obj: *mut *mut self::opaque::Blob, repo: *mut repository::opaque::Repo, oid: *const GitOid) -> c_int;
    fn git_blob_rawsize(obj: *const self::opaque::Blob) -> GitOff;
    fn git_blob_rawcontent(obj: *mut self::opaque::Blob) -> *const u8;
    fn git_blob_owner(obj: *const self::opaque::Blob) -> *mut repository::opaque::Repo;
    fn git_blob_id(obj: *const self::opaque::Blob) -> *const GitOid;
    fn git_blob_is_binary(obj: *const self::opaque::Blob) -> c_int;

}



struct GitBlobPtr {
    _val: *mut self::opaque::Blob
}
    
#[deriving(Clone)]
pub struct Blob {
    _ptr: Rc<GitBlobPtr>,
    _repo: git2::Repository
}

impl Blob {
    fn _new(p: *mut self::opaque::Blob, repo: &git2::Repository) -> Blob {
        Blob {
            _ptr: Rc::new(GitBlobPtr{_val:p}),
            _repo: repo.clone()
        }
    }

    /// Lookup a blob object from a repository.
    pub fn lookup<T: ToOID>(repo: &Repository, oid: T) -> Result<Blob, GitError> {
        let mut p: *mut self::opaque::Blob = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        let ret = unsafe { git_blob_lookup(&mut p, repo._get_ptr(), _oid._get_ptr()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("done git_object_lookup, p is {}", p);
        return Ok(Blob::_new(p, repo));
    }

    pub fn _get_ptr(&self) -> *mut self::opaque::Blob { self._ptr.deref()._val }
    pub fn _get_const_ptr(&self) -> *const self::opaque::Blob { self._ptr.deref()._val as *const self::opaque::Blob }

    /// Get the size in bytes of the contents of a blob
    pub fn rawsize(&self) -> GitOff { unsafe {git_blob_rawsize(self._get_const_ptr())}}

    /// Get a buffer with the raw content of a blob.
    pub fn rawcontent(&self) -> Vec<u8> {
        let size : uint = self.rawsize() as uint;
        let cptr = unsafe {
            // The pointerd returned is owned internall by libgit2 and may be invalidated later
            git_blob_rawcontent(self._get_ptr())
        };

        unsafe{vec::raw::from_buf(cptr, size)}

    }

    /// Get the id of a blob.
    pub fn id(&self) -> git2::OID {
        unsafe {git2::OID::_new(git_blob_id(self._get_const_ptr()))}
    }

    /// Determine if the blob content is most certainly binary or not.
    ///
    /// The heuristic used to guess if a file is binary is taken from core git: Searching for NUL
    /// bytes and looking for a reasonable ratio of printable to non-printable characters among the
    /// first 4000 bytes.
    pub fn is_binary(&self) -> bool {
        unsafe {git_blob_is_binary(self._get_const_ptr()) == 1}
    }

    /// Get the repository that contains the blob.
    pub fn owner(&self) -> &git2::Repository {
        unsafe { 
            let p = git_blob_owner(self._get_const_ptr());
            if p != self._repo._get_ptr() {
                fail!("Repo mismatch!");
            }
        }
        return &self._repo;
        
    }
}

impl Drop for GitBlobPtr {
    fn drop(&mut self) {
        println!("dropping this blob!");
        unsafe { git_blob_free(self._val)}
    }
}
