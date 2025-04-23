use super::pagination::Pagination;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataResponse<D> {
    pub data: D,
}

pub type ListResponse<D> = DataResponse<Vec<D>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginationResponse<D> {
    pub data: Vec<D>,
    pub pagination: Pagination,
}
