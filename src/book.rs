use std::{fmt::Debug, fs::File, ops::Range};

use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Book {
    pub id: Id,
    pub title: Title,
    pub author: Author,
    pub edition: Edition,
    pub publication_date: PublicationDate,
    pub content: Content,
}

#[derive(Debug, Default)]
pub enum Content {
    #[default]
    Empty,
    Text(String),
    File(String),
}

#[derive(Debug, Default)]
pub enum PublicationDate {
    #[default]
    Unknown,
    LocalDate(DateTime<Local>),
    Year(i32),
    Around(Range<i32>),
}

#[derive(Debug, Default)]
pub enum Title {
    #[default]
    Unknown,
    Partial(String),
    Full(String),
}

#[derive(Debug, Default)]
pub struct Id(pub i32);

#[derive(Debug, Default)]
pub enum Author {
    #[default]
    Unknown,
    Singular(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Default)]
pub enum Edition {
    #[default]
    Original,
    Copy,
    Translation(String),
}
