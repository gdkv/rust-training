// Написать функцию, принимающую слайс generic-объектов и выводящую на экран все элементы этого
// слайса. Продемонстрировать, что данная функция работает с любыми типами, реализующими
// необходимый трейт.
//
// Для вывода объектов на экран можно использовать трейт Printable из примеров выше. Данный трейт
// потребуется реализовать на типах, которые будут использоваться при демонстрации.
#[derive(Debug)]
struct Media<'a> {
    id: u8,
    title: String,
    author: String,
    cost: i32,
    is_active: bool,
    length: u64,
    description: String,
    category: &'a Category,
}

#[derive(Debug)]
struct Category {
    id: u8,
    title: String,
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Catalog {
        Catalog { items: vec![] }
    }
    fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }
    fn get_item_by_id(&self, id: u8) -> Option<&Media> {
        self.items.iter().find(|item| item.id == id)
    }
}
fn main() {
    let category = Category {
        id: 1,
        title: String::from("test cat"),
    };

    let media_1 = Media::new();

    let media_2 = Media {
        id: 2,
        title: String::from("Test 22 Media"),
        author: String::from("Test 2 Author"),
        cost: 2000,
        is_active: false,
        length: 500,
        description: String::from("Test 22 Description"),
        category: ,
    };

    let media_3 = Media {
        id: 3,
        title: String::from("Test 33 Media"),
        author: String::from("Author 3"),
        cost: 1400,
        is_active: true,
        length: 5100,
        description: String::from("Test 33 Description"),
        category: ,
    };

    let mut catalog = Catalog::new();

    catalog.add_item(media_1);
    catalog.add_item(media_2);
    catalog.add_item(media_3);

    // println!("{:#?}", catalog.get_item_by_id(122));

    match catalog.get_item_by_id(122) {
        Some(media_item) => println!("{:#?}", media_item),
        None => println!("Nothing found"),
    }
}