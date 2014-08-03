extern crate libc;
use std::rc::Rc;
use std::ptr;
use std::string::raw::from_buf;
use std::iter::Iterator;
use self::libc::{c_int,c_char,c_uchar};

use git2;
use git2::error::{GitError, get_last_error};

extern {
    fn git_config_free(obj: *mut GitConfig);
    fn git_config_get_bool(out: *mut c_int, obj: *const GitConfig, name: *const c_char) -> c_int;
    fn git_config_get_string(out: *mut *mut c_char, obj: *const GitConfig, name: *const c_char) -> c_int;
    fn git_config_get_entry(out: *mut *mut GitConfigEntryRaw, obj: *const GitConfig, name: *const c_char) -> c_int;
    fn git_config_iterator_new(out: *mut *mut GitConfigIteratorRaw, obj: *const GitConfig) -> c_int;
    fn git_config_next(out: *mut *mut GitConfigEntryRaw, obj: *mut GitConfigIteratorRaw) -> c_int;
    fn git_config_iterator_free(obj: *mut GitConfigIteratorRaw);
}

// Represents the opaque git_config pointer type
pub enum GitConfig {}
// Represents the opaque git_config_iterator pointer type
pub enum GitConfigIteratorRaw {}

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

pub struct GitConfigEntryRaw {
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
    _val: *mut GitConfig
}


pub struct GitConfigIterator {
    _iter: *mut GitConfigIteratorRaw
}

#[deriving(Clone)]
pub struct Config {
    _ptr: Rc<GitConfigPtr>
}

impl Config {
    pub fn _get_ptr(&self) -> *const GitConfig { self._ptr.deref()._val as *const GitConfig }
    pub fn _new(p: *mut GitConfig) -> Config {
        Config {_ptr : Rc::new(GitConfigPtr{_val:p})} 
    }
    pub fn get_bool(&self, name: &str) -> Result<bool,GitError> {
        let mut val: c_int = -1; 
        match unsafe { git_config_get_bool(&mut val, self._get_ptr(), name.to_c_str().as_ptr()) } {
            0 => Ok(val == 1),
            _ => Err(get_last_error())
        }
    }
    pub fn get_string(&self, name: &str) -> Result<String,GitError> {
        let mut p: *mut c_char = ptr::mut_null();
        unsafe {
            match git_config_get_string(&mut p, self._get_ptr(), name.to_c_str().as_ptr()) {
                0 => Ok(from_buf(p as *const u8)),
                _ => Err(get_last_error())
            }
        }
    }
    
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
    pub fn iterator(&self) -> Result<GitConfigIterator,GitError> {
        let mut iter: *mut GitConfigIteratorRaw = ptr::mut_null();
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
