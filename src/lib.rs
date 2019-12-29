#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod pman19rs {
    use serde::Deserialize;
    use std::collections::{BTreeMap, BTreeSet};
    use std::cmp::{Ord, Ordering};

    #[derive(Eq)]
    pub struct Tags {
        tags: BTreeSet<String>
    }

    impl Tags {
        fn new(tag_line: String) -> Tags {
            let mut tags = Tags { tags: BTreeSet::new() };
            // TODO parse tag line
            return tags
        }
    }

    impl Ord for Tags {
        fn cmp(&self, other: &Self) -> Ordering {
            self.tags.cmp(&other.tags)
        }
    }

    impl PartialOrd for Tags {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for Tags {
        fn eq(&self, other: &Self) -> bool {
            self.tags == other.tags
        }
    }
    
    pub struct TagsToData<D> {
        data_map: BTreeMap<Tags, D>
    }

    impl<D> TagsToData<D> {
        pub fn new() -> TagsToData<D> {
            TagsToData::<D>{ data_map: BTreeMap::new() }
        }

        pub fn put_object_for_tags(&mut self, o: D, tags: Tags) {
            self.data_map.insert(tags, o);
        }
    }

    // native compiler and linker flags, 'exports' section
    #[derive(Debug, Deserialize)]
    pub struct ForeignFlags {
        pub c_flags: String,
        pub linker_flags: String,
    }

    #[derive(Debug, Deserialize)]
    pub enum PackageKind {
        Static { flags: BuildFlags },
        Dynamic { flags: BuildFlags },
        Foreign { flags: ForeignFlags },
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

    }
}