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
  - src (configured and built with the main project)
  - src-alien (built and installed into deps-bin before configuring the project)
  - binary-only (unpacked to deps-bin before configuring the main project)


Source packages can be native(src) and foreign(src-alien). Native are those that have a known
structure, are built and managed automatically. Foreign ones require
launching a custom build script.



Build tags
~~~~~~~~~~

Features of a build are specified via tags. Every project is configured to be
built with a certain set of tags. Some tags are predefined, e.g. 'windows',
'win64', 'linux', 'osx', 'android', 'debug', 'release', 'test', 'amd64',
'arm', 'arm64'. Others are user-defined, custom.

Custom tags are composite, and are prefixed with the project name, e.g.
'mylib-with-logs', where 'mylib' is the project name and 'with-logs' is a
project-related tag. This way one can specify a set of features for the
dependencies.

Sets of tags can be used to limit source files to be included in the build,
switch between dependencies, e.g. use appropriate binary packages for the
target platform or appropriate system libraries.

A source file or a dependency can be limited by sets of tags,
e.g. the sequence [{'linux', 'debug'}, {'osx', 'debug'}] means that the attached
source files will be included in the build only if both 'linux' and 'debug' tags
are in effect, or both 'osx' and 'debug' ones.


Layout
~~~~~~

::

  package_root (project root)
    - pman19.toml
    - app
      - srcfileN.c (one source file = one executable binary)
    - cmake - custom cmake files
    - examples
      - srcfileN.c (one source file = one executable binary)
    - assets (static resources)
    - assets_src (recommended name for resources to process, this dir not used by the build tool)
    - src
      - include/package_name - public header files for a library project
      - srcfileN.c, srcFile.h (source files which are linked into static or dynamic lib)
    - tests (integration tests)
      - srcN.c, srcN.h (common code for tests)
      - testM.c (test exec)
    - build (build artifacts, must be never put under VCS)
      - debug -- directory for debug target artifacts / cmake binary directory
        - cmake_src - generated cmake lists / cmake source directory
          - assets -- copied assets per package
            - package_name
        - cmake_out -- cmake binary dir, produced binaries
      - release -- same layout as for the 'debug'

    - dep-src -- fetched packages
      - package_name
        - same layout ast for package_root but without 'build' directory (only package sources)
    - dep-bin -- installed alien packages, or tools
      - [os_arch] - directory tree per cross arch., 'host' is the default name (runnable on build machine).
        - bin
        - include
        - lib


pman19rs home directory or deps-bin
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

::

  home/pman19rs
    - share/package_name/assets
    - bin
      - pman19rs executable
      - installed packages executables and DLLs
    - share/pman19rs/
        - installed.toml - lists source and version info per installed package


Backend
~~~~~~~

pman19rs generates a script that is executed by CMake.


Cmake backend
~~~~~~~~~~~~~

Generates a single project for current package AND dependencies.

There's a 'configure' step, which generates cmake files and build system files,
and optional 'build', 'install' steps. 

The configuration step uses a provided toml config files with tags and values.
This file is copied to the binary directory for reference or generated based on
passed command line arguments, so that repeated commands produce the same binary.


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

