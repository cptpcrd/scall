//! System call numbers for MIPS64 Linux.

/* automatically generated by nr_from_src.py */

pub const _NEWSELECT: usize = 5022;
pub const ACCEPT: usize = 5042;
pub const ACCEPT4: usize = 5293;
pub const ACCESS: usize = 5020;
pub const ACCT: usize = 5158;
pub const ADD_KEY: usize = 5239;
pub const ADJTIMEX: usize = 5154;
pub const ALARM: usize = 5037;
pub const BIND: usize = 5048;
pub const BPF: usize = 5315;
pub const BRK: usize = 5012;
pub const CACHEFLUSH: usize = 5197;
pub const CAPGET: usize = 5123;
pub const CAPSET: usize = 5124;
pub const CHDIR: usize = 5078;
pub const CHMOD: usize = 5088;
pub const CHOWN: usize = 5090;
pub const CHROOT: usize = 5156;
pub const CLOCK_ADJTIME: usize = 5300;
pub const CLOCK_GETRES: usize = 5223;
pub const CLOCK_GETTIME: usize = 5222;
pub const CLOCK_NANOSLEEP: usize = 5224;
pub const CLOCK_SETTIME: usize = 5221;
pub const CLONE: usize = 5055;
pub const CLONE3: usize = 5435;
pub const CLOSE: usize = 5003;
pub const CLOSE_RANGE: usize = 5436;
pub const CONNECT: usize = 5041;
pub const COPY_FILE_RANGE: usize = 5320;
pub const CREAT: usize = 5083;
pub const DELETE_MODULE: usize = 5169;
pub const DUP: usize = 5031;
pub const DUP2: usize = 5032;
pub const DUP3: usize = 5286;
pub const EPOLL_CREATE: usize = 5207;
pub const EPOLL_CREATE1: usize = 5285;
pub const EPOLL_CTL: usize = 5208;
pub const EPOLL_PWAIT: usize = 5272;
pub const EPOLL_PWAIT2: usize = 5441;
pub const EPOLL_WAIT: usize = 5209;
pub const EVENTFD: usize = 5278;
pub const EVENTFD2: usize = 5284;
pub const EXECVE: usize = 5057;
pub const EXECVEAT: usize = 5316;
pub const EXIT: usize = 5058;
pub const EXIT_GROUP: usize = 5205;
pub const FACCESSAT: usize = 5259;
pub const FACCESSAT2: usize = 5439;
pub const FADVISE64: usize = 5215;
pub const FALLOCATE: usize = 5279;
pub const FANOTIFY_INIT: usize = 5295;
pub const FANOTIFY_MARK: usize = 5296;
pub const FCHDIR: usize = 5079;
pub const FCHMOD: usize = 5089;
pub const FCHMODAT: usize = 5258;
pub const FCHOWN: usize = 5091;
pub const FCHOWNAT: usize = 5250;
pub const FCNTL: usize = 5070;
pub const FDATASYNC: usize = 5073;
pub const FGETXATTR: usize = 5185;
pub const FINIT_MODULE: usize = 5307;
pub const FLISTXATTR: usize = 5188;
pub const FLOCK: usize = 5071;
pub const FORK: usize = 5056;
pub const FREMOVEXATTR: usize = 5191;
pub const FSCONFIG: usize = 5431;
pub const FSETXATTR: usize = 5182;
pub const FSMOUNT: usize = 5432;
pub const FSOPEN: usize = 5430;
pub const FSPICK: usize = 5433;
pub const FSTAT: usize = 5005;
pub const FSTATFS: usize = 5135;
pub const FSYNC: usize = 5072;
pub const FTRUNCATE: usize = 5075;
pub const FUTEX: usize = 5194;
pub const FUTIMESAT: usize = 5251;
pub const GET_MEMPOLICY: usize = 5228;
pub const GET_ROBUST_LIST: usize = 5269;
pub const GETCPU: usize = 5271;
pub const GETCWD: usize = 5077;
pub const GETDENTS: usize = 5076;
pub const GETDENTS64: usize = 5308;
pub const GETEGID: usize = 5106;
pub const GETEGID32: usize = 5106;
pub const GETEUID: usize = 5105;
pub const GETEUID32: usize = 5105;
pub const GETGID: usize = 5102;
pub const GETGID32: usize = 5102;
pub const GETGROUPS: usize = 5113;
pub const GETGROUPS32: usize = 5113;
pub const GETITIMER: usize = 5035;
pub const GETPEERNAME: usize = 5051;
pub const GETPGID: usize = 5119;
pub const GETPGRP: usize = 5109;
pub const GETPID: usize = 5038;
pub const GETPPID: usize = 5108;
pub const GETPRIORITY: usize = 5137;
pub const GETRANDOM: usize = 5313;
pub const GETRESGID: usize = 5118;
pub const GETRESGID32: usize = 5118;
pub const GETRESUID: usize = 5116;
pub const GETRESUID32: usize = 5116;
pub const GETRLIMIT: usize = 5095;
pub const GETRUSAGE: usize = 5096;
pub const GETSID: usize = 5122;
pub const GETSOCKNAME: usize = 5050;
pub const GETSOCKOPT: usize = 5054;
pub const GETTID: usize = 5178;
pub const GETTIMEOFDAY: usize = 5094;
pub const GETUID: usize = 5100;
pub const GETUID32: usize = 5100;
pub const GETXATTR: usize = 5183;
pub const INIT_MODULE: usize = 5168;
pub const INOTIFY_ADD_WATCH: usize = 5244;
pub const INOTIFY_INIT: usize = 5243;
pub const INOTIFY_INIT1: usize = 5288;
pub const INOTIFY_RM_WATCH: usize = 5245;
pub const IO_CANCEL: usize = 5204;
pub const IO_DESTROY: usize = 5201;
pub const IO_GETEVENTS: usize = 5202;
pub const IO_PGETEVENTS: usize = 5328;
pub const IO_SETUP: usize = 5200;
pub const IO_SUBMIT: usize = 5203;
pub const IO_URING_ENTER: usize = 5426;
pub const IO_URING_REGISTER: usize = 5427;
pub const IO_URING_SETUP: usize = 5425;
pub const IOCTL: usize = 5015;
pub const IOPRIO_GET: usize = 5274;
pub const IOPRIO_SET: usize = 5273;
pub const KCMP: usize = 5306;
pub const KEXEC_LOAD: usize = 5270;
pub const KEYCTL: usize = 5241;
pub const KILL: usize = 5060;
pub const LANDLOCK_ADD_RULE: usize = 5445;
pub const LANDLOCK_CREATE_RULESET: usize = 5444;
pub const LANDLOCK_RESTRICT_SELF: usize = 5446;
pub const LCHOWN: usize = 5092;
pub const LGETXATTR: usize = 5184;
pub const LINK: usize = 5084;
pub const LINKAT: usize = 5255;
pub const LISTEN: usize = 5049;
pub const LISTXATTR: usize = 5186;
pub const LLISTXATTR: usize = 5187;
pub const LOOKUP_DCOOKIE: usize = 5206;
pub const LREMOVEXATTR: usize = 5190;
pub const LSEEK: usize = 5008;
pub const LSETXATTR: usize = 5181;
pub const LSTAT: usize = 5006;
pub const MADVISE: usize = 5027;
pub const MBIND: usize = 5227;
pub const MEMBARRIER: usize = 5318;
pub const MEMFD_CREATE: usize = 5314;
pub const MIGRATE_PAGES: usize = 5246;
pub const MINCORE: usize = 5026;
pub const MKDIR: usize = 5081;
pub const MKDIRAT: usize = 5248;
pub const MKNOD: usize = 5131;
pub const MKNODAT: usize = 5249;
pub const MLOCK: usize = 5146;
pub const MLOCK2: usize = 5319;
pub const MLOCKALL: usize = 5148;
pub const MMAP: usize = 5009;
pub const MOUNT: usize = 5160;
pub const MOUNT_SETATTR: usize = 5442;
pub const MOVE_MOUNT: usize = 5429;
pub const MOVE_PAGES: usize = 5267;
pub const MPROTECT: usize = 5010;
pub const MQ_GETSETATTR: usize = 5235;
pub const MQ_NOTIFY: usize = 5234;
pub const MQ_OPEN: usize = 5230;
pub const MQ_TIMEDRECEIVE: usize = 5233;
pub const MQ_TIMEDSEND: usize = 5232;
pub const MQ_UNLINK: usize = 5231;
pub const MREMAP: usize = 5024;
pub const MSGCTL: usize = 5069;
pub const MSGGET: usize = 5066;
pub const MSGRCV: usize = 5068;
pub const MSGSND: usize = 5067;
pub const MSYNC: usize = 5025;
pub const MUNLOCK: usize = 5147;
pub const MUNLOCKALL: usize = 5149;
pub const MUNMAP: usize = 5011;
pub const NAME_TO_HANDLE_AT: usize = 5298;
pub const NANOSLEEP: usize = 5034;
pub const NEWFSTATAT: usize = 5252;
pub const NFSSERVCTL: usize = 5173;
pub const OPEN: usize = 5002;
pub const OPEN_BY_HANDLE_AT: usize = 5299;
pub const OPEN_TREE: usize = 5428;
pub const OPENAT: usize = 5247;
pub const OPENAT2: usize = 5437;
pub const PAUSE: usize = 5033;
pub const PERF_EVENT_OPEN: usize = 5292;
pub const PERSONALITY: usize = 5132;
pub const PIDFD_GETFD: usize = 5438;
pub const PIDFD_OPEN: usize = 5434;
pub const PIDFD_SEND_SIGNAL: usize = 5424;
pub const PIPE: usize = 5021;
pub const PIPE2: usize = 5287;
pub const PIVOT_ROOT: usize = 5151;
pub const PKEY_ALLOC: usize = 5324;
pub const PKEY_FREE: usize = 5325;
pub const PKEY_MPROTECT: usize = 5323;
pub const POLL: usize = 5007;
pub const PPOLL: usize = 5261;
pub const PRCTL: usize = 5153;
pub const PREAD64: usize = 5016;
pub const PREADV: usize = 5289;
pub const PREADV2: usize = 5321;
pub const PRLIMIT64: usize = 5297;
pub const PROCESS_MADVISE: usize = 5440;
pub const PROCESS_VM_READV: usize = 5304;
pub const PROCESS_VM_WRITEV: usize = 5305;
pub const PSELECT6: usize = 5260;
pub const PTRACE: usize = 5099;
pub const PWRITE64: usize = 5017;
pub const PWRITEV: usize = 5290;
pub const PWRITEV2: usize = 5322;
pub const QUOTACTL: usize = 5172;
pub const QUOTACTL_FD: usize = 5443;
pub const READ: usize = 5000;
pub const READAHEAD: usize = 5179;
pub const READLINK: usize = 5087;
pub const READLINKAT: usize = 5257;
pub const READV: usize = 5018;
pub const REBOOT: usize = 5164;
pub const RECVFROM: usize = 5044;
pub const RECVMMSG: usize = 5294;
pub const RECVMSG: usize = 5046;
pub const REMAP_FILE_PAGES: usize = 5210;
pub const REMOVEXATTR: usize = 5189;
pub const RENAME: usize = 5080;
pub const RENAMEAT: usize = 5254;
pub const RENAMEAT2: usize = 5311;
pub const REQUEST_KEY: usize = 5240;
pub const RESTART_SYSCALL: usize = 5213;
pub const RMDIR: usize = 5082;
pub const RSEQ: usize = 5327;
pub const RT_SIGACTION: usize = 5013;
pub const RT_SIGPENDING: usize = 5125;
pub const RT_SIGPROCMASK: usize = 5014;
pub const RT_SIGQUEUEINFO: usize = 5127;
pub const RT_SIGRETURN: usize = 5211;
pub const RT_SIGSUSPEND: usize = 5128;
pub const RT_SIGTIMEDWAIT: usize = 5126;
pub const RT_TGSIGQUEUEINFO: usize = 5291;
pub const SCHED_GET_PRIORITY_MAX: usize = 5143;
pub const SCHED_GET_PRIORITY_MIN: usize = 5144;
pub const SCHED_GETAFFINITY: usize = 5196;
pub const SCHED_GETATTR: usize = 5310;
pub const SCHED_GETPARAM: usize = 5140;
pub const SCHED_GETSCHEDULER: usize = 5142;
pub const SCHED_RR_GET_INTERVAL: usize = 5145;
pub const SCHED_SETAFFINITY: usize = 5195;
pub const SCHED_SETATTR: usize = 5309;
pub const SCHED_SETPARAM: usize = 5139;
pub const SCHED_SETSCHEDULER: usize = 5141;
pub const SCHED_YIELD: usize = 5023;
pub const SECCOMP: usize = 5312;
pub const SEMCTL: usize = 5064;
pub const SEMGET: usize = 5062;
pub const SEMOP: usize = 5063;
pub const SEMTIMEDOP: usize = 5214;
pub const SENDFILE: usize = 5039;
pub const SENDMMSG: usize = 5302;
pub const SENDMSG: usize = 5045;
pub const SENDTO: usize = 5043;
pub const SET_MEMPOLICY: usize = 5229;
pub const SET_ROBUST_LIST: usize = 5268;
pub const SET_THREAD_AREA: usize = 5242;
pub const SET_TID_ADDRESS: usize = 5212;
pub const SETDOMAINNAME: usize = 5166;
pub const SETFSGID: usize = 5121;
pub const SETFSGID32: usize = 5121;
pub const SETFSUID: usize = 5120;
pub const SETFSUID32: usize = 5120;
pub const SETGID: usize = 5104;
pub const SETGID32: usize = 5104;
pub const SETGROUPS: usize = 5114;
pub const SETGROUPS32: usize = 5114;
pub const SETHOSTNAME: usize = 5165;
pub const SETITIMER: usize = 5036;
pub const SETNS: usize = 5303;
pub const SETPGID: usize = 5107;
pub const SETPRIORITY: usize = 5138;
pub const SETREGID: usize = 5112;
pub const SETREGID32: usize = 5112;
pub const SETRESGID: usize = 5117;
pub const SETRESGID32: usize = 5117;
pub const SETRESUID: usize = 5115;
pub const SETRESUID32: usize = 5115;
pub const SETREUID: usize = 5111;
pub const SETREUID32: usize = 5111;
pub const SETRLIMIT: usize = 5155;
pub const SETSID: usize = 5110;
pub const SETSOCKOPT: usize = 5053;
pub const SETTIMEOFDAY: usize = 5159;
pub const SETUID: usize = 5103;
pub const SETUID32: usize = 5103;
pub const SETXATTR: usize = 5180;
pub const SHMAT: usize = 5029;
pub const SHMCTL: usize = 5030;
pub const SHMDT: usize = 5065;
pub const SHMGET: usize = 5028;
pub const SHUTDOWN: usize = 5047;
pub const SIGALTSTACK: usize = 5129;
pub const SIGNALFD: usize = 5276;
pub const SIGNALFD4: usize = 5283;
pub const SOCKET: usize = 5040;
pub const SOCKETPAIR: usize = 5052;
pub const SPLICE: usize = 5263;
pub const STAT: usize = 5004;
pub const STATFS: usize = 5134;
pub const STATX: usize = 5326;
pub const SWAPOFF: usize = 5163;
pub const SWAPON: usize = 5162;
pub const SYMLINK: usize = 5086;
pub const SYMLINKAT: usize = 5256;
pub const SYNC: usize = 5157;
pub const SYNC_FILE_RANGE: usize = 5264;
pub const SYNCFS: usize = 5301;
pub const SYSFS: usize = 5136;
pub const SYSINFO: usize = 5097;
pub const SYSLOG: usize = 5101;
pub const TEE: usize = 5265;
pub const TGKILL: usize = 5225;
pub const TIMER_CREATE: usize = 5216;
pub const TIMER_DELETE: usize = 5220;
pub const TIMER_GETOVERRUN: usize = 5219;
pub const TIMER_GETTIME: usize = 5218;
pub const TIMER_SETTIME: usize = 5217;
pub const TIMERFD_CREATE: usize = 5280;
pub const TIMERFD_GETTIME: usize = 5281;
pub const TIMERFD_SETTIME: usize = 5282;
pub const TIMES: usize = 5098;
pub const TKILL: usize = 5192;
pub const TRUNCATE: usize = 5074;
pub const UMASK: usize = 5093;
pub const UMOUNT2: usize = 5161;
pub const UNAME: usize = 5061;
pub const UNLINK: usize = 5085;
pub const UNLINKAT: usize = 5253;
pub const UNSHARE: usize = 5262;
pub const USERFAULTFD: usize = 5317;
pub const USTAT: usize = 5133;
pub const UTIME: usize = 5130;
pub const UTIMENSAT: usize = 5275;
pub const UTIMES: usize = 5226;
pub const VHANGUP: usize = 5150;
pub const VMSPLICE: usize = 5266;
pub const VSERVER: usize = 5236;
pub const WAIT4: usize = 5059;
pub const WAITID: usize = 5237;
pub const WRITE: usize = 5001;
pub const WRITEV: usize = 5019;
