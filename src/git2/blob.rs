extern crate libc;

use std::rc::Rc;
use std::vec;
use self::libc::{c_int};
use git2;
use git2::repository::{GitRepo};
use git2::oid::{GitOid};

pub type GitOff = i64;

extern {
    fn git_blob_free(obj: *mut GitBlob);
    //fn git_blob_lookup(obj: *mut *mut GitBlob, repo: *mut GitRepo, oid: *const GitOid) -> c_int;
    fn git_blob_rawsize(obj: *const GitBlob) -> GitOff;
    fn git_blob_rawcontent(obj: *mut GitBlob) -> *const u8;
    fn git_blob_owner(obj: *const GitBlob) -> *mut GitRepo;
    fn git_blob_id(obj: *const GitBlob) -> *const GitOid;
    fn git_blob_is_binary(obj: *const GitBlob) -> c_int;

}


pub enum GitBlob {}


struct GitBlobPtr {
    _val: *mut GitBlob
}
    
#[deriving(Clone)]
pub struct Blob {
    _ptr: Rc<GitBlobPtr>,
    _repo: git2::Repository
}

impl Blob {
    pub fn _new(p: *mut GitBlob, repo: &git2::Repository) -> Blob {
        Blob {
            _ptr: Rc::new(GitBlobPtr{_val:p}),
            _repo: repo.clone()
        }
    }
    pub fn _get_ptr(&self) -> *mut GitBlob { self._ptr.deref()._val }
    pub fn _get_const_ptr(&self) -> *const GitBlob { self._ptr.deref()._val as *const GitBlob }
    pub fn rawsize(&self) -> GitOff { unsafe {git_blob_rawsize(self._get_const_ptr())}}
    pub fn rawcontent(&self) -> Vec<u8> {
        let size : uint = self.rawsize() as uint;
        let cptr = unsafe {
            // The pointerd returned is owned internall by libgit2 and may be invalidated later
            git_blob_rawcontent(self._get_ptr())
        };

        unsafe{vec::raw::from_buf(cptr, size)}

    }
    pub fn id(&self) -> git2::OID {
        unsafe {git2::OID::_new(git_blob_id(self._get_const_ptr()))}
    }
    pub fn is_binary(&self) -> bool {
        unsafe {git_blob_is_binary(self._get_const_ptr()) == 1}
    }
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
