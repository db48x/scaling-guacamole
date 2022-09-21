table! {
    community (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}

table! {
    community_members (community_id, user_id) {
        community_id -> Integer,
        user_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        displayname -> Nullable<Text>,
    }
}

joinable!(community_members -> community (community_id));
joinable!(community_members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    community,
    community_members,
    users,
);
