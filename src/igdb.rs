use crate::auth::Auth;

pub(crate) struct ApiService {
    auth: Auth,
}
impl ApiService {
    pub fn new() -> Self {
        Self {
            auth: Auth::new("IGDB"),
        }
    }
}