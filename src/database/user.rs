use crate::database::model::{NewUser, User};
use crate::diesel::RunQueryDsl;
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use diesel::QueryDsl;

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
