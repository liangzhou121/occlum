use super::*;

#[derive(Debug)]
pub struct Devi915;

impl INode for Devi915 {
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
            type_: vfs::FileType::CharDevice,
            mode: 0o666,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
        })
    }

    fn io_control(&self, _cmd: u32, _data: usize) -> vfs::Result<()> {
        let mut ioctl_cmd =
            unsafe { IoctlCmd::new(_cmd, _data as *mut u8).map_err(|_| FsError::InvalidParam)? };
        self.ioctl(&mut ioctl_cmd).map_err(|e| {
            error!("{}", e.backtrace());
            FsError::IOCTLError
        })?;
        Ok(())
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl Devi915 {
    fn ioctl(&self, cmd: &mut IoctlCmd) -> Result<i32> {
        Ok(0)
    }
}
