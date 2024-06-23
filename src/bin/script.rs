use diesel::{prelude::*, update};
use journalia_backend::schema::user;
use journalia_backend::util;

fn main() {
    let pool = util::get_pg_conn_pool();
    let mut conn = pool.get().expect("Could not retrieve connection from pool");

    update(user::table)
        .set(user::trophies.eq(1000))
        .execute(&mut conn)
        .expect("Could not update user ratings");
}
