// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vote_value"))]
    pub struct VoteValue;
}

diesel::table! {
    articles (article_id) {
        article_id -> Int4,
        topic_id -> Int4,
        writer_id -> Int4,
        title -> Varchar,
        image_url -> Nullable<Varchar>,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Int4,
        article_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    topics (topic_id) {
        topic_id -> Int4,
        topic_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        access_token -> Nullable<Varchar>,
        role -> UserRole,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VoteValue;

    votes (vote_id) {
        vote_id -> Int4,
        article_id -> Int4,
        user_id -> Int4,
        vote_type -> VoteValue,
    }
}

diesel::joinable!(articles -> topics (topic_id));
diesel::joinable!(articles -> users (writer_id));
diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(votes -> articles (article_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(articles, comments, topics, users, votes,);
