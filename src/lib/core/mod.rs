pub mod config;
/// Module for handling file system.
pub mod fs;
/// Module for handling markdown.
pub mod markdown;
pub mod html;

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
