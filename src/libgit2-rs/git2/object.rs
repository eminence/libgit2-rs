
//use std::num::{FromPrimitive};
use std::rc::Rc;
use git2;


pub struct GitObject;

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
    _val: *mut GitObject
}
    
#[deriving(Clone)]
pub struct Object {
    _ptr: Rc<GitObjPtr>,
}

impl Object {
    pub fn _new(p: *mut GitObject) -> Object {
        Object{_ptr: Rc::new(GitObjPtr{_val:p})}
    }
    pub fn _get_ptr(&self) -> *mut GitObject { self._ptr.deref()._val }
    pub fn get_type(&self) -> GitObjectType {
        unsafe { git2::git_object_type(self._get_ptr()) }
    }
}

impl Drop for GitObjPtr {
    fn drop(&mut self) {
        println!("dropping this object!");
        unsafe { git2::git_object_free(self._val)}
    }
}
