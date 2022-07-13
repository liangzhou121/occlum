use super::*;
use rcore_fs::vfs::FsError::DeviceError;

#[derive(Debug)]
pub struct DevRender;

impl INode for DevRender {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> vfs::Result<usize> {
        Err(FsError::PermError)
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> vfs::Result<usize> {
        Err(FsError::PermError)
    }

    fn poll(&self) -> vfs::Result<vfs::PollStatus> {
        Err(FsError::PermError)
    }

    fn metadata(&self) -> vfs::Result<Metadata> {
        Ok(Metadata {
            dev: 1,
            inode: 0,
            size: 0,
            blk_size: 0,
            blocks: 0,
            atime: Timespec { sec: 0, nsec: 0 },
            mtime: Timespec { sec: 0, nsec: 0 },
            ctime: Timespec { sec: 0, nsec: 0 },
            type_: vfs::FileType::RealDevice,
            mode: 0o666,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
        })
    }

    // fn io_control(&self, cmd: u32, data: usize) -> vfs::Result<()> {
    //     let mut ioctl_cmd =
    //         unsafe { IoctlCmd::new(cmd, data as *mut u8).map_err(|_| FsError::InvalidParam)? };

    //     let ioctl = |ioctl_cmd: IoctlCmd| -> Result<i32> {
    //         let mut render_fd = RENDER_FD.lock().unwrap();
    //         if render_fd.is_none() {
    //             let fd = try_libc!({
    //                 let mut fd: i32 = 0;
    //                 let status = occlum_open_i915(&mut fd as *mut i32);
    //                 assert!(status == sgx_status_t::SGX_SUCCESS);
    //                 fd
    //             });

    //             if fd < 0 {
    //                 return return_errno!(EACCES, "failed to open i915 device");
    //             }

    //             *render_fd = Some(fd);
    //         }

    //         let render_fd = render_fd.unwrap();
    //         let cmd_num = ioctl_cmd.cmd_num() as c_int;
    //         let cmd_arg_ptr = ioctl_cmd.arg_ptr() as *mut c_void;
    //         let ret = try_libc!({
    //             let mut retval: i32 = 0;
    //             let status = occlum_ocall_ioctl(
    //                 &mut retval as *mut i32,
    //                 render_fd,
    //                 cmd_num,
    //                 cmd_arg_ptr,
    //                 ioctl_cmd.arg_len(),
    //             );
    //             assert!(status == sgx_status_t::SGX_SUCCESS);
    //             retval
    //         });
    //         Ok(ret)
    //     };

    //     match ioctl(ioctl_cmd) {
    //         Ok(0) => Ok(()),
    //         Ok(e) => Err(DeviceError(e)),
    //         _ => Err(FsError::IOCTLError),
    //     }
    // }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

// lazy_static! {
//     pub static ref RENDER_FD: SgxMutex<Option<i32>> = { SgxMutex::new(None) };
// }

// extern "C" {
//     pub fn occlum_open_render(ret: *mut i32) -> sgx_status_t;
// }
