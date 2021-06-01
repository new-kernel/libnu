use uefi::proto::media::file::{Directory, File};
pub use uefi::proto::media::file::{FileAttribute, FileMode};

extern "C" {
    pub(crate) fn fat_reinit() -> Directory;
}
//                                 What?
//                                  ↓
/// The struct IFile is used for "getting" a file. The return type of the get function is Directory
/// from UEFI rs, this function gets the root directory information the same way Novusk does it so
/// you can read/write to files. FileAttribute and FileMode are publicly used from UEFI rs. The
/// reason why you should use this instead of getting the root directory info yourself is because
/// this is something you can do in 2 lines instead of 10+ lines
pub struct IFile;

/// ### Example:
/// ```
/// let mut file = IFile;
/// file.get().open(/* File arguments */);
/// ```
impl IFile {
    pub fn get(&mut self) -> Directory {
        unsafe { return fat_reinit() }
    }
}
