use std::fmt::Display;

use crate::common::status::Status;

pub trait App<Config: Display>: Sized {
    type Error;

    /// Runs the application.
    /// It should store a thread handler like `JoinHandle` or similar
    /// It should not block the current thread.
    fn run(&self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Configures the application with the provided configuration.
    fn configure(config: Config) -> impl Future<Output = Result<Self, Self::Error>>;

    /// Stops the application by aborting the running thread or process.
    fn stop(&self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Those methods are used to get the state of the application.
    /// This should return a displayable configuration. (Not the secret !!!)
    fn configuration(&self) -> impl Future<Output = Result<impl Display, Self::Error>>;

    /// Returns the current status of the application.
    fn status(&self) -> impl Future<Output = Status>;

    fn name(&self) -> String;
}

