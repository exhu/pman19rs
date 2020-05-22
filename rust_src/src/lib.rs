#[cfg(test)]
mod tests {
    use crate::pman19rsold::{Tags, ForeignFlags, TaggedItems, TaggedData};
    #[test]
    fn tags_contained_in() {
        let a = Tags {tags: vec![String::from("ab"), String::from("cd")]};
        let b = Tags {tags: vec![String::from("ab"), String::from("cd"), String::from("efff")]};

        assert_eq!(a.contained_in(&b), true);
        assert_eq!(b.contained_in(&a), false);
        assert_eq!(a.tags.len(), 2);
        assert_eq!(b.tags.len(), 3);
    }

    #[test]
    fn filter_items() {
        let mut a = TaggedItems::<ForeignFlags>::new();
        a.items.push(TaggedData { tags: Tags { tags: vec![String::from("abc"), String::from("efggg")] }, data: ForeignFlags::new() });
        let filtered = a.filter_items(&Tags { tags: vec![String::from("abc"), String::from("efggg")]});
        assert_eq!(filtered.items.len(), 1);
        println!("{:#?}", filtered);
        let filtered = a.filter_items(&Tags { tags: vec![String::from("abc")]});
        assert_eq!(filtered.items.len(), 0);
        println!("{:#?}", filtered);
    }
}

pub mod pman19rsold {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Tags {
        pub tags: Vec<String>
    }

    impl Tags {
        pub fn contained_in(&self, other: &Tags) -> bool {
            self.tags.iter().all(|item| other.tags.contains(&item))
        }

    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct TaggedData<D> {
        pub tags: Tags,
        pub data: D,
    }

    impl<D> TaggedData<D> {
        pub fn satisfies_filter(&self, filter: &Tags) -> bool {
            self.tags.contained_in(filter)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct TaggedItems<D> {
        pub items: Vec<TaggedData<D>>
    }

    impl<D: Clone> TaggedItems<D> {
        pub fn new() -> TaggedItems<D> {
            TaggedItems::<D> { items: Vec::new() }
        }

        pub fn filter_items(&self, filter: &Tags) -> TaggedItems<D> {
            let items = TaggedItems { items: self.items.iter().filter(|item| item.satisfies_filter(filter)).cloned().collect()};
            items
        }
    }

    // native compiler and linker flags, 'exports' section
    #[derive(Debug, Deserialize, Clone)]
    pub struct ForeignFlags {
        pub c_flags: String,
        pub linker_flags: String,
    }

    impl ForeignFlags {
        pub fn new() -> ForeignFlags {
            ForeignFlags { c_flags: String::new(), linker_flags: String::new() }
        }
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


pub mod pman19rs {
    pub struct Tags {
        pub tags: Vec<String>
    }

    impl Tags {
        pub fn contained_in(&self, other: &Tags) -> bool {
            self.tags.iter().all(|item| other.tags.contains(&item))
        }
    }
}
