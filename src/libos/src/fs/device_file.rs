use super::*;

use rcore_fs::vfs::FileType::RealDevice;

#[derive(Debug)]
pub struct DeviceFile {
    // host_fd: HostFd,
    inode_file: INodeFile,
}

impl File for DeviceFile {
    fn metadata(&self) -> Result<Metadata> {
        self.inode_file.metadata()
    }

    fn ioctl(&self, cmd: &mut IoctlCmd) -> Result<i32> {
        self.inode_file.ioctl(cmd)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DeviceFile {
    pub fn open(inode: Arc<dyn INode>, abs_path: &str, flags: u32) -> Result<Self> {
        if inode.metadata().is_err() {
            return_errno!(EACCES, "File has no metadata");
        }

        let metadata = inode.metadata().unwrap();
        if metadata.type_ != RealDevice {
            return_errno!(EACCES, "File not a device");
        }

        let inode_file = INodeFile::open(inode, abs_path, flags);
        match inode_file {
            Ok(inode_file) => Ok(DeviceFile { inode_file }),
            Err(e) => Err(e),
        }
    }
}
