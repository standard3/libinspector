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
    process_id: Pid,
    /// Filename of the executable
    name: String,
    // Process state
    state: ProcessState,
    /// The PID of the parent of this process.
    parent_id: Pid,
    /// The process group ID of the process.
    parent_group_id: Pid,
    /// The session ID of the process.
    session_id: Pid,
    /// The controlling terminal of the process.
    tty_nr: u32,
    /// The ID of the foreground process group of the controlling terminal of the process.
    tpgid: u32,
    /// The kernel flags word of the process.
    flags: u32,
    /// The number of minor faults the process has made which have not required loading a memory page from disk.
    minflt: u64,
    /// The number of minor faults that the process's waited-for children have made.
    cminflt: u64,
    /// The number of major faults the process has made which have required loading a memory page from disk.
    majflt: u64,
    /// The number of major faults that the process's waited-for children have made.
    cmajflt: u64,
    /// Amount of time that this process has been scheduled in user mode, measured in clock ticks.
    utime: u64,
    /// Amount of time that this process has been scheduled in kernel mode, measured in clock ticks.
    stime: u64,
    /// Amount of time that this process's waited-for children have been scheduled in user mode, measured in clock ticks
    cutime: u64,
    /// Amount of time that this process's waited-for children have been scheduled in kernel mode, measured in clock ticks
    cstime: u64,
    /// Obsolete
    priority: i8,
    /// The nice value.
    nice: i8,
    /// Number of threads in this process.
    /// Option because Process can be a thread.
    num_threads: Option<i8>,
    /// Obsolete
    itrealvalue: u64,
    /// The time the process started after system boot, measured in clock ticks.
    starttime: u64,
    /// Virtual memory size in bytes.
    vsize: u64,
    /// Resident Set Size: number of pages the process has in real memory.
    rss: u64,
    /// Current soft limit in bytes on the rss of the process.
    rsslim: u64,
    /// The address above which program text can run.
    startcode: u64,
    /// The address below which program text can run.
    endcode: u64,
    /// The address of the start (i.e., bottom) of the stack.
    startstack: u64,
    /// The current value of ESP (stack pointer), as found in the kernel stack page for the process.
    kstkesp: u64,
    /// The current EIP (instruction pointer).
    kstkeip: u64,
    /// Obsolete
    signal: u64,
    // Obsolete
    blocked: u64,
    /// Obsolete
    sigignore: u64,
    /// Obsolete
    sigcatch: u64,
    /// This is the "channel" in which the process is waiting.
    /// It is the address of a location in the kernel where the process is sleeping.
    wchan: u64,
    /// Number of pages swapped (not maintained).
    nswap: u64,
    /// Cumulative nswap for child processes (not maintained).
    cnswap: u64,
    /// Signal to be sent to parent when we die.
    exit_signal: i16,
    /// CPU number last executed on.
    processor: i16,
    /// Real-time scheduling priority, a number in the range 1 to 99 for processes
    /// scheduled under a real-time policy, or 0, for non-real-time processes
    rt_priority: u32,
    /// Scheduling policy.
    policy: u32,
    /// Aggregated block I/O delays, measured in clock ticks
    delayacct_blkio_ticks: u64,
    /// Guest time of the process (time spent running a virtual
    /// CPU for a guest operating system), measured in clock ticks
    guest_time: u64,
    /// Guest time of the process's children, measured in clock ticks
    cguest_time: u64,
    /// Address above which program initialized and uninitialized (BSS) data are placed.
    start_data: u64,
    /// Address below which program initialized and uninitialized (BSS) data are placed.
    end_data: u64,
    /// Address above which program heap can be expanded with [brk(2)](https://www.man7.org/linux/man-pages/man2/brk.2.html).
    start_brk: u64,
    /// Address above which program command-line arguments (argv) are placed.
    arg_start: u64,
    /// Address below program command-line arguments (argv) are placed.
    arg_end: u64,
    /// Address above which program environment is placed.
    env_start: u64,
    /// Address below which program environment is placed.
    env_end: u64,
    /// The thread's exit status in the form reported by [waitpid(2)](https://www.man7.org/linux/man-pages/man2/waitpid.2.html).
    exit_code: u32,

    // additional custom fields
    /// Threads in this process, threads in Linux are very similar to Processes so we use the same struct.
    threads: Option<Vec<Process>>,
    /// Segments in the process's virtual address space.
    segments: Vec<Box<Segment>>,
}
