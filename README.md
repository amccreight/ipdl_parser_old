# Rust IPDL parser

This is a parser and type checker for the [IPC Protocol Definition
Language
(IPDL)](https://developer.mozilla.org/en-US/docs/Mozilla/IPDL) used by
Firefox. It is written in the Rust programming language. The only real
user at the moment is [Searchfox](https://searchfox.org).

# Syncing the Rust parser with Firefox

* Get an up-to-date mozilla-central checkout of Firefox.

* Make sure the existing tests all pass with Firefox's IPDL parser. To
do this from your mozilla-central checkout, go to
$OBJDIR/ipc/ipdl/test and run make check. These tests are run as part
of the build now, so this shouldn't be an issue.

* Delete all files in the tests/ok/ and tests/error directories. Copy
the .ipdl and .ipdlh files from ipc/ipdl/test/ipdl/ok/ and
ipc/ipdl/test/ipdl/error/ from a mozilla-central checkout into the
corresponding directories you just deleted files from. Delete
PasyncMessageListed.ipdl, unknownIntrMessage.ipdl and
unknownSyncMessage.ipdl from the error/ directory, because these fail
due to the sync message checker, which has not been implemented in
Rust.

* You can run these tests with `cargo test`. It is likely that some
tests will fail.

* Look at the list of revisions for
[parser.py](https://hg.mozilla.org/mozilla-central/log/tip/ipc/ipdl/ipdl/parser.py)
and [type.py](https://hg.mozilla.org/mozilla-central/log/tip/ipc/ipdl/ipdl/type.py)
to find the set of revisions since the last time the Rust
IPDL parser was updated.

* For each such revision, make sure there is a front end test. You can
  do this by looking at the commit, and also by using SearchFox to see
if there are any .ipdl files in the test directory that contain the
feature. Some times tests are added in follow up bugs. These tests
should valid states and failure cases for anything added to
type.py. If a test is missing, please write one and land it in
mozilla-central.

* Once there are unit tests for every new feature, and all of them are
passing, you should make sure the parser works on all IPDL files in
the tree. See directions for that in make_test_command.py in this
directory.
