use std::{future::Future, pin::Pin};

mod warp_server;

trait ThenTrait {
    fn then<T>(&self, fun: fn(&Self) -> T) -> T;
}

impl<V> ThenTrait for V {
    fn then<T>(&self, fun: fn(&Self) -> T) -> T {
        fun(self)
    }
}

pub trait Webserver {
    fn listen(&self) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub fn new() -> Box<dyn Webserver> {
    Box::new(warp_server::WarpWebserver::new())
}
