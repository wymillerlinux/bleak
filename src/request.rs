pub fn get_request(request: &String) -> Result<reqwest::Response, reqwest::Error> {
    let response = reqwest::get(request);
    response
}