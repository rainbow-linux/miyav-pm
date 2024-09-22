use semver::Version;

#[derive(Debug)]
pub struct IndexEntry {
    pub name: String,
    pub version: Version,
    pub location: String,
}