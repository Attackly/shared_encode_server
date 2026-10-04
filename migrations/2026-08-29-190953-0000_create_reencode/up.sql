CREATE TABLE reencodes (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    hash BIGINT NOT NULL,
    ffmpeg_version VARCHAR NOT NULL,
    ffmpeg_command TEXT NOT NULL,
    svtav1_version VARCHAR NOT NULL,
    timestamp_start TIMESTAMPTZ NOT NULL,
    timestamp_end TIMESTAMPTZ,
    vmaf JSONB
);