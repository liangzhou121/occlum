use super::*;

use crate::vm::{MMapFlags, VMPerms};
use rcore_fs::vfs::FileType::SharedMemory;
use std::ffi::CString;

#[derive(Debug)]
pub struct SharedMemoryFile {}

impl File for SharedMemoryFile {
    fn set_len(&self, len: u64) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SharedMemoryFile {
    pub fn open(inode: Arc<dyn INode>, abs_path: &str, flags: u32) -> Result<Self> {
        error!("path {:?} len {:?}", abs_path, abs_path.len());
        if inode.metadata().is_err() {
            return_errno!(EACCES, "File has no metadata");
        }

        let metadata = inode.metadata().unwrap();
        if metadata.type_ != SharedMemory {
            return_errno!(EACCES, "File not a shared memory file");
        }

        Ok(SharedMemoryFile {})
    }

    pub fn mmap(
        &self,
        addr: usize,
        size: usize,
        perms: VMPerms,
        flags: MMapFlags,
        fd: FileDesc,
        offset: usize,
    ) -> Result<usize> {
        error!(
            "device mmap: size:{:?} perms:{:?} flags:{:?} fd:{:?} offset:{:#01x}",
            size,
            perms.bits(),
            flags.bits(),
            fd,
            offset
        );
        let mut ret_addr: u64 = 0;
        unsafe {
            occlum_ocall_device_mmap(
                &mut ret_addr,
                addr as u64,
                size as size_t,
                perms.bits() as i32,
                33, // MAP_SHARED | MAP_ANONYMOUS
                -1, // not a real file
                0,  // from the beginning
            );
        }
        Ok(ret_addr as usize)
    }

    pub fn munmap(&self, addr: usize, size: usize) -> Result<()> {
        let mut ret: i32 = 0;
        unsafe {
            occlum_ocall_device_munmap(&mut ret, addr as u64, size);
        }

        Ok(())
    }
}

pub trait AsSharedMemoryFile {
    fn as_sharedmemory_file(&self) -> Result<&SharedMemoryFile>;
}

impl AsSharedMemoryFile for FileRef {
    fn as_sharedmemory_file(&self) -> Result<&SharedMemoryFile> {
        self.as_any()
            .downcast_ref::<SharedMemoryFile>()
            .ok_or_else(|| errno!(EBADF, "not an device file"))
    }
}

extern "C" {
    pub fn occlum_ocall_device_mmap(
        ret_addr: *mut u64,
        addr: u64,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: u64,
    ) -> sgx_status_t;

    pub fn occlum_ocall_device_munmap(ret: *mut i32, addr: u64, length: size_t) -> sgx_status_t;
}
