#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Pagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i64,
}
