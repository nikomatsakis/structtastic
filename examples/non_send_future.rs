#![feature(async_closure)]

use std::{cell::RefCell, rc::Rc};

// We are using the current_thread runtime because this version
// of the macro returns a non-Send Future.
#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    // This value is not thread-safe
    let value = Rc::new(RefCell::new(22));

    moro::scope_local(async |scope| {
        scope.spawn(async {
            scope.spawn(async {
                *value.borrow_mut() *= 2; // mutate shared state
            });

            *value.borrow_mut() *= 2;
        });

        *value.borrow_mut() *= 2;
    })
    .await;
    println!("{value:?}");
}
