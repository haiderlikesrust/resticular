use std::thread::JoinHandle;

pub mod config;
/// Module for handling file system.
pub mod fs;
/// Module for handling markdown.
pub mod markdown;
pub mod html;
pub mod http;

/// This trait basically implements the functionality of returning whats inside the tuple struct.
/// ```
/// use resticular::core::IntoInner;
/// struct Foo(String);
/// impl IntoInner for Foo {
///     type Output = String;
///     fn into_inner(self) -> Self::Output {
///         self.0
///     }
/// }
/// ```
pub trait IntoInner {
    type Output;

    /// This method takes the ownership of the struct, which is erased from memory after this is ran and returns the `Output`
    ///  what you have given it.
    fn into_inner(&self) -> Self::Output;
}


pub struct JoinHandler<T, U, V> {
    pub t1: JoinHandle<T>,
    pub t2: JoinHandle<U>,
    pub t3: JoinHandle<V>
}

impl<T, U, V> JoinHandler<T, U, V> {
    pub fn join(self) {
        self.t1.join();
        self.t2.join();
        self.t3.join();
    }
}