/// This module contains the structs and functions to introspect a segment (memory mapping).
use std::path::Path;

pub type InodeId = u64;

/// Small device abstrcation.
/// See https://linux-kernel-labs.github.io/refs/heads/master/labs/device_model.html#classes
pub struct Device {
    major: u32,
    minor: u32,
}

/// Information about a segment in the process's virtual address space.
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
pub enum DataSegment {
    /// The process's heap.
    Heap,
    Initialized,
    Uninitialized,
}

/// Permissions for a segment.
pub enum SegmentPermission {
    Read,
    Write,
    Execute,
    NoPermission,
    Private,
    Shared,
}

/// Mapped memory region in the process's virtual address space.
pub struct Segment {
    /// Start address
    start: u64,
    /// End address
    end: u64,
    // Permissions
    permissions: [SegmentPermission; 4],
    /// Offset into the file/whatever
    offset: u64,
    /// Device (major:minor)
    device: Option<Device>,
    /// Inode on that device
    inode: Option<InodeId>,
    /// Usually the file that is backing the mapping
    path: Path,
}
