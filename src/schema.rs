// @generated automatically by Diesel CLI.
diesel::table! {
    reencodes (id) {
        id -> Uuid,
        name -> Varchar,
        hash -> Int8,
        ffmpeg_version -> Varchar,
        ffmpeg_command -> Text,
        svtav1_version -> Varchar,
        timestamp_start -> Timestamptz,
        timestamp_end -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    vmaf_scores (id) {
        id -> Uuid,
        reencode_id -> Uuid,
        mean -> Float8,
        harmonic_mean -> Float8,
        min -> Float8,
        percentile_1 -> Float8,
        percentile_5 -> Float8,
        max -> Float8,
    }
}

diesel::joinable!(vmaf_scores -> reencodes (reencode_id));

diesel::allow_tables_to_appear_in_same_query!(reencodes, vmaf_scores,);
