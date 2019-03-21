table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    users (username) {
        username -> Text,
        password_hash -> Text,
        user_roles -> Array<Text>,
    }
}

allow_tables_to_appear_in_same_query!(posts, users,);
