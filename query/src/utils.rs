use polars::frame::DataFrame;

pub fn collect_json(df: &DataFrame) -> serde_json::Result<String> {
    serde_json::to_string(df)
}
