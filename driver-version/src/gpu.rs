use nix::fcntl::{open, OFlag};
use nix::ioctl_readwrite;
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::os::fd::RawFd;

mod definitions;

// Allow dynamic types so we can freely use `?` everywhere to make the code
// simpler.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// From the linux kernel source tree, see:
// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/drm/drm.h#n966
ioctl_readwrite!(
    drm_driver_version,
    b'd',
    0x00,
    definitions::version::CDrmVersion
);

pub struct GPU {
    file_descriptor: RawFd,
}

/// GPU abstraction. Allows us to connect to the kernel-side GPU driver and
/// execute commands on it.
impl GPU {
    pub fn open() -> Result<Self> {
        let fd = open("/dev/dri/card0", OFlag::O_RDWR, Mode::empty())?;

        Ok(GPU {
            file_descriptor: fd,
        })
    }

    pub fn driver_version(&self) -> Result<definitions::version::DrmVersion> {
        let mut version: definitions::version::CDrmVersion = Default::default();

        unsafe {
            drm_driver_version(self.file_descriptor, &mut version)?;
        }

        Ok(version.into())
    }
}

// Resources clean up
impl Drop for GPU {
    fn drop(&mut self) {
        close(self.file_descriptor).unwrap()
    }
}
