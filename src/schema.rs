// infer_schema!("dotenv:DATABASE_URL");

table! {
    checks (id) {
        id -> Int4,
        url -> Text,
        rate -> Int4,
        last_start -> Nullable<Timestamptz>,
        last_end -> Nullable<Timestamptz>,
        http_status -> Nullable<Int4>,
        state -> Nullable<Text>,
    }
}