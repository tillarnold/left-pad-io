extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate quick_error;


use std::io;
use std::io::Read;

use hyper::Client;
use hyper::Url;

use rustc_serialize::json;
use rustc_serialize::json::Json;


static STR_NOT_RETURNED: &'static str = "The response does not contain 'str'";
static STR_NOT_A_STRING: &'static str = "The returned 'str' parameter was not a string";
static NOT_AN_OBJECT: &'static str = "The response from left-pad.io was not an object";

quick_error! {
  #[derive(Debug)]
  pub enum PadError {
    Io(err: io::Error) {
      from()
        description("io error")
        display("I/O error: {}", err)
        cause(err)
    }
    Hyper(err: hyper::Error) {
      from()
        display("hyper error: {}", err)
        cause(err)
    }
    Json(err: json::ParserError) {
      from()
        display("json error: {}", err)
        cause(err)
    }
    ApiError{error_message: String, error_type :  String} {
      display("ApiError errorMessage:\"{}\" errorType:\"{}\"", error_message, error_type)
    }
    UnknownResponse(response: String) {
      display("UnknownResponse {:?}", response)
    }
  }
}


fn api_error_response(obj: &mut json::Object) -> PadError {
    let error_message = obj.remove("errorMessage").unwrap(); // we know this exists
    let error_type_res = obj.remove("errorType");

    if let Some(error_type) = error_type_res {
        if let Json::String(type_string) = error_type {
            if let Json::String(message_string) = error_message {
                return PadError::ApiError {
                    error_message: message_string,
                    error_type: type_string,
                };

            }
        }
    }
    return PadError::UnknownResponse(format!("{:?}", obj));


}


pub fn left_pad(string: &str, ch: &str, len: u32) -> Result<String, PadError> {
    let mut url = Url::parse("https://api.left-pad.io").unwrap();
    url.query_pairs_mut()
        .clear()
        .extend_pairs(&[("str", string), ("len", &len.to_string()), ("ch", ch)]);

    let client = Client::new();
    let mut res = try!(client.get(url).send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));


    let json = try!(Json::from_str(&body));

    if let Json::Object(mut obj) = json {
        if obj.contains_key("errorMessage") {
            return Err(api_error_response(&mut obj));
        }
        if let Some(result) = obj.remove("str") {
            if let Json::String(str_res) = result {
                return Ok(str_res);
            }
            return Err(PadError::UnknownResponse(STR_NOT_A_STRING.to_string()));
        }

        return Err(PadError::UnknownResponse(STR_NOT_RETURNED.to_string()));

    }

    return Err(PadError::UnknownResponse(NOT_AN_OBJECT.to_string()));
}

#[cfg(test)]
mod test {
    use super::left_pad;
    use super::PadError;
    #[test]
    fn it_pads_a_string_left() {
        assert_eq!("     hello", left_pad("hello", " ", 10).unwrap());
    }

    #[test]
    fn it_does_not_cut_string_of() {
        assert_eq!("hello", left_pad("hello", " ", 3).unwrap());
    }

    #[test]
    fn it_does_not_work_with_numbers_above_1024() {

        let result = left_pad("hello", " ", 1025).unwrap_err();
        if let PadError::ApiError { .. } = result {

        } else {
            panic!("should have been an ApiError was: {}", result);
        }
    }

}
