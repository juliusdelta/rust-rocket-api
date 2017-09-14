use schema::movies;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub rating: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="movies"]
pub struct NewMovie< 'a > {
    pub title: &'a str,
    pub director: &'a str,
    pub rating: &'a str,
}
