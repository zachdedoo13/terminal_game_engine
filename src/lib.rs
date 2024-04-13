
mod modules {
    pub(crate) mod window;
    pub(crate) mod input;
    pub(crate) mod colors;
    pub(crate) mod other;
    pub(crate) mod vectors;
}

pub use modules::vectors::Vec2;
pub use modules::input::Input;
pub use modules::window::Window;
pub use modules::colors::Color;

pub use modules::other::wait;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("works")
    }
}
