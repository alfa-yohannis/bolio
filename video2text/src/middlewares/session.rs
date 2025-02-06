use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture};
use std::rc::Rc;

pub struct SessionMiddleware;

impl<S, B> Transform<S, ServiceRequest> for SessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddlewareService<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(ok(SessionMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct SessionMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SessionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let session_id = req
        //     .cookie("session_id")
        //     .map(|cookie| cookie.value().to_string());

        // // Store the session_id in request extensions for later access
        // req.extensions_mut().insert(session_id);

        let path = req.path().to_string();

        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)

            // let res = fut.await;
            // match &res {
            //     Ok(response) => {
            //         log::info!("Middleware passed response for path: {}", path);
            //         log::info!("Response status: {:?}", response.response().status());
            //     }
            //     Err(e) => log::error!("Error in middleware for path {}: {:?}", path, e),
            // }
            // res
        })
    }
}
