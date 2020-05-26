use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::database::model::{NewUser, User};

pub fn create(
    conn: &MysqlConnection,
    user_name: String,
    password: &str,
) -> QueryResult<User> {
    use rand::{distributions::Alphanumeric, Rng};

    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect::<String>();
    let password_hash = argon2rs::argon2i_simple(password, &salt);

    let new_user_name = user_name.clone();
    let new_user = NewUser::new_local(user_name, password_hash, salt);
    {
        use crate::database::schema::user::dsl::*;

        diesel::insert_into(user).values(&new_user).execute(conn)?;

        user.filter(user_name.eq(new_user_name)).select((uid, service_name, user_name, point)).get_result(conn)
    }
}