extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::process;
use hyper::Client;
use hyper::header::{Headers, Authorization, Bearer};
use std::io::Read;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Codic {
    successful: bool,
    text: String,
    translated_text: String,
    words: Vec<Word>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Word {
    successful: bool,
    text: String,
    translated_text: Option<String>,
    candidates: Vec<Candidate>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Candidate {
    text: String,
}

const CODIC_KEY: &'static str = "CODIC_KEY";

fn main() {
    if let Ok(codic_key) = env::var(CODIC_KEY) {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            println!("put only one word");
            process::exit(1);
        }
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: codic_key.to_owned() }));
        let mut url = String::new();
        let host = "https://api.codic.jp/v1/engine/translate.json?text=".to_owned();
        let word = args[1].to_owned();
        let casing = "&casing=lower+underscore".to_owned();
        url.push_str(&host);
        url.push_str(&word);
        url.push_str(&casing);

        let mut res = Client::new()
                          .get(url.as_str())
                          .headers(headers)
                          .send()
                          .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        println!("json: {}", body);
        let decoded_codics: Vec<Codic> = json::decode(&body).unwrap();
        for decoded_codic in decoded_codics {
            for word in decoded_codic.words {
                for candidate in word.candidates {
                    println!("Response: {}", candidate.text);
                }
            }
        }
    } else {
        println!("make sure environment variable is setted properly")
    }
}
