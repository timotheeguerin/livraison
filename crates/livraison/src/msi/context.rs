use uuid::Uuid;

pub struct Context {
    /// UUID that uniquely identifies this specific package at this specific version.
    pub product_code: Uuid,
    /// The UUID that uniquely identifies this installer package even if the name changes.
    pub upgrade_code: Uuid,
}
