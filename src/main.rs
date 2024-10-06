mod book;
use std::{
    default,
    fs::{self, File},
};

use book::{Author, Content, Edition, Id, PublicationDate, Title};

use crate::book::Book;
fn main() {
    let book = Book {
        id: Id::default(),
        title: Title::Full("Gospel of Matthew".into()),
        author: Author::Singular("Matthew".into()),
        edition: Edition::Translation("KJV".into()),
        publication_date: PublicationDate::Around(60..130),
        content: Content::File("matthewkjv.txt".into()),
    };

    match &book.author {
        Author::Singular(author) => println!("This book is written by {author}"),
        _default => todo!(),
    }
    println!("The book's id is {}", &book.id.0);
    match &book.title {
        Title::Full(title) => println!("The book is titled {title}"),
        _default => todo!(),
    }
    match &book.content {
        Content::File(file) => println!(
            "{}",
            fs::read_to_string(file).expect("File matthewkjv.txt not found")
        ),
        _default => todo!(),
    }
}
