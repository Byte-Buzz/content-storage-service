-- queue of processing jobs: one row per file and kind, completed jobs are deleted

-- allow clients to request transcoding of originals
ALTER TYPE processing_task ADD VALUE 'transcode';

-- create new types
-- ingest is created by the storage event consumer, the rest by ingest from uploads.tasks
CREATE TYPE processing_job_kind AS ENUM ('ingest', 'thumbnails', 'frames', 'transcode');

-- create table for the job queue
CREATE TABLE processing_jobs
(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    -- the same id for the whole file lifetime: points to uploads for ingest and to files for the rest,
    -- so there is no foreign key
    file_id UUID NOT NULL,
    kind processing_job_kind NOT NULL,
    -- incremented on claim, so a job that crashes the worker still runs out of attempts
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    -- the job can be claimed once run_after has passed:
    -- a claim moves it forward by the lease duration, a retry by the backoff
    run_after TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- set on claim; results are saved only while it still matches,
    -- so a worker whose lease has expired cannot overwrite them
    lease_id UUID,
    last_error TEXT,
    -- set when attempts run out; failed jobs are kept for investigation and removed by cleanup
    failed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- not updated automatically: every UPDATE sets it
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- storage events are delivered at least once: duplicates are skipped with ON CONFLICT DO NOTHING
    UNIQUE (file_id, kind)
);

-- jobs that can be claimed
CREATE INDEX idx_processing_jobs_ready
    ON processing_jobs (run_after)
    WHERE failed_at IS NULL;
