/// This module contains the structs and functions to introspect a segment (memory mapping).
use anyhow::Result;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::process::Pid;

pub type InodeId = u64;

#[derive(Debug)]
pub enum SegmentParseError {
    ParseIntError(ParseIntError),
    ParseError(String),
}

impl Display for SegmentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SegmentParseError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            SegmentParseError::ParseError(s) => write!(f, "ParseError: {}", s),
        }
    }
}

impl Error for SegmentParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SegmentParseError::ParseIntError(e) => Some(e),
            SegmentParseError::ParseError(_) => None,
        }
    }
}

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
    type Err = SegmentParseError;

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
                _ => Err(SegmentParseError::ParseError(format!(
                    "Unknown segment type: {}",
                    s
                ))),
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

impl FromStr for SegmentPermission {
    type Err = SegmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(SegmentPermission::Read),
            "w" => Ok(SegmentPermission::Write),
            "x" => Ok(SegmentPermission::Execute),
            "-" => Ok(SegmentPermission::NoPermission),
            "p" => Ok(SegmentPermission::Private),
            "s" => Ok(SegmentPermission::Shared),
            _ => Err(SegmentParseError::ParseError(format!(
                "Unknown segment permission: {}",
                s
            ))),
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
    pub device: Device,
    /// Inode on that device
    pub inode: InodeId,
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
        device: Device,
        inode: InodeId,
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

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        let address = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No address".to_string()))?;
        let permissions = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No permissions".to_string()))?;
        let offset = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No offset".to_string()))?;
        let device = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No device".to_string()))?;
        let inode = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No inode".to_string()))?;
        let pathname = parts
            .next()
            .ok_or(SegmentParseError::ParseError("No pathname".to_string()))?;

        // Addresses range
        let addresses: Vec<&str> = address.split('-').collect();
        let start = u64::from_str_radix(addresses[0], 16)?;
        let end = u64::from_str_radix(addresses[1], 16)?;

        // Permissions
        let permissions = [
            SegmentPermission::from_str(&permissions[0..1])?,
            SegmentPermission::from_str(&permissions[1..2])?,
            SegmentPermission::from_str(&permissions[2..3])?,
            SegmentPermission::from_str(&permissions[3..4])?,
        ];

        let offset = u64::from_str_radix(offset, 16)?;
        let device = Device::from_str(device)?;
        let inode = u64::from_str(inode)?;
        let pathname = SegmentType::from_str(pathname)?;

        Ok(Segment::new(
            start,
            end,
            permissions,
            offset,
            device,
            inode,
            pathname,
        ))
    }
}

pub fn get_from_pid(pid: Pid) -> Result<Vec<Segment>> {
    let maps_path = format!("/proc/{}/maps", pid);
    let maps_file = File::open(maps_path)?;
    let maps_reader = BufReader::new(maps_file);
    let mut segments = Vec::new();

    for line in maps_reader.lines() {
        let line = line?;
        let segment = Segment::from_str(&line)?;

        segments.push(segment);
    }

    Ok(segments)
}
