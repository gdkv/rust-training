use super::media::Media;

#[derive(Debug)]
pub struct Catalog<'a> {
    pub items: Vec<Media<'a>>,
}

impl<'a> Catalog<'a> {
    pub fn new() -> Catalog<'a> {
        Catalog { items: vec![] }
    }
    pub fn add_item(&mut self, item: Media<'a>) {
        self.items.push(item);
    }
    pub fn get_item_by_id(&self, id: u8) -> Option<&Media<'a>> {
        self
            .items
            .iter()
            .find(|item| item.id == id)
    }
}