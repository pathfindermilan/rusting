#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String},
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author} => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie {title, director} => {
                format!("Book: {} {}", title, director)
            },
            Media::Audiobook { title} => {
                format!("AudioBook {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast episode number {}", episode_number)
            }
            Media::Placeholder => {
                format!("This is a placeholder")
            }
        }
    }
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }
    fn add(&mut self, media: Media) {
        self.items.push(media)
    }
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main(){
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director")
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.get_by_index(10) {
        Some(value) => {
            println!("Item {:#?}", value);
        }
        None => {
            println!("No value")
        }
    }
}