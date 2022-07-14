use super::*;

use crate::vm::{MMapFlags, VMPerms};
use rcore_fs::vfs::FileType::RealDevice;
use std::ffi::CString;

#[derive(Debug)]
pub struct DeviceFile {
    host_fd: HostFd,
    inode_file: INodeFile,
}

impl File for DeviceFile {
    fn metadata(&self) -> Result<Metadata> {
        self.inode_file.metadata()
    }

    fn ioctl(&self, cmd: &mut IoctlCmd) -> Result<i32> {
        let host_fd = self.host_fd.to_raw() as c_int;
        let cmd_num = cmd.cmd_num() as c_int;
        let cmd_arg_ptr = cmd.arg_ptr() as *mut c_void;
        let ret = try_libc!({
            let mut retval: i32 = 0;
            // let status = occlum_ocall_ioctl(
            //     &mut retval as *mut i32,
            //     host_fd,
            //     cmd_num,
            //     cmd_arg_ptr,
            //     cmd.arg_len(),
            // );

            let status = occlum_ocall_device_ioctl(
                &mut retval as *mut i32,
                host_fd,
                cmd_num,
                cmd_arg_ptr as u64,
            );
            assert!(status == sgx_status_t::SGX_SUCCESS);
            retval
        });
        Ok(ret)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DeviceFile {
    pub fn open(inode: Arc<dyn INode>, abs_path: &str, flags: u32) -> Result<Self> {
        error!("path {:?} len {:?}", abs_path, abs_path.len());
        if inode.metadata().is_err() {
            return_errno!(EACCES, "File has no metadata");
        }

        let metadata = inode.metadata().unwrap();
        if metadata.type_ != RealDevice {
            return_errno!(EACCES, "File not a device");
        }

        let raw_host_fd = try_libc!({
            let device_path = CString::new(abs_path).unwrap();
            let mut fd: i32 = 0;
            let status = occlum_ocall_open_device(
                &mut fd as *mut i32,
                device_path.as_ptr() as *const u8,
                abs_path.len() + 1,
                flags,
            );
            assert!(status == sgx_status_t::SGX_SUCCESS);
            fd
        });
        error!("raw_host_fd {:?}", raw_host_fd);

        if raw_host_fd < 0 {
            return_errno!(EACCES, "File open failed");
        }
        let host_fd = HostFd::new(raw_host_fd as u32);

        let inode_file = INodeFile::open(inode, abs_path, flags);
        match inode_file {
            Ok(inode_file) => Ok(DeviceFile {
                host_fd,
                inode_file,
            }),
            Err(e) => Err(e),
        }
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
            self.host_fd.to_raw(),
            offset
        );
        let mut ret_addr: u64 = 0;
        unsafe {
            occlum_ocall_device_mmap(
                &mut ret_addr,
                addr as u64,
                size as size_t,
                perms.bits() as i32,
                flags.bits() as i32,
                self.host_fd.to_raw() as i32,
                offset as u64,
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

pub trait AsDeviceFile {
    fn as_device_file(&self) -> Result<&DeviceFile>;
}

impl AsDeviceFile for FileRef {
    fn as_device_file(&self) -> Result<&DeviceFile> {
        self.as_any()
            .downcast_ref::<DeviceFile>()
            .ok_or_else(|| errno!(EBADF, "not an device file"))
    }
}

extern "C" {
    pub fn occlum_ocall_open_device(
        ret: *mut i32,
        device_name_buf: *const u8,
        device_name_buf_len: size_t,
        flags: u32,
    ) -> sgx_status_t;

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

    pub fn occlum_ocall_device_ioctl(
        ret: *mut i32,
        fd: c_int,
        request: c_int,
        arg: u64,
    ) -> sgx_status_t;
}
