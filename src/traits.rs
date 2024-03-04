use crate::model::TableRow;

#[async_trait::async_trait]
pub trait FetchAndTransform {
    async fn fetch_data(&self, ids: &str) -> Result<String, reqwest::Error>;
    async fn transform_data(&self, data: &str) -> Result<Vec<TableRow>, serde_json::Error>;
}
