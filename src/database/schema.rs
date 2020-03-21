use diesel::table;

table! {
    user (uid) {
        uid -> VarChar,
        service_name -> Text,
        user_name -> Text,
        point -> Integer,
    }
}

table! {
    word (id) {
        id -> Integer,
        pyo_je_eo -> Text,
    }
}
