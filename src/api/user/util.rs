use super::InputUser;
use crate::api::RedisConn;
use crate::error::DieselError;
use crate::models::NewUser;
use crate::models::UpdateUser;
use crate::models::User;
use crate::models::UserRole;
use crate::schema::users::role;
use crate::util::function;
use anyhow::Result;
use diesel::prelude::*;
use redis::Commands;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatsResponse {
    pub highest_attack_score: i32,
    pub highest_defense_score: i32,
    pub trophies: i32,
    pub position_in_leaderboard: i32,
    pub no_of_emps_used: i32,
    pub total_damage_defense: i32,
    pub total_damage_attack: i32,
    pub no_of_attackers_suicided: i32,
    pub no_of_attacks: i32,
    pub no_of_defenses: i32,
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    user_name: String,
    email: String,
    phone: String,
    role: UserRole,
}

pub fn fetch_user(conn: &mut PgConnection, player_id: i32) -> Result<Option<User>> {
    use crate::schema::users;
    Ok(users::table
        .filter(users::user_id.eq(player_id))
        .first::<User>(conn)
        .optional()
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?)
}

pub fn fetch_all_user(conn: &mut PgConnection) -> Result<Vec<User>> {
    use crate::schema::users;
    Ok(users::table
        // .order_by(user::trophies.desc())
        .load::<User>(conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?)
}

pub fn add_user(
    pg_conn: &mut PgConnection,
    mut redis_conn: RedisConn,
    user: &InputUser,
) -> anyhow::Result<()> {
    use crate::models::UserRole;
    use crate::schema::users;
    let new_user = NewUser {
        user_name: &user.user_name,
        email: &user.email,
        phone: &user.phone,
        access_token: None,
        role: UserRole::User,
    };
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(pg_conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?;
    // Set last reset password time as 0 for new user
    redis_conn.set(user.user_id, 0)?;
    Ok(())
}

pub fn update_user(conn: &mut PgConnection, user_id: i32, update_user: &UpdateUser) -> Result<()> {
    use crate::schema::users;
    diesel::update(users::table.find(user_id))
        .set(update_user)
        .execute(conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?;
    Ok(())
}

pub fn get_duplicate_users(conn: &mut PgConnection, user: &InputUser) -> Result<Vec<User>> {
    use crate::schema::users;
    let duplicates = users::table
        .filter(users::user_name.eq(&user.user_name))
        .load::<User>(conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?;
    Ok(duplicates)
}

pub fn get_duplicate_username(conn: &mut PgConnection, username: &str) -> Result<Option<User>> {
    use crate::schema::users;
    Ok(users::table
        .filter(users::user_name.eq(username))
        .first::<User>(conn)
        .optional()
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?)
}

pub fn make_profile_response(user: &User) -> Result<UserProfileResponse> {
    let profile = UserProfileResponse {
        user_name: user.user_name.clone(),
        email: user.email.clone(),
        phone: user.phone.clone(),
        role: user.role.clone(),
    };

    Ok(profile)
}
