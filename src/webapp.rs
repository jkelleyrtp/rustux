use yew::prelude::*;

// The base requirements that a webapp is built on
// pub trait WebApplication {


// }

/// Serves as the root component and statemanager for the Yew Application
pub struct WebApplication {
    


}









impl WebApplication {
    pub fn start_yew_app<T: Component> (&mut self) where T: Renderable<T> {
        yew::initialize();








        App::<T>::new();


    }
}