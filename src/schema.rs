table! {
    follows (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        follow_user_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        text -> Text,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        password_digest -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(follows, posts, users,);
