use diesel::{ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};
use juniper::parser::Token;
use crate::database::model::User;

pub fn find(
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