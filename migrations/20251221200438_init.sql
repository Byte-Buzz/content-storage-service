-- app
CREATE TABLE app
(
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- uploads
CREATE TABLE uploads
(
    id UUID PRIMARY KEY,
    app_id BIGINT NOT NULL REFERENCES app(id),
    settings INTEGER NOT NULL,
    info JSONB,
    secret TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_uploads_expires_at ON uploads(expires_at);

-- temp_files
CREATE TABLE temp_files
(
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    app_id BIGINT NOT NULL REFERENCES app(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_temp_files_expires_at ON temp_files(expires_at);

-- files
CREATE TYPE file_access AS ENUM ('unsigned', 'signed');

CREATE TABLE files
(
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    app_id BIGINT NOT NULL REFERENCES app(id),
    extension TEXT NOT NULL,
    info JSONB,
    access file_access NOT NULL DEFAULT 'signed',
    miniatures INTEGER[],
    hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
