CREATE TABLE projects (
    id BIGSERIAL PRIMARY KEY,
    ---------
    name VARCHAR NOT NULL,
    url_name VARCHAR NOT NULL UNIQUE
);
