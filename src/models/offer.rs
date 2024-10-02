#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "offers"]
pub struct Offer {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: f64,
}