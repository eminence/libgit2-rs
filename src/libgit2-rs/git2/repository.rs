
use std::rc::Rc;
use std::ptr;
use std::path::Path;

use git2;
use git2::reference::{Reference, GitReference};

pub struct GitRepo;

priv struct GitRepoPtr {
    priv _val: *GitRepo
}
    
#[deriving(Clone)]
pub struct Repository {
    priv _ptr: Rc<GitRepoPtr>
}



impl Repository {
    fn _new(p: *GitRepo) -> Repository {
        Repository {_ptr : Rc::new(GitRepoPtr{_val:p})} 
    }
    fn _get_ptr(&self) -> *GitRepo {
        self._ptr.borrow()._val
    }
    pub fn init(local_path: &Path, is_bare: bool) -> Result<Repository, git2::GitError> {
        let p: *GitRepo = ptr::null();
        println!("git repo pointer starts at {}", p);
        let ret = unsafe { git2::git_repository_init(ptr::to_unsafe_ptr(&p), local_path.to_c_str().unwrap(), is_bare as u32) };
        if ret != 0 {
            return Err(git2::get_last_error().unwrap());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open(local_path: &Path) -> Result<Repository, git2::GitError> {
        let p: *GitRepo = ptr::null();
        println!("git repo pointer starts at {}", p);
        let ret = unsafe { git2::git_repository_open(ptr::to_unsafe_ptr(&p), local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(git2::get_last_error().unwrap());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    pub fn open_bare(local_path: &Path) -> Result<Repository, git2::GitError> {
        let p: *GitRepo = ptr::null();
        println!("git repo pointer starts at {}", p);
        let ret = unsafe { git2::git_repository_open_bare(ptr::to_unsafe_ptr(&p), local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(git2::get_last_error().unwrap());
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
            Path::new(::std::str::raw::from_c_str(_path))
        }
    }

    pub fn lookup_reference(&self, name: &str) -> Result<Reference, git2::GitError> {
        unsafe {
            let p: *GitReference = ptr::null();
            let ret = git2::git_reference_lookup(ptr::to_unsafe_ptr(&p), self._get_ptr(), name.to_c_str().unwrap());
            if ret != 0 {
                return Err(git2::get_last_error().unwrap());
            }
            println!("ref is OK");
            Ok(Reference::_new(self.clone(), p))
        }
    }
}



impl Drop for GitRepoPtr {
    fn drop(&mut self) {
        println!("Dropping this repository pointer!");
        unsafe {git2::git_repository_free(self._val)} 
    }
}
