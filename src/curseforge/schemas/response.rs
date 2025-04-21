use super::pagination::Pagination;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataResponse<D> {
    pub data: D,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListResponse<D> {
    pub data: Vec<D>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginationResponse<D> {
    pub data: Vec<D>,
    pub pagination: Pagination,
}
