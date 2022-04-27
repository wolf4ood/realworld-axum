-- Add up migration script here
CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    author_id UUID NOT NULL,
    article_id VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (article_id) REFERENCES articles(slug) ON DELETE CASCADE
);
