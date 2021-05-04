table! {
    hilights (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    results (id) {
        id -> Unsigned<Bigint>,
        winner_id -> Bigint,
        loser_id -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(
    hilights,
    posts,
    results,
);
