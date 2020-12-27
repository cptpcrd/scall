#!/usr/bin/env python3

import os
import re
import tempfile
import subprocess
import sys

from typing import Iterator, Iterable, List, Set, Tuple

SIMPLE_MATH = re.compile(r"^[()+0-9a-fx\s]*$")
NUMBER = re.compile(r"[0-9a-fx]+")

NICE_ARCH_NAMES = {
    "x86_64": "x86-64",
    "powerpc": "PowerPC",
    "powerpc64": "PowerPC64",
    "mips": "MIPS",
    "mips64": "MIPS64",
    "sparc64": "SPARC64",
    "armeabi": "arm",
}


def load_table(
    linux_path: str, path: str, arches: Set[str]
) -> Iterator[Tuple[str, int]]:
    with open("{}/{}".format(linux_path, path)) as f:
        for line in f:
            line = line.strip()
            if line.startswith("#") or not line:
                continue

            nr, arch, name = line.split("\t", 4)[0:3]
            if arch in arches:
                yield (name, int(nr))


def eval_expr(expr: str) -> int:
    if not SIMPLE_MATH.match(expr):
        raise Exception(
            '"{}" looks like an expression, but not a supported one'.format(expr)
        )

    return sum(int(x.group(0), 0) for x in NUMBER.finditer(expr))


def load_headers(
    linux_path: str, names: Iterable[Tuple[str, str]], arch: str, extra: str = ""
) -> Iterator[Tuple[str, int]]:
    with tempfile.NamedTemporaryFile(mode="w+", suffix=".h") as f:
        with tempfile.TemporaryDirectory() as temp_include_dir:
            os.mkdir("{}/asm".format(temp_include_dir))
            # Create empty asm/unistd-eabi.h and asm/unistd-common.h because
            # the ARM asm/unistd.h header needs them.
            with open("{}/asm/unistd-eabi.h".format(temp_include_dir), "w"):
                pass
            with open("{}/asm/unistd-common.h".format(temp_include_dir), "w"):
                pass

            f.write(extra)
            f.write("\n")
            f.write("#include <asm/unistd.h>\n")
            for prefix, name in names:
                if prefix is None:
                    prefix = ""
                f.write(
                    "gen_nr {prefix}{name} __{prefix}NR_{name}\n".format(
                        prefix=prefix, name=name
                    )
                )
            f.flush()

            lines = subprocess.run(
                [
                    "gcc",
                    "-nostdinc",
                    "-I",
                    "{}/arch/{}/include/uapi".format(linux_path, arch),
                    "-I",
                    "{}/arch/{}/include/generated/uapi".format(linux_path, arch),
                    "-I",
                    "{}/include".format(linux_path),
                    "-I",
                    "{}/include/uapi".format(linux_path),
                    "-I",
                    temp_include_dir,
                    "-P",  # don't include line number markers, which make the output annoying to parse
                    "-E",  # only preprocess, don't compile
                    f.name,
                ],
                check=True,
                encoding="utf-8",
                stdout=subprocess.PIPE,
            ).stdout.split("\n")

    for line in lines:
        if line.startswith("gen_nr "):
            _, name, nr = line.split(" ", 2)
            if nr.startswith("__"):
                # unsupported on this arch
                continue
            yield (name, eval_expr(nr))


def main(args: List[str]) -> None:
    if len(args) != 1:
        print("Usage: {} <path to Linux kernel source>".format(sys.argv[0]))
        sys.exit(1)

    repo_path = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))

    linux_path = args[0]

    RE_SYSCALL_NR = re.compile(r"\b__([A-Z]+_)?NR_([a-z0-9_]+)\b")
    names = set(
        x.groups()
        for x in RE_SYSCALL_NR.finditer(
            subprocess.run(
                ["git", "--no-pager", "grep", r"\<__\([A-Z]\+_\)\?NR_"],
                cwd=linux_path,
                check=True,
                encoding="utf-8",
                stdout=subprocess.PIPE,
            ).stdout
        )
    )

    if len(names) < 380:
        print(
            "Didn't find anywhere near enough syscalls; hack must have failed",
            file=sys.stderr,
        )
        sys.exit(1)

    numbers = {
        "aarch64": dict(load_headers(linux_path, names, "arm64")),
        "armeabi": dict(
            list(
                load_table(linux_path, "arch/arm/tools/syscall.tbl", {"common", "eabi"})
            )
            + list(load_headers(linux_path, names, "arm", "#define __ARM_EABI__"))
        ),
        "mips": dict(
            load_headers(linux_path, names, "mips", "#define _MIPS_SIM _MIPS_SIM_ABI32")
        ),
        "mips64": dict(
            load_headers(linux_path, names, "mips", "#define _MIPS_SIM _MIPS_SIM_ABI64")
        ),
        "powerpc": dict(
            load_headers(linux_path, names, "powerpc", "#undef __arch64__")
        ),
        "powerpc64": dict(
            load_headers(
                linux_path,
                names,
                "powerpc",
                "#define __arch64__ 1\n#define __powerpc64__",
            )
        ),
        "sparc64": dict(load_headers(linux_path, names, "sparc")),
        "x86": dict(
            load_table(linux_path, "arch/x86/entry/syscalls/syscall_32.tbl", {"i386"})
        ),
        "x86_64": dict(
            load_table(
                linux_path, "arch/x86/entry/syscalls/syscall_64.tbl", {"common", "64"}
            )
        ),
    }

    for arch, nums in numbers.items():
        if not nums:
            continue

        with open("{}/src/platform/linux-{}/nr.rs".format(repo_path, arch), "w") as f:
            f.write(
                "//! System call numbers for {} Linux.\n\n".format(
                    NICE_ARCH_NAMES.get(arch, arch)
                )
            )
            f.write("/* automatically generated by nr_from_src.py */\n\n")
            for name, nr in sorted(nums.items()):
                f.write("pub const {}: usize = {};\n".format(name.upper(), nr))


if "__main__" == __name__:
    main(sys.argv[1:])
