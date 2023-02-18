use actix_web::{
    web,
    Error,
    HttpRequest,
    HttpResponse, 
};
use futures::{future::ok, stream::once};

pub async fn root(_req: HttpRequest) -> &'static str {
    "Welcome to Morale Meter!"
}

pub async fn api_root(_req: HttpRequest) -> HttpResponse {
    let data = b"{ \"status\": \"ok!\" }";
    let body = once(ok::<_, Error>(web::Bytes::from_static(data)));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, test, web, App, Error};

    use super::*;
    
    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = test::init_service(App::new().route("/", web::get().to(root))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;
        assert!(resp.status().is_success());
        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, "Welcome to Morale Meter!");
        Ok(())
    }

    #[actix_web::test]
    async fn test_api_root() -> Result<(), Error> {
        let app = test::init_service(App::new().route("/api/v1", web::get().to(api_root))).await;
        let req = test::TestRequest::get().uri("/api/v1").to_request();
        let resp = app.call(req).await.unwrap();
        assert!(resp.status().is_success());
        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await.unwrap(), "{ \"status\": \"ok!\" }");
        Ok(())
    }


}
