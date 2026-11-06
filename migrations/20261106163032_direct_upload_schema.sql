-- switch to direct uploads: storage receives files, the service only tracks expectations

-- remove deprecated tables
DROP TABLE temp_files, uploads, files;

-- remove deprecated type
DROP TYPE file_access;


-- create new types
CREATE TYPE file_visibility AS ENUM ('private', 'public');
CREATE TYPE processing_task AS ENUM ('thumbnails', 'frames');

-- recreate table with pending uploads and what is expected from them
CREATE TABLE uploads
(
    id UUID PRIMARY KEY,
    app_id BIGINT NOT NULL REFERENCES app(id),
    content_type TEXT NOT NULL,
    accepted_content_types TEXT[] NOT NULL DEFAULT '{}',
    max_size BIGINT NOT NULL CHECK (max_size > 0),
    max_duration_seconds INTEGER CHECK (max_duration_seconds > 0),
    tasks processing_task[] NOT NULL DEFAULT '{}',
    external_tasks TEXT[] NOT NULL DEFAULT '{}',
    info BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX idx_uploads_expires_at ON uploads (expires_at);

-- recreate table for processed files
CREATE TABLE files
(
    id UUID PRIMARY KEY,
    app_id BIGINT NOT NULL REFERENCES app(id),
    content_type TEXT NOT NULL,
    variants TEXT[] NOT NULL DEFAULT '{}',
    visibility file_visibility NOT NULL DEFAULT 'private',
    visibility_version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
