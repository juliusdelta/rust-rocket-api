#[derive(Queryable)]
#[derive(Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub rating: String,
}
