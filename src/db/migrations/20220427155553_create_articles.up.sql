-- Add up migration script here

CREATE TABLE articles (
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) PRIMARY KEY,
    description VARCHAR(1024) NOT NULL,
    body TEXT NOT NULL,
    tag_list jsonb NOT NULL DEFAULT '[]',
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
