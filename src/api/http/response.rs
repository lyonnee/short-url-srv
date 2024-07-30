use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            code: 200,
            msg: None,
            data: Some(data),
        }
    }

    pub fn fail(code: i32, msg: String) -> Self {
        Response {
            code: code,
            msg: Some(msg),
            data: None,
        }
    }
}
