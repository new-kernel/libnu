use uefi::proto::media::file::Directory;

extern "C" {
    pub(crate) fn fat_reinit() -> Directory;
}

pub struct File;

impl File {
    pub fn get(&mut self) -> Directory {
        unsafe { return fat_reinit() }
    }
}
