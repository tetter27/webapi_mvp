CREATE TABLE technologies (
    id BIGSERIAL PRIMARY KEY,
    ---------
    name VARCHAR NOT NULL,
    url_name VARCHAR NOT NULL UNIQUE,
    image_url VARCHAR NOT NULL
);