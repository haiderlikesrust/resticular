use crate::handle_thread_error_with_error;
use thiserror;
#[test]
fn check_handle_error() {
    let a = handle_thread_error_with_error!(test_func(), Error::FooError);
    
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Foo Error")]
    FooError
}

fn test_func() -> Result<&'static str, Error> {
    let a = 10;
    match a {
        10 => Ok("Good"),
        _ => Err(Error::FooError)
    }
}