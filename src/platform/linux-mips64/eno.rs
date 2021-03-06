//! Error numbers for MIPS64 Linux.

/* automatically generated by eno_from_src.py */

pub const E2BIG: i32 = 7;
pub const EACCES: i32 = 13;
pub const EADDRINUSE: i32 = 125;
pub const EADDRNOTAVAIL: i32 = 126;
pub const EADV: i32 = 68;
pub const EAFNOSUPPORT: i32 = 124;
pub const EAGAIN: i32 = 11;
pub const EALREADY: i32 = 149;
pub const EBADE: i32 = 50;
pub const EBADF: i32 = 9;
pub const EBADFD: i32 = 81;
pub const EBADMSG: i32 = 77;
pub const EBADR: i32 = 51;
pub const EBADRQC: i32 = 54;
pub const EBADSLT: i32 = 55;
pub const EBFONT: i32 = 59;
pub const EBUSY: i32 = 16;
pub const ECANCELED: i32 = 158;
pub const ECHILD: i32 = 10;
pub const ECHRNG: i32 = 37;
pub const ECOMM: i32 = 70;
pub const ECONNABORTED: i32 = 130;
pub const ECONNREFUSED: i32 = 146;
pub const ECONNRESET: i32 = 131;
pub const EDEADLK: i32 = 45;
pub const EDEADLOCK: i32 = 56;
pub const EDESTADDRREQ: i32 = 96;
pub const EDOM: i32 = 33;
pub const EDOTDOT: i32 = 73;
pub const EDQUOT: i32 = 1133;
pub const EEXIST: i32 = 17;
pub const EFAULT: i32 = 14;
pub const EFBIG: i32 = 27;
pub const EHOSTDOWN: i32 = 147;
pub const EHOSTUNREACH: i32 = 148;
pub const EHWPOISON: i32 = 168;
pub const EIDRM: i32 = 36;
pub const EILSEQ: i32 = 88;
pub const EINIT: i32 = 141;
pub const EINPROGRESS: i32 = 150;
pub const EINTR: i32 = 4;
pub const EINVAL: i32 = 22;
pub const EIO: i32 = 5;
pub const EISCONN: i32 = 133;
pub const EISDIR: i32 = 21;
pub const EISNAM: i32 = 139;
pub const EKEYEXPIRED: i32 = 162;
pub const EKEYREJECTED: i32 = 164;
pub const EKEYREVOKED: i32 = 163;
pub const EL2HLT: i32 = 44;
pub const EL2NSYNC: i32 = 38;
pub const EL3HLT: i32 = 39;
pub const EL3RST: i32 = 40;
pub const ELIBACC: i32 = 83;
pub const ELIBBAD: i32 = 84;
pub const ELIBEXEC: i32 = 87;
pub const ELIBMAX: i32 = 86;
pub const ELIBSCN: i32 = 85;
pub const ELNRNG: i32 = 41;
pub const ELOOP: i32 = 90;
pub const EMEDIUMTYPE: i32 = 160;
pub const EMFILE: i32 = 24;
pub const EMLINK: i32 = 31;
pub const EMSGSIZE: i32 = 97;
pub const EMULTIHOP: i32 = 74;
pub const ENAMETOOLONG: i32 = 78;
pub const ENAVAIL: i32 = 138;
pub const ENETDOWN: i32 = 127;
pub const ENETRESET: i32 = 129;
pub const ENETUNREACH: i32 = 128;
pub const ENFILE: i32 = 23;
pub const ENOANO: i32 = 53;
pub const ENOBUFS: i32 = 132;
pub const ENOCSI: i32 = 43;
pub const ENODATA: i32 = 61;
pub const ENODEV: i32 = 19;
pub const ENOENT: i32 = 2;
pub const ENOEXEC: i32 = 8;
pub const ENOKEY: i32 = 161;
pub const ENOLCK: i32 = 46;
pub const ENOLINK: i32 = 67;
pub const ENOMEDIUM: i32 = 159;
pub const ENOMEM: i32 = 12;
pub const ENOMSG: i32 = 35;
pub const ENONET: i32 = 64;
pub const ENOPKG: i32 = 65;
pub const ENOPROTOOPT: i32 = 99;
pub const ENOSPC: i32 = 28;
pub const ENOSR: i32 = 63;
pub const ENOSTR: i32 = 60;
pub const ENOSYS: i32 = 89;
pub const ENOTBLK: i32 = 15;
pub const ENOTCONN: i32 = 134;
pub const ENOTDIR: i32 = 20;
pub const ENOTEMPTY: i32 = 93;
pub const ENOTNAM: i32 = 137;
pub const ENOTRECOVERABLE: i32 = 166;
pub const ENOTSOCK: i32 = 95;
pub const ENOTTY: i32 = 25;
pub const ENOTUNIQ: i32 = 80;
pub const ENXIO: i32 = 6;
pub const EOPNOTSUPP: i32 = 122;
pub const EOVERFLOW: i32 = 79;
pub const EOWNERDEAD: i32 = 165;
pub const EPERM: i32 = 1;
pub const EPFNOSUPPORT: i32 = 123;
pub const EPIPE: i32 = 32;
pub const EPROTO: i32 = 71;
pub const EPROTONOSUPPORT: i32 = 120;
pub const EPROTOTYPE: i32 = 98;
pub const ERANGE: i32 = 34;
pub const EREMCHG: i32 = 82;
pub const EREMDEV: i32 = 142;
pub const EREMOTE: i32 = 66;
pub const EREMOTEIO: i32 = 140;
pub const ERESTART: i32 = 91;
pub const ERFKILL: i32 = 167;
pub const EROFS: i32 = 30;
pub const ESHUTDOWN: i32 = 143;
pub const ESOCKTNOSUPPORT: i32 = 121;
pub const ESPIPE: i32 = 29;
pub const ESRCH: i32 = 3;
pub const ESRMNT: i32 = 69;
pub const ESTALE: i32 = 151;
pub const ESTRPIPE: i32 = 92;
pub const ETIME: i32 = 62;
pub const ETIMEDOUT: i32 = 145;
pub const ETOOMANYREFS: i32 = 144;
pub const ETXTBSY: i32 = 26;
pub const EUCLEAN: i32 = 135;
pub const EUNATCH: i32 = 42;
pub const EUSERS: i32 = 94;
pub const EWOULDBLOCK: i32 = 11;
pub const EXDEV: i32 = 18;
pub const EXFULL: i32 = 52;
