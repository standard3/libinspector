/// This module contains the structs and functions to introspect a segment (memory mapping).
use std::path::Path;

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

/// Information about a segment in the process's virtual address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentType {
    /// The initial process's (also known as the main thread's) stack.
    Stack,
    /// The virtual dynamically linked shared object.
    SharedLibrary,
    Data(DataSegment),
    Code,
    /// A named private anonymous mapping.
    Anonymous(String),
    /// A named shared anonymous mapping.
    SharedAnonymous(String),
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
    /// Type of the segment
    pub segment_type: SegmentType,
    /// Usually the file that is backing the mapping
    pub path: Box<Path>,
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
        segment_type: SegmentType,
        path: Box<Path>,
    ) -> Self {
        Segment {
            start,
            end,
            permissions,
            offset,
            device,
            inode,
            segment_type,
            path,
        }
    }

    /// Get the start address of the segment.
    pub fn start(&self) -> u64 {
        self.start
    }

    /// Get the end address of the segment.
    pub fn end(&self) -> u64 {
        self.end
    }

    /// Get the permissions of the segment.
    pub fn permissions(&self) -> &[SegmentPermission; 4] {
        &self.permissions
    }

    /// Get the offset of the segment.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Get the device of the segment.
    pub fn device(&self) -> Option<Device> {
        self.device.clone()
    }

    /// Get the inode of the segment.
    pub fn inode(&self) -> Option<InodeId> {
        self.inode
    }

    /// Get the type of the segment.
    pub fn segment_type(&self) -> &SegmentType {
        &self.segment_type
    }

    /// Get the path of the segment.
    pub fn path(&self) -> &Path {
        &self.path
    }
}
