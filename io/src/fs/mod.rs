use uefi::proto::media::file::{Directory, File};
pub use uefi::proto::media::file::{FileAttribute, FileMode};

extern "C" {
    pub(crate) fn fat_reinit() -> Directory;
}

pub struct IFile;

impl IFile {
    pub fn get(&mut self) -> Directory {
        unsafe { return fat_reinit() }
    }
}
