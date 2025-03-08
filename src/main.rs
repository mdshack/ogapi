use axum::{Json, Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};


fn app() -> Router {
    Router::new().route("/", get(get_opengraph))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app();

    println!("Running server at: http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn make_error_response(message: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: message.to_string(),
    })
}

async fn get_opengraph(
    Query(payload): Query<OpenGraphQuery>,
) -> Result<(StatusCode, Json<OpenGraphResponse>), impl IntoResponse> {
    let response = match reqwest::get(payload.url).await {
        Ok(res) => res,
        Err(_err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                make_error_response("Invalid response"),
            ));
        }
    };

    let html = match response.text().await {
        Ok(res) => res,
        Err(_err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                make_error_response("Unable to read body"),
            ));
        }
    };

    let document = Document::new(html);

    let json = OpenGraphResponse {
        og_title: document.select_attr(
            "meta[property=\"og:title\"], meta[name=\"og:title\"]".to_string(),
            "content".to_string(),
        ),
        og_type: document.select_attr(
            "meta[property=\"og:type\"], meta[name=\"og:type\"]".to_string(),
            "content".to_string(),
        ),
        og_url: document.select_attr(
            "meta[property=\"og:url\"], meta[name=\"og:url\"]".to_string(),
            "content".to_string(),
        ),
        og_image: document.select_attr(
            "meta[property=\"og:image\"], meta[name=\"og:image\"]".to_string(),
            "content".to_string(),
        ),
        og_site_name: document.select_attr(
            "meta[property=\"og:site_name\"], meta[name=\"og:site_name\"]".to_string(),
            "content".to_string(),
        ),
        og_description: document.select_attr(
            "meta[property=\"og:description\"], meta[name=\"og:description\"]".to_string(),
            "content".to_string(),
        ),
        fb_page_id: document.select_attr(
            "meta[property=\"fb:page_id\"], meta[name=\"fb:page_id\"]".to_string(),
            "content".to_string(),
        ),
        application_name: document.select_attr(
            "meta[property=\"application-name\"], meta[name=\"application-name\"]".to_string(),
            "content".to_string(),
        ),
        og_email: document.select_attr(
            "meta[property=\"og:email\"], meta[name=\"og:email\"]".to_string(),
            "content".to_string(),
        ),
        og_phone_number: document.select_attr(
            "meta[property=\"og:phone_number\"], meta[name=\"og:phone_number\"]".to_string(),
            "content".to_string(),
        ),
        og_fax_number: document.select_attr(
            "meta[property=\"og:fax_number\"], meta[name=\"og:fax_number\"]".to_string(),
            "content".to_string(),
        ),
        og_latitude: document.select_attr(
            "meta[property=\"og:latitude\"], meta[name=\"og:latitude\"]".to_string(),
            "content".to_string(),
        ),
        og_longitude: document.select_attr(
            "meta[property=\"og:longitude\"], meta[name=\"og:longitude\"]".to_string(),
            "content".to_string(),
        ),
        og_street_address: document.select_attr(
            "meta[property=\"og:street-address\"], meta[name=\"og:street-address\"]".to_string(),
            "content".to_string(),
        ),
        og_locality: document.select_attr(
            "meta[property=\"og:locality\"], meta[name=\"og:locality\"]".to_string(),
            "content".to_string(),
        ),
        og_region: document.select_attr(
            "meta[property=\"og:region\"], meta[name=\"og:region\"]".to_string(),
            "content".to_string(),
        ),
        og_postal_code: document.select_attr(
            "meta[property=\"og:postal-code\"], meta[name=\"og:postal-code\"]".to_string(),
            "content".to_string(),
        ),
        og_country_name: document.select_attr(
            "meta[property=\"og:country-name\"], meta[name=\"og:country-name\"]".to_string(),
            "content".to_string(),
        ),
        fb_admins: document.select_attr(
            "meta[property=\"fb:admins\"], meta[name=\"fb:admins\"]".to_string(),
            "content".to_string(),
        ),
        og_points: document.select_attr(
            "meta[property=\"og:points\"], meta[name=\"og:points\"]".to_string(),
            "content".to_string(),
        ),
        og_video: document.select_attr(
            "meta[property=\"og:video\"], meta[name=\"og:video\"]".to_string(),
            "content".to_string(),
        ),
        og_video_height: document.select_attr(
            "meta[property=\"og:video:height\"], meta[name=\"og:video:height\"]".to_string(),
            "content".to_string(),
        ),
        og_video_width: document.select_attr(
            "meta[property=\"og:video:width\"], meta[name=\"og:video:width\"]".to_string(),
            "content".to_string(),
        ),
        og_video_type: document.select_attr(
            "meta[property=\"og:video:type\"], meta[name=\"og:video:type\"]".to_string(),
            "content".to_string(),
        ),
        og_audio: document.select_attr(
            "meta[property=\"og:audio\"], meta[name=\"og:audio\"]".to_string(),
            "content".to_string(),
        ),
        og_audio_title: document.select_attr(
            "meta[property=\"og:audio:title\"], meta[name=\"og:audio:title\"]".to_string(),
            "content".to_string(),
        ),
        og_audio_artist: document.select_attr(
            "meta[property=\"og:audio:artist\"], meta[name=\"og:audio:artist\"]".to_string(),
            "content".to_string(),
        ),
        og_audio_album: document.select_attr(
            "meta[property=\"og:audio:album\"], meta[name=\"og:audio:album\"]".to_string(),
            "content".to_string(),
        ),
        og_audio_type: document.select_attr(
            "meta[property=\"og:audio:type\"], meta[name=\"og:audio:type\"]".to_string(),
            "content".to_string(),
        ),
    };

    Ok((StatusCode::OK, Json(json)))
}

#[derive(Deserialize)]
struct OpenGraphQuery {
    url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct OpenGraphResponse {
    #[serde(rename = "og:title")]
    og_title: String,
    #[serde(rename = "og:type")]
    og_type: String,
    #[serde(rename = "og:url")]
    og_url: String,
    #[serde(rename = "og:image")]
    og_image: String,
    #[serde(rename = "og:site_name")]
    og_site_name: String,
    #[serde(rename = "og:description")]
    og_description: String,
    #[serde(rename = "fb:page_id")]
    fb_page_id: String,
    #[serde(rename = "application-name")]
    application_name: String,
    #[serde(rename = "og:email")]
    og_email: String,
    #[serde(rename = "og:phone_number")]
    og_phone_number: String,
    #[serde(rename = "og:fax_number")]
    og_fax_number: String,
    #[serde(rename = "og:latitude")]
    og_latitude: String,
    #[serde(rename = "og:longitude")]
    og_longitude: String,
    #[serde(rename = "og:street-address")]
    og_street_address: String,
    #[serde(rename = "og:locality")]
    og_locality: String,
    #[serde(rename = "og:region")]
    og_region: String,
    #[serde(rename = "og:postal-code")]
    og_postal_code: String,
    #[serde(rename = "og:country-name")]
    og_country_name: String,
    #[serde(rename = "fb:admins")]
    fb_admins: String,
    #[serde(rename = "og:points")]
    og_points: String,
    #[serde(rename = "og:video")]
    og_video: String,
    #[serde(rename = "og:video:height")]
    og_video_height: String,
    #[serde(rename = "og:video:width")]
    og_video_width: String,
    #[serde(rename = "og:video:type")]
    og_video_type: String,
    #[serde(rename = "og:audio")]
    og_audio: String,
    #[serde(rename = "og:audio:title")]
    og_audio_title: String,
    #[serde(rename = "og:audio:artist")]
    og_audio_artist: String,
    #[serde(rename = "og:audio:album")]
    og_audio_album: String,
    #[serde(rename = "og:audio:type")]
    og_audio_type: String,
}

trait HasHtmlElements {
    fn new(html: String) -> Self;
    fn select_attr(&self, selector: String, attribute: String) -> String;
}

struct Document {
    pub document: scraper::html::Html,
}

impl HasHtmlElements for Document {
    fn new(html: String) -> Self {
        Self {
            document: Html::parse_document(&html),
        }
    }

    fn select_attr(&self, selector: String, attribute: String) -> String {
        let target = Selector::parse(&selector).unwrap();
        let element = match self.document.select(&target).next() {
            Some(res) => res,
            None => return "".to_string(),
        };
        return match element.value().attr(&attribute) {
            Some(res) => res.to_string(),
            None => "".to_string(),
        };
    }
}

// TODO: testing
// mod tests {
//     use super::*;
//     use axum::{
//         body::Body,
//         extract::connect_info::MockConnectInfo,
//         http::{self, Request, StatusCode},
//     };
//     use serde_json::{json, Value};
//     use tokio::net::TcpListener;
//     use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

//     #[tokio::test]
//     async fn test_get_opengraph() {
//         // Request a new server from the pool
//         let mut server = mockito::Server::new();
    
//         // Use one of these addresses to configure your client
//         let host = server.host_with_port();
//         let url = server.url();
    
//         // Create a mock
//         let mock = server.mock("GET", "/hello")
//             .with_status(201)
//             .with_header("content-type", "text/plain")
//             .with_header("x-api-key", "1234")
//             .with_body("world")
//             .create();
    
//         // let mut res = reqwest::blocking::get("/?url=/hello")?;
    
//         // print!("{}", res.status());
    
//         // // You can use `Mock::assert` to verify that your mock was called
    
//         let app = app();
    
    
//         let response = app
//             .oneshot(Request::builder().uri(format!("/?url={}/hello", url)).body(Body::empty()).unwrap())
//             .await
//             .unwrap();
    
//         print!("host: {}, url: {}", host, url);
//         // let app
    
//         // get_opengraph(Query(OpenGraphQuery{url: "/hello".to_string()}));
    
//         // let resp = reqwest::blocking::get(format!("{}/?url=/hello", url))?.text()?;
//         // println!("{:#?}", resp);
    
//         mock.assert();
//     }
// }