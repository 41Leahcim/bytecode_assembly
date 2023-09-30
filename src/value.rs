#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Number(i64),
}
