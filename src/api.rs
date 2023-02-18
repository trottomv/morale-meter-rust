use actix_web::{
    web,
    Error,
    HttpRequest,
    HttpResponse,
};
use futures::{future::ok, stream::once};


pub async fn surveys_list(_req: HttpRequest) -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"[{\"id\": 1, \"name\": \"Ownership & Empowerment\", \"slug\": \"ownership_empowerment\"},{\"id\": 2, \"name\": \"Teamwork & Alignment\", \"slug\": \"teamwork_alignment\"},{\"id\": 3, \"name\": \"Safety to take a risk\", \"slug\": \"safety_take_risk\"},{\"id\": 4, \"name\": \"Pride\", \"slug\": \"pride\"},{\"id\": 5, \"name\": \"Fun & Joy\", \"slug\": \"fun_joy\"}]")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, test, web, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_surveys_list() -> Result<(), Error> {
        let app = test::init_service(App::new().route("/api/v1/surveys", web::get().to(surveys_list))).await;
        let req = test::TestRequest::get().uri("/api/v1/surveys").to_request();
        let resp = app.call(req).await.unwrap();
        assert!(resp.status().is_success());
        let response_body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(to_bytes(response_body_bytes).await.unwrap(), "[{\"id\": 1, \"name\": \"Ownership & Empowerment\", \"slug\": \"ownership_empowerment\"},{\"id\": 2, \"name\": \"Teamwork & Alignment\", \"slug\": \"teamwork_alignment\"},{\"id\": 3, \"name\": \"Safety to take a risk\", \"slug\": \"safety_take_risk\"},{\"id\": 4, \"name\": \"Pride\", \"slug\": \"pride\"},{\"id\": 5, \"name\": \"Fun & Joy\", \"slug\": \"fun_joy\"}]");
        Ok(())
    }
}
