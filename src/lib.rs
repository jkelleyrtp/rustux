#[macro_use]
extern crate derive_builder;
mod extensions;
mod webapp;

pub mod Extensions {
    use super::extensions;

    pub use extensions::pwa::PWABuilder;
    // pub use extensions::pwabuilder::PWABuilder
}


// mod Extensions;


/// Globally accessible tree that manages the state of stores
pub struct StateTree {


}


pub struct Store {
    
}
