use gloo_net::http::{Request, Response};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Data {
  url: String,
  request_headers: HashMap<String, String>,
  body: String,
}

#[wasm_bindgen]
pub async fn handler(
  req: web_sys::Request,
) -> Result<web_sys::Response, JsError> {
  let req: Request = req.into();
  let request_headers: HashMap<_, _> = req.headers().entries().collect();
  let text = req.text().await.unwrap_or("".to_string());
  let d = Data {
    url: req.url(),
    request_headers,
    body: format!(
      "The request body contained: {}",
      if text.is_empty() {
        "nothing".to_string()
      } else {
        text
      }
    ),
  };

  Response::new()
    .json(&d)
    .map(|x| x.into())
    .map_err(|x| JsError::new("An error occurred"))
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn it_works() {
  //   let result = add(1, 2);
  //   assert_eq!(result, 3);
  // }
}
