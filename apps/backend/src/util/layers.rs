use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use futures_util::future::{Either, Ready, ready};
use http::StatusCode;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::middlewares::CurrentUser;

#[derive(Clone)]
pub struct RequireUserLayer {
    allow_user: bool,
    allow_admin: bool,
}
impl RequireUserLayer {
    pub fn user_only() -> Self {
        Self {
            allow_user: true,
            allow_admin: false,
        }
    }
    pub fn admin_only() -> Self {
        Self {
            allow_user: false,
            allow_admin: true,
        }
    }
    pub fn user_or_admin() -> Self {
        Self {
            allow_user: true,
            allow_admin: true,
        }
    }
}

#[derive(Clone)]
pub struct RequireUser<S> {
    inner: S,
    allow_user: bool,
    allow_admin: bool,
}

impl<S> Layer<S> for RequireUserLayer {
    type Service = RequireUser<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RequireUser {
            inner,
            allow_user: self.allow_user,
            allow_admin: self.allow_admin,
        }
    }
}

impl<S, B> Service<Request<B>> for RequireUser<S>
where
    S: Service<Request<B>, Response = Response>,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Either<S::Future, Ready<Result<Response, S::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let ok = match req.extensions().get::<CurrentUser>() {
            Some(CurrentUser::User(_)) => self.allow_user,
            Some(CurrentUser::Admin(_)) => self.allow_admin,
            _ => false,
        };

        if ok {
            Either::Left(self.inner.call(req))
        } else {
            Either::Right(ready(Ok(
                (StatusCode::FORBIDDEN, "forbidden").into_response()
            )))
        }
    }
}
