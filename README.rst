Package manager and build tool for C/C++
----------------------------------------

Inspired by Rust's Cargo package manager and build tool, and pipenv.

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

    Example:
      1) zlib-system -- uses system installed zlib, adds only find_package()
    to the generated cmake project file.
      2) zlib-src with modern cmake directives -- downloaded and put to deps-src directory,
      used via add_subdirectory() cmake call.
      3) zlib-src-alien -- downloaded and put to deps-src directory, built and
      installed to deps-bin directory, added to project via find_package()
      4) win32-sdk -- downloaded to deps-src and unpacked to deps-bin,
        added via find_package()

So we have the following types of packages:
  - config-only (system)
  - src
  - src-alien
  - binary-only


Source packages can be native(src) and foreign(src-alien). Native are those that have a known
structure, are built and managed automatically. Foreign ones require
launching a custom build script.

Layout
~~~~~~

TODO change

::

  package_root (project root)
    - pman19.toml
    - assets (static resources)
    - assets_src (recommended name for resources to process, this dir not used by the build tool)
    - src
      - srcfileN.c (source files which are linked into static or dynamic lib)
      - bin (one source file = one executable binary)
        - srcfileM.c - executable source
    - tests (integration tests)
      - srcN.c (common code for tests)
      - bin
        - testM.c (test exec)
    - build (build artifacts, must be never put under VCS)
      - any files, common for all targets are put right here, e.g. preprocessed resources or generated code
      - debug -- directory for debug target artifacts
        - assets -- static + processed/generated resources
          - package_name
        - bin (executable files, DLLs copied here)
        - package_name (for dependencies as well, i.e. per package)
          - intermediate object files, static libs
      - release -- same layout as for the 'debug'
      - dependencies
        - package_name
          - same layout ast for package_root but without 'build' directory (only package sources)


pman19rs home directory or deps-bin
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

::

  home/pman19rs
    - assets
      - package_name (per installed package)
    - bin
      - pman19rs executable
      - installed packages executables and DLLs
    - installed
        - package_name.toml - lists source and version info


Backend
~~~~~~~

pman19rs generates a script that is executed by CMake/Ninja/Make.


Cmake backend
~~~~~~~~~~~~~

Generates a single project for current package AND dependencies.


Roadmap
~~~~~~~

::

  - native package can be compiled, linked
    - sections by tags (-all, -linux etc)
    - 'build' section support
    - library
    - executables
    - dynamic library

  - dependencies support
    - source path dependendcy on native package
    - build
    - DLL copy

  - 'exports' section support to propagate linker and compiler flags

  - foreign package support with only 'exports' section

  - copying support ('copy' section)
    - assets
    - foreign DLLs

  - build script support
    - script invocation and error handling
    - merging generated toml with package root toml

