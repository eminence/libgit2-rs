extern crate libc;

//use std::num::{FromPrimitive};
use std::rc::Rc;
use std::ptr;
use self::libc::{c_int};

use git2::error::{GitError, get_last_error};
use git2::repository::{Repository};
use git2::repository;
use git2::oid::{ToOID, GitOid};

pub mod opaque {
    pub enum Object {}
}

extern {
    fn git_object_free(obj: *mut self::opaque::Object);
    fn git_object_lookup(obj: *mut *mut self::opaque::Object, repo: *mut repository::opaque::Repo, oid: *const GitOid, t:GitObjectType) -> c_int;
    fn git_object_type(obj: *mut self::opaque::Object) -> GitObjectType;
}

#[deriving(Eq,PartialEq,FromPrimitive)]
#[repr(C)]
pub enum GitObjectType {
    GIT_OBJ_ANY = -2,                //*< Object can be any of the following */
    GIT_OBJ_BAD = -1,                //*< Object is invalid. */
    GIT_OBJ__EXT1 = 0,                //*< Reserved for future use. */
    GIT_OBJ_COMMIT = 1,                //*< A commit object. */
    GIT_OBJ_TREE = 2,                //*< A tree (directory listing) object. */
    GIT_OBJ_BLOB = 3,                //*< A file revision object. */
    GIT_OBJ_TAG = 4,                //*< An annotated tag object. */
    GIT_OBJ__EXT2 = 5,                //*< Reserved for future use. */
    GIT_OBJ_OFS_DELTA = 6, //*< A delta, base is given by an offset. */
    GIT_OBJ_REF_DELTA = 7, //*< A delta, base is given by object id. */
}

struct GitObjPtr {
    _val: *mut self::opaque::Object
}
    
#[deriving(Clone)]
pub struct Object {
    _ptr: Rc<GitObjPtr>,
}

impl Object {
    fn _new(p: *mut self::opaque::Object) -> Object {
        Object{_ptr: Rc::new(GitObjPtr{_val:p})}
    }
    pub fn lookup<T: ToOID>(repo: &Repository, oid:T, t: GitObjectType) -> Result<Object, GitError> {
        let mut p: *mut self::opaque::Object = ptr::mut_null();
        let _oid = match oid.to_oid() {
            Err(e) => {return Err(e); },
            Ok(o) => o
        };

        println!("About to git_object_lookup");
        let ret = unsafe{ git_object_lookup(&mut p, repo._get_ptr(), _oid._get_ptr(), t) };
        if ret != 0 {
            return Err(get_last_error());
        }
        println!("done git_object_lookup, p is {}", p);
        return Ok(Object::_new(p));
    }
    pub fn _get_ptr(&self) -> *mut self::opaque::Object { self._ptr.deref()._val }
    pub fn get_type(&self) -> GitObjectType {
        unsafe { git_object_type(self._get_ptr()) }
    }
}

impl Drop for GitObjPtr {
    fn drop(&mut self) {
        println!("dropping this object!");
        unsafe { git_object_free(self._val)}
    }
}
