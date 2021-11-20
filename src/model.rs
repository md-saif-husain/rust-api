use serde::{ Serialize, Deserialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}
