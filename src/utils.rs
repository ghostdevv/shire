pub fn get_ua_header() -> String {
    format!(
        "Shire DDNS/{} (+https://github.com/ghostdevv/shire)",
        env!("CARGO_PKG_VERSION")
    )
}
