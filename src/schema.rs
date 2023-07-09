// @generated automatically by Diesel CLI.


diesel::table! {
    videos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        removed -> Bool,
    }
}
