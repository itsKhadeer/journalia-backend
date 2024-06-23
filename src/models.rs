use super::schema::*;
use serde::Serialize;

// Custom SQL types
#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Clone, PartialEq, Copy)]
#[DieselTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    Admin,
    Moderator,
    Writer,
    User,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Clone, PartialEq, Copy)]
#[DieselTypePath = "crate::schema::sql_types::VoteValue"]
pub enum VoteValue {
    Up,
    Down,
}

// Articles table
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Article {
    pub article_id: i32,
    pub topic_id: i32,
    pub writer_id: i32,
    pub title: String,
    pub image_url: Option<String>,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle<'a> {
    pub topic_id: &'a i32,
    pub writer_id: &'a i32,
    pub title: &'a str,
    pub image_url: Option<&'a str>,
    pub content: &'a str,
    pub created_at: &'a chrono::NaiveDateTime,
    pub updated_at: &'a chrono::NaiveDateTime,
}

// Comments table
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Comment {
    pub comment_id: i32,
    pub article_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub article_id: &'a i32,
    pub user_id: &'a i32,
    pub content: &'a str,
    pub created_at: &'a chrono::NaiveDateTime,
    pub updated_at: &'a chrono::NaiveDateTime,
}

// Topics table
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Topic {
    pub topic_id: i32,
    pub topic_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = topics)]
pub struct NewTopic<'a> {
    pub topic_name: &'a str,
}

// Users table
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
    pub phone: String,
    pub access_token: Option<String>,
    pub role: UserRole,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub access_token: Option<&'a str>,
    pub role: UserRole,
}

// Votes table
// UpdateUser table
#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub user_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub role: Option<UserRole>,
}
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Vote {
    pub vote_id: i32,
    pub article_id: i32,
    pub user_id: i32,
    pub vote_type: VoteValue,
}

#[derive(Insertable)]
#[diesel(table_name = votes)]
pub struct NewVote<'a> {
    pub article_id: &'a i32,
    pub user_id: &'a i32,
    pub vote_type: VoteValue,
}
