
diesel::table! {
    domain_inscription_info (id) {
        id -> BigInt,
        inscribe_num -> BigInt,
        inscribe_id -> Varchar,
        sat -> BigInt,
        domain_name -> Varchar,
        address -> Varchar,
        create_time -> BigInt,
        update_time -> BigInt,
        expire_date -> BigInt,
        register_date -> BigInt,
    }
}

diesel::table! {
    black_info (id) {
        id -> BigInt,
        inscribe_num -> BigInt,
        inscribe_id -> Varchar,
        create_time -> BigInt,
        update_time -> BigInt,
    }
}