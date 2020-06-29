table! {
    original_language_word (id) {
        id -> Integer,
        word_id -> Integer,
        language -> Varchar,
        original -> Text,
    }
}

table! {
    region_word (id) {
        id -> Integer,
        word_id -> Integer,
        region -> Varchar,
    }
}

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
        lemma -> Varchar,
        group_code -> Integer,
        group_order -> Integer,
        word_unit -> Varchar,
        word_type -> Varchar,
        category -> Nullable<Varchar>,
        definition -> Text,
        position -> Nullable<Varchar>,
        sense_type -> Varchar,
        space -> Varchar,
    }
}

joinable!(original_language_word -> word (word_id));
joinable!(region_word -> word (word_id));

allow_tables_to_appear_in_same_query!(
    original_language_word,
    region_word,
    user,
    word,
);
