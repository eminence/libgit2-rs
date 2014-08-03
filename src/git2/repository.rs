
use std::rc::Rc;
use std::ptr;
use std::path::Path;
use std::string::raw::from_buf;

use git2;
use git2::error::{GitError, get_last_error};
use git2::reference::{Reference, GitReference};
use git2::oid::{ToOID};
use git2::object::{GitObject, Object, GitObjectType};
use git2::blob::{Blob, GitBlob};
use git2::commit::{Commit, GitCommit};

pub struct GitRepo;

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
        let ret = unsafe { git2::git_repository_init(&mut p, local_path.to_c_str().unwrap(), is_bare as u32) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut GitRepo = ptr::mut_null();
        let ret = unsafe { git2::git_repository_open(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open_bare(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut GitRepo = ptr::mut_null();
        let ret = unsafe { git2::git_repository_open_bare(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn is_bare(&self) -> bool { unsafe {git2::git_repository_is_bare(self._get_ptr()) == 1 } }
    pub fn is_empty(&self) -> bool { unsafe {git2::git_repository_is_empty(self._get_ptr()) == 1 } }
    pub fn is_shallow(&self) -> bool { unsafe {git2::git_repository_is_shallow(self._get_ptr()) == 1 } }
    pub fn path(&self) -> Path {
        unsafe {
            let _path = git2::git_repository_path(self._get_ptr());
            Path::new(from_buf(_path))
        }
    }

    pub fn lookup_reference(&self, name: &str) -> Result<Reference, GitError> {
        unsafe {
            let mut p: *mut GitReference = ptr::mut_null();
            let ret = git2::git_reference_lookup(&mut p, self._get_ptr(), name.to_c_str().unwrap());
            if ret != 0 {
                return Err(get_last_error());
            }
            println!("ref is OK");
            Ok(Reference::_new(self.clone(), p))
        }
    }

    pub fn lookup_object<T: ToOID>(&self, oid: T, t: GitObjectType) -> Result<Object, GitError> {
        let mut p: *mut GitObject = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        println!("About to git_object_lookup");
        let ret = unsafe{ git2::git_object_lookup(&mut p, self._get_ptr(), _oid._get_ptr(), t) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("done git_object_lookup, p is {}", p);
        return Ok(Object::_new(p));
    }

    pub fn lookup_blob<T: ToOID>(&self, oid: T) -> Result<Blob, GitError> {
        let mut p: *mut GitBlob = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        let ret = unsafe { git2::git_blob_lookup(&mut p, self._get_ptr(), _oid._get_ptr()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("done git_object_lookup, p is {}", p);
        return Ok(Blob::_new(p, self));
    }

    pub fn lookup_commit<T: ToOID>(&self, oid: T) -> Result<Commit, GitError> {
        let mut p: *mut GitCommit = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        let ret = unsafe { git2::git_commit_lookup(&mut p, self._get_ptr(), _oid._get_ptr()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        return Ok(Commit::_new(p));
    }
}



impl Drop for GitRepoPtr {
    fn drop(&mut self) {
        println!("Dropping this repository pointer!");
        unsafe {git2::git_repository_free(self._val)} 
    }
}
