pub use toybox::prelude::*;
pub use toybox::input::raw::{Scancode, MouseButton};

pub use toybox::gfx::mesh::traits::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
