/// This module contains the structs and functions to introspect a process.
/// Based on https://www.man7.org/linux/man-pages/man5/proc.5.html
use crate::introspection::segment::Segment;
use anyhow::Result;
use std::{error::Error, fmt::Display, num::ParseIntError, str::FromStr};

pub type Pid = u32; // maximum value: 2^22

#[derive(Debug)]
pub enum ProcessParseError {
    ParseIntError(ParseIntError),
    ParseError(String),
}

impl Display for ProcessParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessParseError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            ProcessParseError::ParseError(s) => write!(f, "ParseError: {}", s),
        }
    }
}

impl Error for ProcessParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessParseError::ParseIntError(e) => Some(e),
            ProcessParseError::ParseError(_) => None,
        }
    }
}

/// Represents a process state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// R : Running
    Running,
    /// D : Waiting in uninterruptible disk sleep
    UninterruptibleSleep,
    /// S : Sleeping in an interruptible wait
    InterruptibleSleep,
    /// T : Stopped (on a signal)
    Stopped,
    /// Z : Zombie
    Zombie,
    /// t : Tracing stop
    Tracing,
    /// X, x : Dead
    Dead,
    /// I : Idle
    Idle,
}

impl Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let state = match self {
            ProcessState::Running => "R",
            ProcessState::UninterruptibleSleep => "D",
            ProcessState::InterruptibleSleep => "S",
            ProcessState::Stopped => "T",
            ProcessState::Zombie => "Z",
            ProcessState::Tracing => "t",
            ProcessState::Dead => "X",
            ProcessState::Idle => "I",
        };
        write!(f, "{}", state)
    }
}

impl FromStr for ProcessState {
    type Err = ProcessParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(ProcessState::Running),
            "D" => Ok(ProcessState::UninterruptibleSleep),
            "S" => Ok(ProcessState::InterruptibleSleep),
            "T" => Ok(ProcessState::Stopped),
            "Z" => Ok(ProcessState::Zombie),
            "t" => Ok(ProcessState::Tracing),
            "X" => Ok(ProcessState::Dead),
            "I" => Ok(ProcessState::Idle),
            _ => Err(ProcessParseError::ParseError(format!(
                "Unknown process state: {}",
                s
            ))),
        }
    }
}

/// Full status information about the process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    /// The process ID
    pub process_id: Pid,
    /// Filename of the executable
    pub name: String,
    // Process state
    pub state: ProcessState,
    /// The PID of the parent of this process.
    pub parent_id: Pid,
    /// The process group ID of the process.
    pub parent_group_id: Pid,
    /// The session ID of the process.
    pub session_id: Pid,
    /// The controlling terminal of the process.
    pub tty_nr: u32,
    /// The ID of the foreground process group of the controlling terminal of the process.
    pub tpgid: u32,
    /// The kernel flags word of the process.
    pub flags: u32,
    /// The number of minor faults the process has made which have not required loading a memory page from disk.
    pub minflt: u64,
    /// The number of minor faults that the process's waited-for children have made.
    pub cminflt: u64,
    /// The number of major faults the process has made which have required loading a memory page from disk.
    pub majflt: u64,
    /// The number of major faults that the process's waited-for children have made.
    pub cmajflt: u64,
    /// Amount of time that this process has been scheduled in user mode, measured in clock ticks.
    pub utime: u64,
    /// Amount of time that this process has been scheduled in kernel mode, measured in clock ticks.
    pub stime: u64,
    /// Amount of time that this process's waited-for children have been scheduled in user mode, measured in clock ticks
    pub cutime: u64,
    /// Amount of time that this process's waited-for children have been scheduled in kernel mode, measured in clock ticks
    pub cstime: u64,
    /// Obsolete
    pub priority: i8,
    /// The nice value.
    pub nice: i8,
    /// Number of threads in this process.
    pub num_threads: i8,
    /// Obsolete
    pub itrealvalue: u64,
    /// The time the process started after system boot, measured in clock ticks.
    pub starttime: u64,
    /// Virtual memory size in bytes.
    pub vsize: u64,
    /// Resident Set Size: number of pages the process has in real memory.
    pub rss: u64,
    /// Current soft limit in bytes on the rss of the process.
    pub rsslim: u64,
    /// The address above which program text can run.
    pub startcode: u64,
    /// The address below which program text can run.
    pub endcode: u64,
    /// The address of the start (i.e., bottom) of the stack.
    pub startstack: u64,
    /// The current value of ESP (stack pointer), as found in the kernel stack page for the process.
    pub kstkesp: u64,
    /// The current EIP (instruction pointer).
    pub kstkeip: u64,
    /// Obsolete
    pub signal: u64,
    // Obsolete
    pub blocked: u64,
    /// Obsolete
    pub sigignore: u64,
    /// Obsolete
    pub sigcatch: u64,
    /// This is the "channel" in which the process is waiting.
    /// It is the address of a location in the kernel where the process is sleeping.
    pub wchan: u64,
    /// Number of pages swapped (not maintained).
    pub nswap: u64,
    /// Cumulative nswap for child processes (not maintained).
    pub cnswap: u64,
    /// Signal to be sent to parent when we die.
    pub exit_signal: i16,
    /// CPU number last executed on.
    pub processor: i16,
    /// Real-time scheduling priority, a number in the range 1 to 99 for processes
    /// scheduled under a real-time policy, or 0, for non-real-time processes
    pub rt_priority: u32,
    /// Scheduling policy.
    pub policy: u32,
    /// Aggregated block I/O delays, measured in clock ticks
    pub delayacct_blkio_ticks: u64,
    /// Guest time of the process (time spent running a virtual
    /// CPU for a guest operating system), measured in clock ticks
    pub guest_time: u64,
    /// Guest time of the process's children, measured in clock ticks
    pub cguest_time: u64,
    /// Address above which program initialized and uninitialized (BSS) data are placed.
    pub start_data: u64,
    /// Address below which program initialized and uninitialized (BSS) data are placed.
    pub end_data: u64,
    /// Address above which program heap can be expanded with [brk(2)](https://www.man7.org/linux/man-pages/man2/brk.2.html).
    pub start_brk: u64,
    /// Address above which program command-line arguments (argv) are placed.
    pub arg_start: u64,
    /// Address below program command-line arguments (argv) are placed.
    pub arg_end: u64,
    /// Address above which program environment is placed.
    pub env_start: u64,
    /// Address below which program environment is placed.
    pub env_end: u64,
    /// The thread's exit status in the form reported by [waitpid(2)](https://www.man7.org/linux/man-pages/man2/waitpid.2.html).
    pub exit_code: u32,

    // additional custom fields
    /// Threads in this process, threads in Linux are very similar to Processes so we use the same struct.
    pub threads: Option<Vec<Process>>,
    /// Segments in the process's virtual address space.
    pub segments: Vec<Box<Segment>>,
}

impl Process {
    /// Create a new process.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        process_id: Pid,
        name: String,
        state: ProcessState,
        parent_id: Pid,
        parent_group_id: Pid,
        session_id: Pid,
        tty_nr: u32,
        tpgid: u32,
        flags: u32,
        minflt: u64,
        cminflt: u64,
        majflt: u64,
        cmajflt: u64,
        utime: u64,
        stime: u64,
        cutime: u64,
        cstime: u64,
        priority: i8,
        nice: i8,
        num_threads: i8,
        itrealvalue: u64,
        starttime: u64,
        vsize: u64,
        rss: u64,
        rsslim: u64,
        startcode: u64,
        endcode: u64,
        startstack: u64,
        kstkesp: u64,
        kstkeip: u64,
        signal: u64,
        blocked: u64,
        sigignore: u64,
        sigcatch: u64,
        wchan: u64,
        nswap: u64,
        cnswap: u64,
        exit_signal: i16,
        processor: i16,
        rt_priority: u32,
        policy: u32,
        delayacct_blkio_ticks: u64,
        guest_time: u64,
        cguest_time: u64,
        start_data: u64,
        end_data: u64,
        start_brk: u64,
        arg_start: u64,
        arg_end: u64,
        env_start: u64,
        env_end: u64,
        exit_code: u32,
    ) -> Self {
        let threads = None; // TODO
        let segments = Vec::new();

        Process {
            process_id,
            name,
            state,
            parent_id,
            parent_group_id,
            session_id,
            tty_nr,
            tpgid,
            flags,
            minflt,
            cminflt,
            majflt,
            cmajflt,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            num_threads,
            itrealvalue,
            starttime,
            vsize,
            rss,
            rsslim,
            startcode,
            endcode,
            startstack,
            kstkesp,
            kstkeip,
            signal,
            blocked,
            sigignore,
            sigcatch,
            wchan,
            nswap,
            cnswap,
            exit_signal,
            processor,
            rt_priority,
            policy,
            delayacct_blkio_ticks,
            guest_time,
            cguest_time,
            start_data,
            end_data,
            start_brk,
            arg_start,
            arg_end,
            env_start,
            env_end,
            exit_code,
            threads,
            segments,
        }
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Process {} ({}): {}",
            self.process_id, self.name, self.state
        )
    }
}

impl FromStr for Process {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let process_id = parts[0].parse()?;
        let name = parts[1].to_string();
        let state = parts[2].parse()?;
        let parent_id = parts[3].parse()?;
        let parent_group_id = parts[4].parse()?;
        let session_id = parts[5].parse()?;
        let tty_nr = parts[6].parse()?;
        let tpgid = parts[7].parse()?;
        let flags = parts[8].parse()?;
        let minflt = parts[9].parse()?;
        let cminflt = parts[10].parse()?;
        let majflt = parts[11].parse()?;
        let cmajflt = parts[12].parse()?;
        let utime = parts[13].parse()?;
        let stime = parts[14].parse()?;
        let cutime = parts[15].parse()?;
        let cstime = parts[16].parse()?;
        let priority = parts[17].parse()?;
        let nice = parts[18].parse()?;
        let num_threads = parts[19].parse()?;
        let itrealvalue = parts[20].parse()?;
        let starttime = parts[21].parse()?;
        let vsize = parts[22].parse()?;
        let rss = parts[23].parse()?;
        let rsslim = parts[24].parse()?;
        let startcode = parts[25].parse()?;
        let endcode = parts[26].parse()?;
        let startstack = parts[27].parse()?;
        let kstkesp = parts[28].parse()?;
        let kstkeip = parts[29].parse()?;
        let signal = parts[30].parse()?;
        let blocked = parts[31].parse()?;
        let sigignore = parts[32].parse()?;
        let sigcatch = parts[33].parse()?;
        let wchan = parts[34].parse()?;
        let nswap = parts[35].parse()?;
        let cnswap = parts[36].parse()?;
        let exit_signal = parts[37].parse()?;
        let processor = parts[38].parse()?;
        let rt_priority = parts[39].parse()?;
        let policy = parts[40].parse()?;
        let delayacct_blkio_ticks = parts[41].parse()?;
        let guest_time = parts[42].parse()?;
        let cguest_time = parts[43].parse()?;
        let start_data = parts[44].parse()?;
        let end_data = parts[45].parse()?;
        let start_brk = parts[46].parse()?;
        let arg_start = parts[47].parse()?;
        let arg_end = parts[48].parse()?;
        let env_start = parts[49].parse()?;
        let env_end = parts[50].parse()?;
        let exit_code = parts[51].parse()?;

        Ok(Process::new(
            process_id,
            name,
            state,
            parent_id,
            parent_group_id,
            session_id,
            tty_nr,
            tpgid,
            flags,
            minflt,
            cminflt,
            majflt,
            cmajflt,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            num_threads,
            itrealvalue,
            starttime,
            vsize,
            rss,
            rsslim,
            startcode,
            endcode,
            startstack,
            kstkesp,
            kstkeip,
            signal,
            blocked,
            sigignore,
            sigcatch,
            wchan,
            nswap,
            cnswap,
            exit_signal,
            processor,
            rt_priority,
            policy,
            delayacct_blkio_ticks,
            guest_time,
            cguest_time,
            start_data,
            end_data,
            start_brk,
            arg_start,
            arg_end,
            env_start,
            env_end,
            exit_code,
        ))
    }
}
