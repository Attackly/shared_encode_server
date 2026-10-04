-- Your SQL goes here
ALTER TABLE reencodes DROP COLUMN IF EXISTS vmaf;

-- Create the new vmaf_scores table
CREATE TABLE vmaf_scores (
    id UUID PRIMARY KEY,
    reencode_id UUID NOT NULL REFERENCES reencodes(id) ON DELETE CASCADE,
    mean FLOAT8 NOT NULL,
    harmonic_mean FLOAT8 NOT NULL,
    min FLOAT8 NOT NULL,
    percentile_1 FLOAT8 NOT NULL,
    percentile_5 FLOAT8 NOT NULL,
    max FLOAT8 NOT NULL
);