// Attribution notice- This function comes from a generated bit found in OpenAPI
// Generator's default Rust implementation.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
