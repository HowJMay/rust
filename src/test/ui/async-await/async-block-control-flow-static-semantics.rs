// Test that `async { .. }` blocks:
// 1. do not allow `break` expressions.
// 2. get targeted by `return` and not the parent function.
// 3. get targeted by `?` and not the parent function.
//
// edition:2018
// ignore-tidy-linelength

#![feature(async_await)]

fn main() {}

use core::future::Future;

fn return_targets_async_block_not_fn() -> u8 {
    //~^ ERROR mismatched types
    let block = async {
        return 0u8;
    };
    let _: &dyn Future<Output = ()> = &block;
    //~^ ERROR type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
}

async fn return_targets_async_block_not_async_fn() -> u8 {
    //~^ ERROR type mismatch resolving
    let block = async {
        return 0u8;
    };
    let _: &dyn Future<Output = ()> = &block;
    //~^ ERROR type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
}

fn no_break_in_async_block() {
    async {
        break 0u8; //~ ERROR `break` inside of an async block
    };
}

fn no_break_in_async_block_even_with_outer_loop() {
    loop {
        async {
            break 0u8; //~ ERROR `break` inside of an async block
        };
    }
}

struct MyErr;
fn err() -> Result<u8, MyErr> { Err(MyErr) }

fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
    //~^ ERROR mismatched types
    let block = async {
        err()?;
        Ok(())
    };
    let _: &dyn Future<Output = Result<(), MyErr>> = &block;
}

fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
    //~^ ERROR mismatched types
    let block = async {
        err()?;
        Ok(())
    };
    let _: &dyn Future<Output = Result<(), MyErr>> = &block;
}
