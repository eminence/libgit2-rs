
use std::rc::Rc;
use git2;

pub struct GitBlob;


priv struct GitBlobPtr {
    priv _val: *GitBlob
}
    
#[deriving(Clone)]
pub struct Blob {
    priv _ptr: Rc<GitBlobPtr>,
}

impl Blob {
    pub fn _new(p: *GitBlob) -> Blob {
        Blob{_ptr: Rc::new(GitBlobPtr{_val:p})}
    }
    pub fn _get_ptr(&self) -> *GitBlob { self._ptr.borrow()._val }
    pub fn rawsize(&self) -> i64 { unsafe {git2::git_blob_rawsize(self._get_ptr())}}
    pub fn rawcontent(&self) -> ~[u8] {
        let size : uint = self.rawsize() as uint;
        let cptr = unsafe {
            git2::git_blob_rawcontent(self._get_ptr())
        };

        unsafe{::std::vec::raw::from_buf_raw(cptr, size)}

    }
}

impl Drop for GitBlobPtr {
    fn drop(&mut self) {
        println!("dropping this blob!");
        unsafe { git2::git_blob_free(self._val)}
    }
}
