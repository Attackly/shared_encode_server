pub struct ReencodeQuery {
    pub hash: i64,
    pub min_vmaf: f32,
    pub encoder: String,
    pub encoder_version: String,
}