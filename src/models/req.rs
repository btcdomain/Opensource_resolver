use rocket::{request::{FromRequest, Outcome}, FromForm, Request};


#[derive(FromForm, Debug)]
pub struct RequestParams { pub host: String }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestParams {
    type Error = rocket::Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("host: {:?}", request.host());
        let url = request.host().unwrap().domain().as_str();
        println!("url: {:?}", url);
        Outcome::Success(RequestParams { host: String::from(url) })
    }
}
