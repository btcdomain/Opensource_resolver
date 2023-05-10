
use std::fs::File;
use std::path::PathBuf;
use log::{info, debug};
use fs2::FileExt;


pub const FILE_LOCK_NAME: &str = "ord.file.lock";
pub const FILE_LOCK2_NAME: &str = "ord2.file.lock";


fn tmp_path(filename: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(filename);
    p
}

#[derive(Debug)]
pub struct FileLock(File);
impl FileLock {
    pub fn lock(file_name: &str) -> FileLock {
        let file_lock_file = tmp_path(file_name);
        info!("Acquiring File lock at {:?} ...", &file_lock_file);
        let f = File::create(&file_lock_file)
            .unwrap_or_else(|_| panic!("Cannot create File lock file at {:?}", &file_lock_file));
        f.lock_exclusive().unwrap();
        
        debug!("File lock acquired!");
        FileLock(f)
    }
}
impl Drop for FileLock {
    fn drop(&mut self) {
        self.0.unlock().unwrap();
        info!("File lock released!");
    }
}