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
    pub struct PackageRoot {
        pub name: String,
    }
}