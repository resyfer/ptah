# Ptah

An easy-to-use, easy-to-understand build system for C. Named after the Egyptian god of building.

# How to use

Clone this and build this (needs Rust 2024 edition at least) :

```sh
git clone https://github.com/resyfer/ptah.git
cd ptah
cargo build --release
```

You will find the binary as `./target/release/ptah`, but you can install it by moving or copying to any directory present in your `PATH`:

```sh
mv ./target/release/ptah /usr/bin
```

To use it in a project, say:

```
$ tree
.
├── include
│   ├── a.h
│   └── b.h
└── src
    ├── 1
    │   └── d.c
    ├── 2
    │   └── e.c
    └── common
        ├── a.c
        └── b.c

6 directories, 7 files
```

```sh
cd path/to/your/project/root
touch config.json
```

then you need a json config:

```json
{
    "name": "hello",
    "version": "0.0.1",
    "compiler": "gcc",
    "executables": [
        {
            "name": "hello1",
            "src": [
                "src/common",
                "src/1"
            ],
            "include": [
                "include"
            ],
            "flags": [
                "-Wall",
                "-O3"
            ],
            "options": [
                {
                    "key": "-stdc",
                    "value": "17"
                }
            ]
        },
        {
            "name": "hello2",
            "src": [
                "src/common",
                "src/2"
            ],
            "include": [
                "include"
            ],
            "flags": [
                "-Wall",
                "-O3"
            ],
            "options": [
                {
                    "key": "-stdc",
                    "value": "17"
                }
            ]
        }
    ]
}
```

Result:
```sh
$ ptah build
        [BUILD] hello1
        [CC]: d.c
        [CC]: a.c
        [CC]: b.c
        [LINK]: hello1
        [BUILD] hello2
        [CC]: e.c
        [LINK]: hello2

$ tree
.
├── build
│   ├── hello1
│   ├── hello2
│   └── src
│       ├── 1
│       │   └── d.c.o
│       ├── 2
│       │   └── e.c.o
│       └── common
│           ├── a.c.o
│           └── b.c.o
├── config.json
├── include
│   ├── a.h
│   └── b.h
└── src
    ├── 1
    │   └── d.c
    ├── 2
    │   └── e.c
    └── common
        ├── a.c
        └── b.c

11 directories, 13 files
```

Use `ptah -h` for more info.

## TODO

Upcoming Features:
- [ ] Improving log messages for debugging.
- [ ] Tests.
- [ ] Custom LD Path.
- [ ] Custom Env Vars.
- [ ] Project Initializing template.
- [ ] YAML or JSON config format.
- [ ] Colors if in an interactive shell.
- [ ] Custom verbosity level.