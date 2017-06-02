// infer_schema!("dotenv:DATABASE_URL");
table! {
    checks (id) {
        id -> Int4,
        url -> Text,
        rate -> Int4,
        last_start -> Nullable<Timestamptz>,
        last_end -> Nullable<Timestamptz>,
        http_status -> Nullable<Int4>,
        meta -> Nullable<Jsonb>,
        user_id -> Int4,
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

table! {
    sessions (id) {
        id -> Int4,
        ext_id -> Text,
        valid_until -> Timestamptz,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        pass -> Text,
        created -> Timestamptz,
        updated -> Nullable<Timestamptz>,
    }
}