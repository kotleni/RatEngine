use lazy_static::lazy_static;

pub struct Engine {

}

impl Engine {
    // TODO: Move all engine logic here
    // TODO: Impl load function
    // TODO: Impl start_loop function
    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

lazy_static! {
    static ref GLOBAL_ENGINE: Engine = Engine { };
}

pub fn engine() -> &'static Engine {
    &GLOBAL_ENGINE
}