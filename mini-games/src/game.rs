use std::pin::Pin;
use std::future::Future;

pub trait Game {
    fn start(&self) -> Pin<Box<dyn Future<Output = ()> + 'static>>;
}