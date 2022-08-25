CREATE TABLE users (
    id               BIGSERIAL NOT NULL,
    name             TEXT NOT NULL,
    create_timestamp TIMESTAMP NOT NULL,
    update_timestamp TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE articles (
    id               BIGSERIAL NOT NULL,
    author_id        BIGINT NOT NULL,
    title            TEXT NOT NULL,
    content          TEXT NOT NULL,
    like_count       INTEGER NOT NULL DEFAULT 0,
    create_timestamp TIMESTAMP NOT NULL,
    update_timestamp TIMESTAMP NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE likes (
    id               BIGSERIAL NOT NULL,
    article_id       BIGINT NOT NULL,
    user_id          BIGINT NOT NULL,
    create_timestamp TIMESTAMP DEFAULT '1000-01-01 00:00:00',
    UNIQUE (user_id, article_id),
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (article_id) REFERENCES articles (id) ON DELETE CASCADE
);
