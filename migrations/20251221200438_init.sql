-- app
CREATE TABLE app
(
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- uploads
CREATE TYPE file_access AS ENUM ('private', 'public');

CREATE TABLE uploads
(
    id UUID PRIMARY KEY,
    app_id BIGINT NOT NULL REFERENCES app(id),
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    max_size INTEGER NOT NULL,
    settings INTEGER NOT NULL,
    info JSONB,
    access file_access NOT NULL DEFAULT 'private',
    secret TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_uploads_expires_at ON uploads(expires_at);

-- temp_files
CREATE TABLE temp_files
(
    id UUID PRIMARY KEY,
    filename TEXT NOT NULL,
    app_id BIGINT NOT NULL REFERENCES app(id),
    content_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_temp_files_expires_at ON temp_files(expires_at);

-- files
CREATE TABLE files
(
    id UUID PRIMARY KEY,
    filename TEXT NOT NULL,
    app_id BIGINT NOT NULL REFERENCES app(id),
    content_type TEXT NOT NULL,
    e_tag TEXT NOT NULL,
    access file_access NOT NULL DEFAULT 'private',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
