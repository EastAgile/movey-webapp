use jelly::actix_web::http::header::{HeaderValue, IntoHeaderValue, InvalidHeaderValue};

pub enum Value {
    RememberMeTokenInvalidate,
}

impl IntoHeaderValue for Value {
    type Error = InvalidHeaderValue;

    #[inline]
    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        match self {
            Value::RememberMeTokenInvalidate => Ok(HeaderValue::from_str(
                "remember_me_token=\"\"; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
            )?),
        }
    }
}
