/// Used to spy on requests and responses.
pub trait Observer {
    fn on_request(&self, method: &str, url: &str, body: &str);
    fn on_response(&self, status: &str, body: &str);
}
