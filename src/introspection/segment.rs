/// This module contains the structs and functions to introspect a segment (memory mapping).
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

// TODO:
// - implement common traits on structs (e.g. Debug, Clone, PartialEq, Eq, ToString + FromStr, Display)

pub type InodeId = u64;

/// Small device abstraction.
/// See <https://linux-kernel-labs.github.io/refs/heads/master/labs/device_model.html#classes>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Device {
    pub major: u32,
    pub minor: u32,
}

impl Device {
    /// Create a new device.
    pub fn new(major: u32, minor: u32) -> Self {
        Device { major, minor }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.major, self.minor)
    }
}

impl From<(u32, u32)> for Device {
    fn from((major, minor): (u32, u32)) -> Self {
        Device { major, minor }
    }
}

impl From<Device> for (u32, u32) {
    fn from(device: Device) -> Self {
        (device.major, device.minor)
    }
}

impl FromStr for Device {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let major = parts[0].parse()?;
        let minor = parts[1].parse()?;
        Ok(Device { major, minor })
    }
}

/// Information about a segment in the process's virtual address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentType {
    /// The initial process's (also known as the main thread's) stack.
    Stack,
    /// The virtual dynamically linked shared object.
    SharedLibrary,
    Data(DataSegment),
    Code(Box<PathBuf>),
    /// A named private anonymous mapping.
    Anonymous(String),
    /// A named shared anonymous mapping.
    SharedAnonymous(String),
}

impl Display for SegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SegmentType::Stack => write!(f, "[stack]"),
            SegmentType::SharedLibrary => write!(f, "[vdso]"),
            SegmentType::Data(DataSegment::Heap) => write!(f, "[heap]"),
            SegmentType::Data(DataSegment::Initialized) => todo!(),
            SegmentType::Data(DataSegment::Uninitialized) => todo!(),
            SegmentType::Code(path) => write!(f, "{}", path.display()),
            SegmentType::Anonymous(name) => write!(f, "[anon:{}]", name),
            SegmentType::SharedAnonymous(name) => write!(f, "[anon_shmem:{}]", name),
        }
    }
}

impl FromStr for SegmentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[anon:") && s.ends_with(']') {
            let name = s.trim_start_matches("[anon:").trim_end_matches(']');
            Ok(SegmentType::Anonymous(name.to_string()))
        } else if s.starts_with("[anon_shmem:") && s.ends_with(']') {
            let name = s.trim_start_matches("[anon_shmem:").trim_end_matches(']');
            Ok(SegmentType::SharedAnonymous(name.to_string()))
        } else if s.starts_with('[') && s.ends_with(']') {
            match s {
                "[stack]" => Ok(SegmentType::Stack),
                "[vdso]" => Ok(SegmentType::SharedLibrary),
                "[heap]" => Ok(SegmentType::Data(DataSegment::Heap)),
                _ => Err(()),
            }
        } else {
            Ok(SegmentType::Code(Box::new(Path::new(&s).to_path_buf())))
        }
    }
}

/// Type of data segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSegment {
    /// The process's heap.
    Heap,
    Initialized,
    Uninitialized,
}

/// Permissions for a segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentPermission {
    Read,
    Write,
    Execute,
    NoPermission,
    Private,
    Shared,
}

impl Display for SegmentPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SegmentPermission::Read => write!(f, "r"),
            SegmentPermission::Write => write!(f, "w"),
            SegmentPermission::Execute => write!(f, "x"),
            SegmentPermission::NoPermission => write!(f, "-"),
            SegmentPermission::Private => write!(f, "p"),
            SegmentPermission::Shared => write!(f, "s"),
        }
    }
}

/// Mapped memory region in the process's virtual address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    /// Start address
    pub start: u64,
    /// End address
    pub end: u64,
    // Permissions
    pub permissions: [SegmentPermission; 4],
    /// Offset into the file/whatever
    pub offset: u64,
    /// Device (major:minor)
    pub device: Option<Device>,
    /// Inode on that device
    pub inode: Option<InodeId>,
    /// Usually the file that is backing the mapping
    pub pathname: SegmentType,
}

impl Segment {
    /// Create a new segment.
    pub fn new(
        start: u64,
        end: u64,
        permissions: [SegmentPermission; 4],
        offset: u64,
        device: Option<Device>,
        inode: Option<InodeId>,
        pathname: SegmentType,
    ) -> Self {
        Segment {
            start,
            end,
            permissions,
            offset,
            device,
            inode,
            pathname,
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:016x}-{:016x} {:?} {:016x} {:?} {:?} {}",
            self.start,
            self.end,
            self.permissions,
            self.offset,
            self.device,
            self.inode,
            self.pathname,
        )
    }
}
