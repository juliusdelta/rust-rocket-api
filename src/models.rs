use schema::movies;

#[derive(Queryable)]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub rating: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="movies"]
pub struct NewMovie {
    pub title: String,
    pub director: String,
    pub rating: String,
}
