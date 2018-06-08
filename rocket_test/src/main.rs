extern crate futures;
extern crate hyper;

use futures::future::Future;

use hyper::server::{Http, Request, Response, Service};
use hyper::Method;

struct HelloWorld;
fn main() {
  println!("running on port 3000...");

  let addr = "127.0.0.1:3000".parse().unwrap();
  let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
  server.run().unwrap();
}

const GREETING: &'static str = "Hello, ";

impl Service for HelloWorld {
  // boilerplate hooking up hyper's server types
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  // The future representing the eventual Response your call will
  // resolve to. This can change to whatever Future you need.
  type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let mut response = Response::new();
    let split_path = req.path().split('/').skip(1).collect::<Vec<_>>();

    response.set_body(match (req.method(), split_path.as_slice()) {
      (&Method::Get, []) => "Path Not Found".to_owned(),
      (&Method::Get, ["hello", name]) => get_hello(name),
      (&Method::Get, ["hello"]) => get_hello(""),
      any => format!("{:?}", any.to_owned()),
    });
    // We're currently ignoring the Request
    // And returning an 'ok' Future, which means it's ready
    // immediately, and build a Response with the 'PHRASE' body.
    Box::new(futures::future::ok(response))
  }
}

fn get_hello(name: &str) -> String {
  let resp = match name {
    "" => "nobody",
    any => any,
  };
  format!("{}{}", GREETING, resp)
}
