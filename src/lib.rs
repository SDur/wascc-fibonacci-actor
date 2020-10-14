extern crate wascc_actor as actor;

#[macro_use]
extern crate log;
extern crate serde;
extern crate wascc_codec;
extern crate num_bigint as bigint;
extern crate num_traits;
#[macro_use] extern crate tramp;

use actor::prelude::*;
use serde::Serialize;
use bigint::BigUint;
use num_traits::{Zero, One};
use tramp::{tramp, Rec};

actor_handlers! {
    codec::http::OP_HANDLE_REQUEST => fibonacci,
    codec::core::OP_HEALTH_REQUEST => health
}

fn fibonacci(r: codec::http::Request) -> HandlerResult<codec::http::Response> {
    info!("Calculating {} th focaccia number", r.query_string);
    let upper = FibonacciResponse {
        input: r.query_string.to_string(),
        result: fib(r.query_string.parse::<i32>().unwrap()).to_string(),
    };
    info!("Result {} focaccia number: {}", r.query_string, upper.result);
    Ok(codec::http::Response::json(upper, 200, "OK"))
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}

fn fib(n: i32) -> BigUint {
  tramp(do_fib(n, Zero::zero(), One::one()))
}

fn do_fib(n: i32, acc: BigUint, curr: BigUint) -> Rec<BigUint> {
  if n <= 0 {
    rec_ret!(acc)
  }else{
    let new = &acc + curr;
    let nn = n - 1;
    rec_call!(do_fib(nn, new, acc))
  }
}

#[derive(Serialize)]
struct FibonacciResponse {
    input: String,
    result: String,
}
