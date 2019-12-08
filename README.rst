Package manager and build tool for C/C++
---

Inspired by Rust's Cargo package manager and build tool.

Aim -- make building cross-platform apps less painful.
Main usage scenario is building a cross-platform app where you may need to
resort to a native IDE to deploy the final product (ios, windows...).

Cases to cover, make things easy for:

    - use platform provided DLLs for one platform and use a custom-built one
      for the other
    - install tools from source packages locally
    - manage third-party binary libraries without source code
    - provide all flags/sources info to use with external tools or manually
      entered into the IDE for code assist etc.


Package kind:

    - source
        - requires build step, is rebuilt automatically per project
        - standard (all flags and binaries are exported automatically)
        - custom (runs a specified build script, flags, binaries are exported
            via the script)
    - binary or configuration only (e.g. flags to link against a system
      library)

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
