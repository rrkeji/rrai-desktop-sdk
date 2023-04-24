use actix_web::web::Data;
use actix_web::{get, web, HttpRequest, HttpResponse, HttpServer, Responder};
use awc::http::StatusCode;
use futures_util::stream::TryStreamExt;

#[get("/{app_cid}/{tail:.*}")]
pub async fn server_proxy(
    req: HttpRequest,
    args: web::Path<(String, String)>,
    body: web::Payload,
    http_client: Data<awc::Client>,
) -> impl Responder {
    tracing::debug!(" args: [{}] [{}]", args.0, args.1);
    // Stream request from the client to the proxied server
    let url = format!(
        "{to}{app_cid}/{path}",
        to = "http://localhost:35080/apps/",
        app_cid = args.0,
        path = args.1
    );

    tracing::debug!("=> {url}");
    match http_client
        .request_from(&url, req.head())
        .send_stream(body)
        .await
    {
        Ok(resp) => {
            // Stream response back to the client
            let status = resp.status();
            tracing::debug!("<= [{status}] {url}", status = status.as_u16());
            let mut resp_builder = HttpResponse::build(status);
            for header in resp.headers() {
                resp_builder.insert_header(header);
            }
            resp_builder.streaming(resp.into_stream())
        }
        Err(err) => {
            tracing::warn!("{url}: {err:?}");
            HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
        }
    }
}
