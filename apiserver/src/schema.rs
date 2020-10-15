table! {
    users (id) {
        id -> Uuid,
        user_email -> Varchar,
        pass_word -> Varchar,
        salt -> Varchar,
        nick_name -> Varchar,
        role_level -> Int2,
        sign_up_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);
