extern crate libc;
use std::rc::Rc;
use std::ptr;
use std::string::raw::from_buf;
use std::iter::Iterator;
use self::libc::{c_int,c_char,c_uchar};

use git2::error::{GitError, get_last_error};

pub mod opaque {
    pub enum Config {}
    pub enum ConfigIterator {}
}

extern {
    fn git_config_free(obj: *mut self::opaque::Config);
    fn git_config_get_bool(out: *mut c_int, obj: *const self::opaque::Config, name: *const c_char) -> c_int;
    fn git_config_get_string(out: *mut *mut c_char, obj: *const self::opaque::Config, name: *const c_char) -> c_int;
    fn git_config_get_entry(out: *mut *mut GitConfigEntryRaw, obj: *const self::opaque::Config, name: *const c_char) -> c_int;
    fn git_config_iterator_new(out: *mut *mut self::opaque::ConfigIterator, obj: *const self::opaque::Config) -> c_int;
    fn git_config_next(out: *mut *mut GitConfigEntryRaw, obj: *mut self::opaque::ConfigIterator) -> c_int;
    fn git_config_iterator_free(obj: *mut self::opaque::ConfigIterator);
}


#[repr(C)]
#[deriving(Show)]
pub enum GitConfigLevel {
    GIT_CONFIG_LEVEL_SYSTEM = 1,
    GIT_CONFIG_LEVEL_XDG = 2,
    GIT_CONFIG_LEVEL_GLOBAL = 3,
    GIT_CONFIG_LEVEL_LOCAL = 4,
    GIT_CONFIG_LEVEL_APP = 5,
    GIT_CONFIG_HIGHEST_LEVEL = -1,

}

struct GitConfigEntryRaw {
    name: *const c_uchar,
    value: *const c_uchar,
    level: GitConfigLevel
}

#[deriving(Show)]
pub struct GitConfigEntry {
    pub name: String,
    pub value: String,
    pub level: GitConfigLevel
}


struct GitConfigPtr {
    _val: *mut self::opaque::Config
}


pub struct GitConfigIterator {
    _iter: *mut self::opaque::ConfigIterator
}

#[deriving(Clone)]
pub struct Config {
    _ptr: Rc<GitConfigPtr>
}

impl Config {
    pub fn _get_ptr(&self) -> *const self::opaque::Config { self._ptr.deref()._val as *const self::opaque::Config }
    pub fn _new(p: *mut self::opaque::Config) -> Config {
        Config {_ptr : Rc::new(GitConfigPtr{_val:p})} 
    }

    /// Get the value of a boolean config variable.
    ///
    /// This function uses the usual C convention of 0 being false and anything else true.
    ///
    /// All config files will be looked into, in the order of their defined level. A higher level
    /// means a higher priority. The first occurrence of the variable will be returned here.
    pub fn get_bool(&self, name: &str) -> Result<bool,GitError> {
        let mut val: c_int = -1; 
        match unsafe { git_config_get_bool(&mut val, self._get_ptr(), name.to_c_str().as_ptr()) } {
            0 => Ok(val == 1),
            _ => Err(get_last_error())
        }
    }

    /// Get the value of a string config variable.
    ///
    /// All config files will be looked into, in the order of their defined level. A higher level
    /// means a higher priority. The first occurrence of the variable will be returned here
    pub fn get_string(&self, name: &str) -> Result<String,GitError> {
        let mut p: *mut c_char = ptr::mut_null();
        unsafe {
            match git_config_get_string(&mut p, self._get_ptr(), name.to_c_str().as_ptr()) {
                0 => Ok(from_buf(p as *const u8)),
                _ => Err(get_last_error())
            }
        }
    }
    
    /// Get the git_config_entry of a config variable.
    pub fn get_entry(&self, name: &str) -> Result<GitConfigEntry,GitError> {
        let mut entryptr: *mut GitConfigEntryRaw = ptr::mut_null();
        unsafe {
            if git_config_get_entry(&mut entryptr, self._get_ptr(), name.to_c_str().as_ptr()) != 0 {
                return Err(get_last_error());
            }
            let deref: GitConfigEntryRaw = *entryptr;
            return Ok(GitConfigEntry{
                name: from_buf(deref.name),
                value: from_buf(deref.value),
                level: deref.level
            });
        }
    }

    /// Iterate over all the config variables
    pub fn iterator(&self) -> Result<GitConfigIterator,GitError> {
        let mut iter: *mut self::opaque::ConfigIterator = ptr::mut_null();
        if unsafe { git_config_iterator_new(&mut iter, self._get_ptr()) } != 0 {
            return Err(get_last_error());
        }
        return Ok(GitConfigIterator{_iter: iter});

    }

}

impl Iterator<GitConfigEntry> for GitConfigIterator {
    fn next(&mut self) -> Option<GitConfigEntry> {
        let mut entryptr: *mut GitConfigEntryRaw = ptr::mut_null();
        let ret = unsafe { git_config_next(&mut entryptr, self._iter) };
        if ret == 0 {
            return unsafe {
                let deref: GitConfigEntryRaw = *entryptr;
                Some(GitConfigEntry{
                    name: from_buf(deref.name),
                    value: from_buf(deref.value),
                    level: deref.level
                })
            }
        } else if ret == -31 { // iter over
            return None;
        } else {
            fail!("git_config_next failure! {}", get_last_error());
        }

    }
}

impl Drop for GitConfigPtr {
    fn drop(&mut self) {
        println!("Dropping this config pointer!");
        unsafe {git_config_free(self._val)} 
    }
}

impl Drop for GitConfigIterator {
    fn drop(&mut self) {
        println!("Dropping this config iterator!");
        unsafe {git_config_iterator_free(self._iter)} 
    }
}
