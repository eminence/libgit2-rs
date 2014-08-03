
use std::rc::Rc;
use std::vec;
use git2;

pub struct GitBlob;


struct GitBlobPtr {
    _val: *mut GitBlob
}
    
#[deriving(Clone)]
pub struct Blob {
    _ptr: Rc<GitBlobPtr>,
}

impl Blob {
    pub fn _new(p: *mut GitBlob) -> Blob {
        Blob{_ptr: Rc::new(GitBlobPtr{_val:p})}
    }
    pub fn _get_ptr(&self) -> *mut GitBlob { self._ptr.deref()._val }
    pub fn rawsize(&self) -> i64 { unsafe {git2::git_blob_rawsize(self._get_ptr())}}
    pub fn rawcontent(&self) -> Vec<u8> {
        let size : uint = self.rawsize() as uint;
        let cptr = unsafe {
            git2::git_blob_rawcontent(self._get_ptr())
        };

        unsafe{vec::raw::from_buf(cptr, size)}

    }
}

impl Drop for GitBlobPtr {
    fn drop(&mut self) {
        println!("dropping this blob!");
        unsafe { git2::git_blob_free(self._val)}
    }
}
