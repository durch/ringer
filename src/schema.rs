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

table! {
    check_runs (id) {
        id -> Int4,
        check_id -> Int4,
        starttime -> Timestamptz,
        endtime -> Timestamptz,
        latency -> Int4,
        http_status -> Int4,
    }
}