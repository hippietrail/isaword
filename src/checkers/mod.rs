pub mod urban;

#[derive(Debug, Clone)]
pub struct CheckerResult {
    pub name: &'static str,
    pub result: Option<bool>,
    pub is_community: bool,
}
