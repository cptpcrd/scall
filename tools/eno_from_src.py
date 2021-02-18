#!/usr/bin/env python3
import os
import re
import sys
from typing import Dict, List, Optional

ERRNO_DEFINE_RE = re.compile(
    r"(\s*/\*([^*]|\*[^/])*\*/)*\s*(#define\s+(?P<name>[A-Z0-9_]+)(\s+((?P<num>\d+)|(?P<other>\w+)))?|#undef\s+(?P<undef_name>[A-Z0-9_]+))(\s*/\*([^*]|\*[^/])*\*/)*\s+"
)

NICE_ARCH_NAMES = {
    "x86_64": "x86-64",
    "powerpc": "PowerPC",
    "powerpc64": "PowerPC64",
    "mips": "MIPS",
    "mips64": "MIPS64",
    "sparc64": "SPARC64",
    "armeabi": "arm",
    "riscv64": "RISCV-64",
}

REQUIRED_ERRNOS = set(
    "EPERM ENOENT EEXIST EISDIR ENOTDIR ESRCH EINTR EIO ENXIO E2BIG ENOEXEC EACCES EAGAIN EALREADY EBADF EBUSY ECHILD EDEADLK EFAULT EFBIG EINPROGRESS EINVAL ENOTBLK ENFILE EMFILE ENOTTY EXDEV ETXTBSY ENOSPC ESPIPE EROFS EMLINK EPIPE EDOM ERANGE ENOTSOCK EDESTADDRREQ EMSGSIZE EPROTOTYPE ENOPROTOOPT EPROTONOSUPPORT ESOCKTNOSUPPORT EOPNOTSUPP EPFNOSUPPORT EAFNOSUPPORT EADDRINUSE EADDRNOTAVAIL ENETDOWN ENETUNREACH ENETRESET ECONNABORTED ECONNRESET ENOBUFS EISCONN ENOTCONN ESHUTDOWN ETOOMANYREFS ETIMEDOUT ECONNREFUSED ELOOP ENAMETOOLONG EHOSTDOWN EHOSTUNREACH ENOTEMPTY EUSERS EDQUOT ESTALE EREMOTE ENOLCK ENOSYS EIDRM ENOMSG EOVERFLOW ECANCELED EILSEQ EBADMSG EMULTIHOP ENOLINK EPROTO ENOMEM ENODEV EBADE EBADFD EBADR EBADRQC EBADSLT ECHRNG ECOMM EHWPOISON EISNAM EKEYEXPIRED EKEYREJECTED EKEYREVOKED ENOKEY EREMOTEIO EL2HLT EL2NSYNC EL3HLT EL3RST ELNRNG EUNATCH ENOCSI EXFULL ENOANO EBFONT ENOTNAM ERFKILL ENAVAIL EUCLEAN ESTRPIPE ELIBEXEC ELIBSCN ELIBMAX ELIBBAD ELIBACC EDOTDOT ERESTART ENOTUNIQ EADV ESRMNT ENOPKG ENONET EREMCHG ETIME ENODATA ENOSR ENOSTR ENOMEDIUM EMEDIUMTYPE ENOTRECOVERABLE EOWNERDEAD EDEADLOCK EWOULDBLOCK".split()
)


def read_errno_header_file(
    fpath: str, base: Optional[Dict[str, int]] = None
) -> Dict[str, int]:
    errnos = dict({} if base is None else base)

    text = ""
    with open(fpath) as file:
        for line in file:
            line = line.strip()
            if line and not line.startswith(
                ("$", ";", "#include", "#if", "#ifdef", "#else", "#endif")
            ):

                text += line + "\n"

    scanner = ERRNO_DEFINE_RE.scanner(text)
    for match in iter(scanner.match, None):
        undef_name = match.group("undef_name")
        if undef_name:
            assert undef_name.startswith("E")
            errnos.pop(undef_name)
            continue

        name = match.group("name")
        if not name.startswith("E"):
            continue

        other_name = match.group("other")
        if other_name:
            errnos[name] = errnos[other_name]
        else:
            errnos[name] = int(match.group("num"))

    errnos.pop("EMAXERRNO", None)
    return errnos


def read_arch_errnos(
    linux_path: str,
    arch: str,
    base: Dict[str, int],
    header_path: Optional[str] = None,
) -> Dict[str, int]:
    if header_path is None:
        header_path = (
            os.path.join(linux_path, "arch", arch, "include/uapi/asm/errno.h")
            if arch is not None
            else os.path.join(linux_path, "include/uapi/asm-generic/errno.h")
        )

    return read_errno_header_file(header_path, base)


def main(args: List[str]) -> None:
    if len(args) != 1:
        print(
            "Usage: {} <path to Linux kernel source>".format(sys.argv[0]),
            file=sys.stderr,
        )
        sys.exit(1)

    repo_path = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    linux_path = args[0]

    errno_base = read_errno_header_file(
        os.path.join(linux_path, "tools/include/uapi/asm-generic/errno-base.h")
    )

    kernel_arches = {
        "aarch64": None,
        "armeabi": None,
        "mips": "mips",
        "mips64": "mips",
        "powerpc": "powerpc",
        "powerpc64": "powerpc",
        "sparc64": "sparc",
        "riscv64": None,
        "x86": None,
        "x86_64": None,
    }

    karch_header_paths = {
        "mips": os.path.join(linux_path, "tools/arch/mips/include/uapi/asm/errno.h"),
    }

    karch_errnos = {None: read_arch_errnos(linux_path, None, errno_base, None)}

    # Read the powerpc-specific errno.h, but "base" it off of the "generic" header
    # (include/uapi/asm-generic/errno.h)
    karch_bases = {"powerpc": karch_errnos[None]}

    for karch in set(kernel_arches.values()) - {None}:
        karch_errnos[karch] = read_arch_errnos(
            linux_path,
            karch,
            karch_bases.get(karch, errno_base),
            karch_header_paths.get(karch, None),
        )

    for arch, karch in kernel_arches.items():
        errnos = karch_errnos[karch]

        assert set(errnos.keys()) >= REQUIRED_ERRNOS, "{}: {}".format(
            arch, REQUIRED_ERRNOS - set(errnos.keys())
        )

        with open("{}/src/platform/linux-{}/eno.rs".format(repo_path, arch), "w") as f:
            f.write(
                "//! Error numbers for {} Linux.\n\n".format(
                    NICE_ARCH_NAMES.get(arch, arch)
                )
            )
            f.write("/* automatically generated by eno_from_src.py */\n\n")
            for name, eno in sorted(errnos.items()):
                f.write("pub const {}: i32 = {};\n".format(name, eno))


if __name__ == "__main__":
    main(sys.argv[1:])
