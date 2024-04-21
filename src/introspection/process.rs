/// This module contains the structs and functions to introspect a process.
/// Based on https://www.man7.org/linux/man-pages/man5/proc.5.html
use crate::introspection::segment::Segment;

pub type Pid = u32; // maximum value: 2^22

/// Represents a process state.
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

/// Full status information about the process.
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
    /// Option because Process can be a thread.
    pub num_threads: Option<i8>,
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
