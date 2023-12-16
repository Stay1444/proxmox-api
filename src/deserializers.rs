use serde::Deserializer;

// Proxmox likes to encode empty fields as empty strings
pub fn non_empty_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()))
}
