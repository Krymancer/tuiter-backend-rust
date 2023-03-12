use axum::{http::StatusCode, response::IntoResponse, Json, http::headers::{self, HeaderValue, HeaderName}};
use serde::Serialize;
use serde_json::json;


#[derive (Debug)]
pub struct CustomResponse<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
}

pub struct CustomResponseBuilder<T: Serialize> {
    body: Option<T>,
    status_code: StatusCode,
}

impl<T> Default for CustomResponseBuilder<T> where T : Serialize {
    fn default() -> Self {
        Self::default()
    }

    fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    fn build(self) -> CustomResponse<T> {
        CustomResponse {
            body: self.body,
            status_code: self.status_code,
        }
    }
}

impl<T> IntoResponse for CustomResponse<T> where T : Serialize {

    fn into_response(self) -> Response {
        let headers = [(
            header::CONTENT_TYPE, HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )];
        (self.status_code, headers, Json(json!(self.body))).into_response()
    }
}


