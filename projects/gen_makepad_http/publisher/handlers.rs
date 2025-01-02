use makepad_widgets::{HttpError, HttpRequest, HttpResponse};


pub struct Handlers{
    pub error: Option<fn(&HttpError)>,
    pub before_request: Option<fn(&mut HttpRequest)>,
    pub after_response: Option<fn(&HttpResponse)>,
}