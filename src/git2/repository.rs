extern crate libc;

use std::rc::Rc;
use std::ptr;
use std::path::Path;
use std::string::raw::from_buf;
use self::libc::{c_char, c_uchar, c_int};
use std::fmt::{Show, Formatter, FormatError};

use git2::error::{GitError, get_last_error};
use git2::reference::{Reference};
use git2::oid::{ToOID};
use git2::object::{Object, GitObjectType};
use git2::blob::{Blob};
use git2::commit::{Commit};
use git2::config::{Config};
use git2::config;


pub mod opaque {
    pub enum Repo {}
}

extern {
    fn git_repository_free(repo: *mut self::opaque::Repo);
    fn git_repository_init(repo: *mut *mut self::opaque::Repo, path: *const c_char, is_bare:u32) -> c_int;
    fn git_repository_open(repo: *mut *mut self::opaque::Repo, path: *const c_char) -> c_int;
    fn git_repository_open_bare(repo: *mut *mut self::opaque::Repo, path: *const c_char) -> c_int;
    fn git_repository_is_bare(repo: *mut self::opaque::Repo) -> c_int;
    fn git_repository_is_empty(repo: *mut self::opaque::Repo) -> c_int;
    fn git_repository_is_shallow(repo: *mut self::opaque::Repo) -> c_int;
    fn git_repository_path(repo: *mut self::opaque::Repo) -> *const c_uchar;
    fn git_repository_config(out: *mut *mut config::opaque::Config, repo: *mut self::opaque::Repo) -> c_int;
}


struct GitRepoPtr {
    _val: *mut self::opaque::Repo
}
    
#[deriving(Clone)]
pub struct Repository {
    _ptr: Rc<GitRepoPtr>
}


/// Represents a git repository
impl Repository {
    pub fn _new(p: *mut self::opaque::Repo) -> Repository {
        //! Not actually a public interface
        Repository {_ptr : Rc::new(GitRepoPtr{_val:p})} 
    }
    pub fn _get_ptr(&self) -> *mut self::opaque::Repo {
        //! Not actually a public interface
        self._ptr.deref()._val
    }

    /// Creates a new Git repository in the given folder.
    ///
    /// local_path : the path to the repository
    ///
    /// is_bare : if true, a Git repository without a working directory is created at the pointed
    /// path. If false, provided path will be considered as the working directory into which the
    /// .git directory will be created.
    ///
    /// Example:
    /// --------
    /// 
    /// ```ignore
    /// let dir = TempDir::new("git2_test1").unwrap();
    /// let repo = match git2::Repository::init(dir.path(), false) {
    ///     Ok(r) => r,
    ///     Err(e) => fail!("Failed to init repo:\n{}", e.message)
    /// };
    /// assert!(repo.is_empty() == true);
    ///
    /// ```
    ///
    pub fn init(local_path: &Path, is_bare: bool) -> Result<Repository, GitError> {
        let mut p: *mut self::opaque::Repo = ptr::mut_null();
        let ret = unsafe { git_repository_init(&mut p, local_path.to_c_str().unwrap(), is_bare as u32) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }
    
    /// Open a git repository.
    ///
    /// local_path : the path to the repository
    ///
    pub fn open(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut self::opaque::Repo = ptr::mut_null();
        let ret = unsafe { git_repository_open(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }

    /// Open a bare repository on the serverside.
    ///
    /// This is a fast open for bare repositories that will come in handy if you're e.g. hosting
    /// git repositories and need to access them efficiently
    ///
    /// local_path : Direct path to the bare repository
    ///
    pub fn open_bare(local_path: &Path) -> Result<Repository, GitError> {
        let mut p: *mut self::opaque::Repo = ptr::mut_null();
        let ret = unsafe { git_repository_open_bare(&mut p, local_path.to_c_str().unwrap()) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("git repo pointer ends at {}", p);
        Ok(Repository::_new(p))
    }

    /// Check if a repository is bare
    pub fn is_bare(&self) -> bool { unsafe {git_repository_is_bare(self._get_ptr()) == 1 } }

    /// Check if a repository is empty
    ///
    /// An empty repository has just been initialized and contains no references.
    pub fn is_empty(&self) -> bool { unsafe {git_repository_is_empty(self._get_ptr()) == 1 } }

    /// Determine if the repository was a shallow clone
    pub fn is_shallow(&self) -> bool { unsafe {git_repository_is_shallow(self._get_ptr()) == 1 } }

    /// Get the path of this repository
    ///
    /// This is the path of the `.git` folder for normal repositories, or of the repository itself
    /// for bare repositories.
    pub fn path(&self) -> Path {
        unsafe {
            let _path = git_repository_path(self._get_ptr());
            Path::new(from_buf(_path))
        }
    }

    /// Get the configuration file for this repository.
    ///
    /// If a configuration file has not been set, the default config set for the repository will be
    /// returned, including global and system configurations (if they are available).
    pub fn config(&self) -> Result<Config,GitError> {
        let mut p: *mut config::opaque::Config = ptr::mut_null();

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

impl Show for Repository {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("<Repository>").as_bytes())
    }
}


impl Drop for GitRepoPtr {
    fn drop(&mut self) {
        println!("Dropping this repository pointer!");
        unsafe {git_repository_free(self._val)} 
    }
}
