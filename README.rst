Package manager for C/C++
-------------------------

Inspired by Rust's Cargo package manager and build tool.

Package kind:

    - source
        - requires build step, is rebuilt automatically per project
        - standard (all flags and binaries are exported automatically)
        - custom (runs specified build script, flags, binaries are exported
            via the script)
    - binary or configuration only
        (e.g. flags to link against system library)
        - precompiled binaries

Export (figured out automatically for source packages and must be explicitly
specified for binary ones):

    - compilation flags
    - link flags
    - resources to copy
    - shared libraries to copy
    - executable files to copy

Source packages can be native and foreign. Native are those that have a known
structure, are built and managed automatically. Foreign ones require
launching a custom build script.
