#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub url: url::Url,
    pub priority: u16,
    pub mirrors: Vec<Mirror>
}

#[derive(Debug)]
pub struct Mirror {
    pub location: chrono_tz::Tz,
    pub url: url::Url,
    pub priority: u16,
}