pub mod field;

use serde::{Deserialize,Serialize};
use thiserror::Error;
extern crate uuid;

#[derive(Debug,Error)]
pub enum ClipError{
    #[error("invalid password : {0}")]
    InvalidPassword(String),
    #[error("invalid title : {0}")]
    InvalidTitle(String),
    #[error("invalid empty content")]
    EmptyContent,
    #[error("invalid date : {0}")]
    InvalidDate(String),
    #[error("invalid date  parse: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("invalid id: {0}")]
    Id(#[from] uuid::Error),
    #[error("invalid hits: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Clip {
    pub clip_id : field::ClipId,
    pub shortcode : field::ShortCode,
    pub content : field::Content,
    pub title : field::Title,
    pub posted : field::Posted,
    pub expires : field::Expires,
    pub password : field::Password,
    pub hits : field::Hits,
}