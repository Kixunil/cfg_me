cfg\_me
=======

File generator for `configure_me`.

About
-----

This tool takes care of auto-generating man pages (and other files in the future) for
projects using [`configure_me`](https://github.com/Kixunil/configure_me).

The whole thing should be self-explanatory. Just download it and run `cargo run man`. It will automatically generate and show the man page for itself. If you want to generate the man page for a different crate, just run the `cfg_me` binary while being in the working directory of your crate.

You can save the man page by running with `--output FILE` (or `-o`). If you happen to find an old crate which doesn't indicate specification location in `Catgo.toml`, you will have to use `--input` (`-i`) option.

Tools
-----

- [x] man pages
- [ ] bash completions
- [ ] debconf
- [ ] HTML forms
- [ ] HTML documentation

License
-------

MITNFA
