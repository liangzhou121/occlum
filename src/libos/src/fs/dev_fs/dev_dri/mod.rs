use super::*;

mod dev_i915;
mod dev_render;

use self::dev_i915::*;
use self::dev_render::*;

#[derive(Debug)]
pub struct DevDri;

impl INode for DevDri {
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
            type_: vfs::FileType::Dir,
            mode: 0o666,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
        })
    }

    fn find(&self, name: &str) -> vfs::Result<Arc<dyn INode>> {
        match name {
            "card0" => Ok(Arc::new(Devi915)),
            "card1" => Ok(Arc::new(Devi915)),
            "renderD128" => Ok(Arc::new(DevRender)),
            _ => Err(FsError::NotSupported),
        }
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}
