extern crate libc;

use std::rc::Rc;
use std::ptr;
use std::path::Path;
use std::string::raw::from_buf;
use self::libc::{c_char, c_uchar, c_int};

use git2::error::{GitError, get_last_error};
use git2::reference::{Reference};
use git2::oid::{ToOID};
use git2::object::{Object, GitObjectType};
use git2::blob::{Blob};
use git2::commit::{Commit};
use git2::config::{Config,GitConfig};

extern {
    fn git_repository_free(repo: *mut GitRepo);
    fn git_repository_init(repo: *mut *mut GitRepo, path: *const c_char, is_bare:u32) -> c_int;
    fn git_repository_open(repo: *mut *mut GitRepo, path: *const c_char) -> c_int;
    fn git_repository_open_bare(repo: *mut *mut GitRepo, path: *const c_char) -> c_int;
    fn git_repository_is_bare(repo: *mut GitRepo) -> c_int;
    fn git_repository_is_empty(repo: *mut GitRepo) -> c_int;
    fn git_repository_is_shallow(repo: *mut GitRepo) -> c_int;
    fn git_repository_path(repo: *mut GitRepo) -> *const c_uchar;
    fn git_repository_config(out: *mut *mut GitConfig, repo: *mut GitRepo) -> c_int;
}

pub enum GitRepo {}

struct GitRepoPtr {
    _val: *mut GitRepo
}
    
#[deriving(Clone)]
pub struct Repository {
    _ptr: Rc<GitRepoPtr>
}



impl Repository {
    pub fn _new(p: *mut GitRepo) -> Repository {
        Repository {_ptr : Rc::new(GitRepoPtr{_val:p})} 
    }
    pub fn _get_ptr(&self) -> *mut GitRepo {
        self._ptr.deref()._val
    }
    pub fn init(local_path: &Path, is_bare: bool) -> Result<Repository, GitError> {
        let mut p: *mut GitRepo = ptr::mut_null();
        let ret = unsafe { git_repository_init(&mut p, local_path.to_c_str().unwrap(), is_bare as u32) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut GitRepo = ptr::mut_null();
        let ret = unsafe { git_repository_open(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open_bare(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut GitRepo = ptr::mut_null();
        let ret = unsafe { git_repository_open_bare(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn is_bare(&self) -> bool { unsafe {git_repository_is_bare(self._get_ptr()) == 1 } }
    pub fn is_empty(&self) -> bool { unsafe {git_repository_is_empty(self._get_ptr()) == 1 } }
    pub fn is_shallow(&self) -> bool { unsafe {git_repository_is_shallow(self._get_ptr()) == 1 } }
    pub fn path(&self) -> Path {
        unsafe {
            let _path = git_repository_path(self._get_ptr());
            Path::new(from_buf(_path))
        }
    }
    pub fn config(&self) -> Result<Config,GitError> {
        let mut p: *mut GitConfig = ptr::mut_null();

        match unsafe { git_repository_config(&mut p, self._get_ptr()) } {
            0 => Ok(Config::_new(p)),
            _ => Err(get_last_error())
        }

    }

    pub fn lookup_reference(&self, name: &str) -> Result<Reference, GitError> {
        Reference::lookup(self, name)
    }

    pub fn lookup_object<T: ToOID>(&self, oid: T, t: GitObjectType) -> Result<Object, GitError> {
        Object::lookup(self, oid, t)
    }

    pub fn lookup_blob<T: ToOID>(&self, oid: T) -> Result<Blob, GitError> {
        Blob::lookup(self, oid)
    }

    pub fn lookup_commit<T: ToOID>(&self, oid: T) -> Result<Commit, GitError> {
        Commit::lookup(self, oid)
    }
}



impl Drop for GitRepoPtr {
    fn drop(&mut self) {
        println!("Dropping this repository pointer!");
        unsafe {git_repository_free(self._val)} 
    }
}
