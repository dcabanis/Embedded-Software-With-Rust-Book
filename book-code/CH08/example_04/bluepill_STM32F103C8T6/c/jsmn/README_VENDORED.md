# Vendored library

`jsmn.h` is vendored unmodified from [zserge/jsmn](https://github.com/zserge/jsmn)
(MIT licensed, see `LICENSE`) — a minimal, dependency-free JSON tokenizer
widely used in embedded/IoT C code. It parses in a single pass into a
caller-supplied, fixed-size array of tokens with no dynamic allocation at
all, which is what makes it a natural fit for `no_std` firmware.

`jsmn.h` is a single-header library: including it normally (as
`../jsmn_impl.c` does) pulls in the full implementation with external
linkage; defining `JSMN_HEADER` first (as `../wrapper.h` does) exposes
only the declarations, which is what *bindgen* is pointed at.
