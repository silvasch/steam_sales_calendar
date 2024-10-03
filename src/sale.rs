use chrono::Utc;

pub struct Sale<'a> {
    pub name: &'a str,
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
}

impl<'a> Sale<'a> {
    pub fn new(name: &'a str, start: chrono::DateTime<Utc>, end: chrono::DateTime<Utc>) -> Self {
        Self { name, start, end }
    }
}
