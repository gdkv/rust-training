use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog { items: vec![] }
    }
    pub fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }
    pub fn get_item_by_id(&self, id: u8) -> Option<&Media> {
        self.items.iter().find(|item| item.id == id)
    }
}