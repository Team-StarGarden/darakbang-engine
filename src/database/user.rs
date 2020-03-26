use crate::database::model::{NewUser, User};
use crate::diesel::RunQueryDsl;
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};

pub fn create_user(
    conn: &MysqlConnection,
    uid: String,
    service_name: String,
    user_name: String,
) -> QueryResult<User> {
    let new_user = NewUser {
        uid,
        service_name,
        user_name,
    };
    {
        use crate::database::schema::user::dsl::*;

        diesel::insert_into(user).values(&new_user).execute(conn)?;

        user.find(uid).get_result(conn)
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
            .first(conn)
            .optional()?;
        Ok(result)
    } else {
        Ok(None)
    }
}
