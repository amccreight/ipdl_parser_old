#!/usr/bin/python3

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Test command generator for the IPDL parser.

# 1. Add the following code to ipc/ipdl/ipdl.py, do a build, and copy
# the output from the build, from INCLUDES to DONE, somewhere:
#
# print("INCLUDES")
# for i in includedirs:
#     print(i)
# print("FILES")
# for f in files:
#     print(f)
# print("DONE")
#
# 2. Adjust leading_text_example as necessary, if the log timestamp
# stuff has changed.
#
# 3. Run this script. You'll need to pass in the path your Firefox
# objdir as an argument, and pipe in the output from step 1. The
# objdir is needed to find IPDL files that have been preprocessed,
# such as PMediaTransport.ipdl. You can run the resulting command
# with bash or whatever.

import argparse
import pathlib
import sys


# Used to decide how many characters to chop off the start.
leading_text_example = " 0:02.39 "

in_include = False
in_files = False

start_trim = len(leading_text_example)


parser = argparse.ArgumentParser(description="Print out a command to parse Firefox IPDL files.")
parser.add_argument("objdir", metavar='OBJDIR', type=str,
                    help="Path to the objdir, to find the location of preprocessed IPDL files.")
args = parser.parse_args()

objdir = pathlib.Path(args.objdir)

if not objdir.exists():
    print("The objdir passed in should exist.")
    exit(-1)

print("cargo run --", end=' ')

for line in sys.stdin:
    line = line[start_trim:-1]
    if line.endswith("INCLUDES"):
        in_include = True
        continue
    if line.endswith("FILES"):
        assert in_include
        in_include = False
        in_files = True
        continue
    if line.endswith("DONE"):
        assert in_files
        break

    assert in_include or in_files

    file_path = pathlib.Path(line)
    if not file_path.exists():
        file_path = objdir / "ipc" / "ipdl" / line
        if not file_path.exists():
            print()
            print()
            print("Couldn't find", line, "either directly or in the objdir.")
            exit(-1)

    if in_include:
        print("-I", str(file_path), end=' ')
    elif in_files:
        print(str(file_path), end=' ')


print()
