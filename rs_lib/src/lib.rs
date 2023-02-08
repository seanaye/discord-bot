use wasm_bindgen::prelude::*;
use web_sys::{Request, Response, ResponseInit, Headers};

#[wasm_bindgen]
pub async fn handler(req: Request) -> Result<Response, JsValue> {
    let a = req.url();
    let headers = Headers::new()?;
    headers.append("Content-Type", "application/json")?;
    let mut init = ResponseInit::new();
    init.status(200).headers(&headers);
    let s = format!(r##"{{ "message": "helloworld", "from": "{}"  }}"##, a);
    Response::new_with_opt_str_and_init(Some(s.as_str()), &init)
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
