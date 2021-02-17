#!/usr/bin/env python3
import os
import re
import sys
from typing import Dict, List, Optional

ERRNO_DEFINE_RE = re.compile(
    r"(\s*\/\*([^*]|\*[^/])*\*\/)*\s*(#define\s+(?P<name>[A-Z0-9_]+)(\s+((?P<num>\d+)|(?P<other>\w+)|(?P<extra>.*)))?|#undef\s+(?P<undef_name>[A-Z0-9_]+))(\s*/\*([^*]|\*[^/])*\*/)*\s+"
)

REQUIRED_ERRNOS = {
    "freebsd": set(
        "EPERM ENOENT EEXIST EISDIR ENOTDIR ESRCH EINTR EIO ENXIO E2BIG ENOEXEC EACCES EAGAIN EALREADY EBADF EBUSY ECHILD EDEADLK EFAULT EFBIG EINPROGRESS EINVAL ENOTBLK ENFILE EMFILE ENOTTY EXDEV ETXTBSY ENOSPC ESPIPE EROFS EMLINK EPIPE EDOM ERANGE ENOTSOCK EDESTADDRREQ EMSGSIZE EPROTOTYPE ENOPROTOOPT EPROTONOSUPPORT ESOCKTNOSUPPORT EOPNOTSUPP EPFNOSUPPORT EAFNOSUPPORT EADDRINUSE EADDRNOTAVAIL ENETDOWN ENETUNREACH ENETRESET ECONNABORTED ECONNRESET ENOBUFS EISCONN ENOTCONN ESHUTDOWN ETOOMANYREFS ETIMEDOUT ECONNREFUSED ELOOP ENAMETOOLONG EHOSTDOWN EHOSTUNREACH ENOTEMPTY EUSERS EDQUOT ESTALE EREMOTE ENOLCK ENOSYS EIDRM ENOMSG EOVERFLOW ECANCELED EILSEQ EBADMSG EMULTIHOP ENOLINK EPROTO ENOMEM ENODEV ENOATTR ENEEDAUTH EAUTH EFTYPE EPROGUNAVAIL EPROGMISMATCH EPROCUNAVAIL ERPCMISMATCH EBADRPC EPROCLIM EDOOFUS ENOTCAPABLE ECAPMODE ENOTRECOVERABLE EOWNERDEAD ENOTSUP EWOULDBLOCK EINTEGRITY".split()
    ),
    "macos": set(
        "EPERM ENOENT EEXIST EISDIR ENOTDIR ESRCH EINTR EIO ENXIO E2BIG ENOEXEC EACCES EAGAIN EALREADY EBADF EBUSY ECHILD EDEADLK EFAULT EFBIG EINPROGRESS EINVAL ENOTBLK ENFILE EMFILE ENOTTY EXDEV ETXTBSY ENOSPC ESPIPE EROFS EMLINK EPIPE EDOM ERANGE ENOTSOCK EDESTADDRREQ EMSGSIZE EPROTOTYPE ENOPROTOOPT EPROTONOSUPPORT ESOCKTNOSUPPORT EOPNOTSUPP EPFNOSUPPORT EAFNOSUPPORT EADDRINUSE EADDRNOTAVAIL ENETDOWN ENETUNREACH ENETRESET ECONNABORTED ECONNRESET ENOBUFS EISCONN ENOTCONN ESHUTDOWN ETOOMANYREFS ETIMEDOUT ECONNREFUSED ELOOP ENAMETOOLONG EHOSTDOWN EHOSTUNREACH ENOTEMPTY EUSERS EDQUOT ESTALE EREMOTE ENOLCK ENOSYS EIDRM ENOMSG EOVERFLOW ECANCELED EILSEQ EBADMSG EMULTIHOP ENOLINK EPROTO ENOMEM ENODEV ENOATTR ENEEDAUTH EAUTH EFTYPE EPROGUNAVAIL EPROGMISMATCH EPROCUNAVAIL ERPCMISMATCH EBADRPC EPROCLIM ENOPOLICY EQFULL EBADMACHO ESHLIBVERS EBADARCH EBADEXEC EDEVERR EPWROFF ENOTRECOVERABLE EOWNERDEAD ENODATA ENOTSUP ENOSTR ETIME EWOULDBLOCK ENOSR".split()
    ),
}

NICE_OS_NAMES = {
    "freebsd": "FreeBSD",
    "macos": "macOS",
}

NICE_ARCH_NAMES = {
    "x86_64": "x86-64",
    "powerpc": "PowerPC",
    "powerpc64": "PowerPC64",
    "mips": "MIPS",
    "mips64": "MIPS64",
    "sparc64": "SPARC64",
}


def read_errno_header_file(
    fpath: str, base: Optional[Dict[str, int]] = None
) -> Dict[str, int]:
    errnos = dict({} if base is None else base)

    text = ""
    with open(fpath) as file:
        for line in file:
            line = line.strip()
            if line and not line.startswith(
                (
                    "$",
                    ";",
                    "#include",
                    "#if",
                    "#ifdef",
                    "#else",
                    "#endif",
                    "__BEGIN_DECLS",
                    "__END_DECLS",
                    "typedef ",
                    "int ",
                    "extern int ",
                )
            ):

                text += line + "\n"

    scanner = ERRNO_DEFINE_RE.scanner(text)
    for match in iter(scanner.match, None):
        if match.group("extra"):
            continue

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

    errnos.pop("ELAST", None)
    return errnos


def main(args: List[str]) -> None:
    if len(args) != 2:
        print(
            "Usage: {} <errno.h file> <OS name>".format(sys.argv[0]),
            file=sys.stderr,
        )
        sys.exit(1)

    repo_path = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))

    errno_fname = args[0]
    os_name = args[1]

    arches = [
        name[len(os_name) + 1 :]
        for name in os.listdir(os.path.join(repo_path, "src/platform"))
        if name.startswith(os_name + "-")
    ]

    errnos = read_errno_header_file(errno_fname)

    if os_name == "macos":
        errnos.pop("ECVPERORR", None)
        errnos.pop("ECVCERORR", None)

    assert set(errnos.keys()) >= REQUIRED_ERRNOS[os_name], REQUIRED_ERRNOS[
        os_name
    ] - set(errnos.keys())

    for arch in arches:
        fpath = os.path.join(repo_path, "src/platform", os_name + "-" + arch, "eno.rs")

        with open(fpath, "w") as file:
            file.write(
                "//! Error numbers for {} {}.\n\n".format(
                    NICE_ARCH_NAMES.get(arch, arch), NICE_OS_NAMES[os_name]
                )
            )
            file.write("/* Automatically generated by bsd_eno_nr_from_src.py */\n\n")

            for name, eno in sorted(errnos.items()):
                file.write("pub const {}: i32 = {};\n".format(name, eno))


if __name__ == "__main__":
    main(sys.argv[1:])
