table! {
    user (uid) {
        uid -> Varchar,
        service_name -> Text,
        user_name -> Text,
        point -> Integer,
        password -> Nullable<Binary>,
        salt -> Nullable<Char>,
    }
}

table! {
    word (id) {
        id -> Integer,
        pyo_je_eo -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    user,
    word,
);
