mod content;
use content::media::Media;
use content::category::Category;
use content::catalog::Catalog;

// Написать функцию, принимающую слайс generic-объектов и выводящую на экран все элементы этого
// слайса. Продемонстрировать, что данная функция работает с любыми типами, реализующими
// необходимый трейт.
//
// Для вывода объектов на экран можно использовать трейт Printable из примеров выше. Данный трейт
// потребуется реализовать на типах, которые будут использоваться при демонстрации.
fn main() {
    let media_1 = Media {
        id: 1,
        title: String::from("Test 1 Media"),
        author: String::from("Test 1 Author"),
        cost: 100,
        is_active: false,
        length: 500,
        description: String::from("Test 1 Description"),
        category: Category {
            id: 1,
            title: String::from("test cat"),
        },
    };

    let media_2 = Media {
        id: 2,
        title: String::from("Test 22 Media"),
        author: String::from("Test 2 Author"),
        cost: 2000,
        is_active: false,
        length: 500,
        description: String::from("Test 22 Description"),
        category: Category {
            id: 2,
            title: String::from("test cat"),
        },
    };

    let media_3 = Media {
        id: 3,
        title: String::from("Test 33 Media"),
        author: String::from("Author 3"),
        cost: 1400,
        is_active: true,
        length: 5100,
        description: String::from("Test 33 Description"),
        category: Category {
            id: 3,
            title: String::from("test cat"),
        },
    };

    let mut catalog = Catalog::new();

    catalog.add_item(media_1);
    catalog.add_item(media_2);
    catalog.add_item(media_3);

    // println!("{:#?}", catalog.get_item_by_id(122));

    match catalog.get_item_by_id(1) {
        Some(media_item) => println!("{:#?}", media_item),
        None => println!("Nothing found"),
    }
}