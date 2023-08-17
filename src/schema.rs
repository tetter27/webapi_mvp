// @generated automatically by Diesel CLI.

diesel::table! {
    adopts (id) {
        id -> Int8,
        projects_id -> Int8,
        technologies_id -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    projects (id) {
        id -> Int8,
        name -> Varchar,
        url_name -> Varchar,
    }
}

diesel::table! {
    technologies (id) {
        id -> Int8,
        name -> Varchar,
        url_name -> Varchar,
        image_url -> Varchar,
    }
}

diesel::joinable!(adopts -> projects (projects_id));
diesel::joinable!(adopts -> technologies (technologies_id));

diesel::allow_tables_to_appear_in_same_query!(
    adopts,
    projects,
    technologies,
);
