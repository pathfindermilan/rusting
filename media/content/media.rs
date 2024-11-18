#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String},
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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
