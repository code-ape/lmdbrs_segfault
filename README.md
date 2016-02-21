# lmdbrs segfault
Example project that uses lmdb and reproduces mysterious segfault when run.

I've encountered a rather bizarre segfault occurrence that is somehow linked to [lmdb-rs](https://github.com/vhbit/lmdb-rs) by [vhbit](https://github.com/vhbit). The segfault is odd and manifests in strangle places. In my project that I currently am experiencing issues with this it generally manifests when trying to transform data read from the db or move an `Arc` to the environment into a new thread. You can [check out the code here](https://github.com/code-ape/ComposeDB/tree/issue_06).

I created this repository purely for reference in helping solve this issue, which I have filed an issue for [here](https://github.com/vhbit/lmdb-rs/issues/29).
