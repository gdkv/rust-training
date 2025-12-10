use super::category::Category;
#[derive(Debug)]
pub struct Media {
    pub id: u8,
    pub title: String,
    pub author: String,
    pub cost: i32,
    pub is_active: bool,
    pub length: u64,
    pub description: String,
    pub category: Category,
}