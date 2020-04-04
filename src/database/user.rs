use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;

use crate::database::model::{NewUser, User};
use crate::diesel::RunQueryDsl;

pub fn get_all_users(
    conn: &MysqlConnection,
) -> QueryResult<Vec<User>> {
    use crate::database::schema::user::dsl::*;
    
    user.select((uid, service_name, user_name, point)).load(conn)
}

pub fn create_user(
    conn: &MysqlConnection,
    uid: String,
    service_name: String,
    user_name: String,
) -> QueryResult<User> {
    let new_user = NewUser::new_oauth(uid, service_name, user_name);
    {
        use crate::database::schema::user::dsl::*;

        diesel::insert_into(user).values(&new_user).execute(conn)?;

        user.find(uid).select((uid, service_name, user_name, point)).get_result(conn)
    }
}

pub fn create_local_user(
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

pub fn find_local_user(
    conn: &MysqlConnection,
    user_name: &str,
    password: &str,
) -> QueryResult<Option<User>> {
    use crate::database::schema::user::dsl::{self, user};
    use zeroize::Zeroizing;

    let salt: Zeroizing<_> = user
        .select(dsl::salt)
        .filter(dsl::user_name.eq(user_name))
        .first::<Option<String>>(conn)
        .optional()?
        .flatten()
        .into();

    if let Some(salt) = salt.as_ref() {
        let password_hash = Zeroizing::new(argon2rs::argon2i_simple(password, salt));
        let result = user
            .filter(dsl::user_name.eq(user_name))
            .filter(dsl::salt.eq(salt))
            .filter(dsl::password.eq(Some(password_hash.as_ref())))
            .select((dsl::uid, dsl::service_name, dsl::user_name, dsl::point))
            .first(conn)
            .optional()?;
        Ok(result)
    } else {
        Ok(None)
    }
}
