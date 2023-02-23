// @generated automatically by Diesel CLI.

diesel::table! {
    program (id) {
        id -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project (id) {
        id -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    round (id) {
        id -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    vote (id) {
        id -> Text,
        createdAt -> Text,
        amount -> Text,
        from -> Text,
        to -> Text,
        token -> Text,
        version -> Text,
        projectId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    program,
    project,
    round,
    vote,
);
