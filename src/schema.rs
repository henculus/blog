table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        author -> Text,
        published -> Bool,
    }
}

table! {
    users (username) {
        username -> Text,
        password_hash -> Text,
        user_roles -> Array<Text>,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
