#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod pman19rs {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Tags {
        pub tags: Vec<String>
    }

    #[derive(Debug, Deserialize)]
    pub struct TaggedData<D> {
        pub tags: Tags,
        pub data: D,
    }

    #[derive(Debug, Deserialize)]
    pub struct TaggedItems<D> {
        pub items: Vec<TaggedData<D>>
    }

    // native compiler and linker flags, 'exports' section
    #[derive(Debug, Deserialize)]
    pub struct ForeignFlags {
        pub c_flags: String,
        pub linker_flags: String,
    }

    #[derive(Debug, Deserialize)]
    pub enum PackageKind {
        Static { flags: TaggedItems<BuildFlags> },
        Dynamic { flags: TaggedItems<BuildFlags> },
        Foreign { flags: TaggedItems<ForeignFlags> },
    }

    // 'build' section
    #[derive(Debug, Deserialize)]
    pub struct BuildFlags {
        // native gcc/msvc flags
        pub c_flags: Vec<String>,
        pub include_paths: Vec<String>,
        pub library_paths: Vec<String>,
        pub libraries: Vec<String>,
    }

    // 'copy' section
    #[derive(Debug, Deserialize)]
    pub struct Artifacts {
        pub assets: Vec<String>,
        pub binaries: Vec<String>,
        pub static_libs: Vec<String>,
        pub dynamic_libs: Vec<String>,
    }

    // root toml
    #[derive(Debug, Deserialize)]
    pub struct PackageRoot {
        pub name: String,
        pub kind: PackageKind,
        pub artifacts: TaggedItems<Artifacts>,

    }
}