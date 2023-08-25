CREATE TABLE adopts (
    id BIGSERIAL PRIMARY KEY,
    ---------
    projects_id BIGINT NOT NULL,
    FOREIGN KEY (projects_id) REFERENCES projects(id),
    technologies_id BIGINT NOT NULL,
    FOREIGN KEY (technologies_id) REFERENCES technologies(id),
    ---------
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);