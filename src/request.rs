use async_std::{prelude::*, io};

pub async fn get_request(request: &String) -> io::Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::get(request).await?;
    response
}