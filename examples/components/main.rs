#![recursion_limit="128"]
mod app;

use rustux::Extensions::PWABuilder;
use crate::app::Model;


fn main () {
    new_pwa();
    // new_webapp();
}

fn new_pwa(){
    // Using a builder pattern for PWA boilerplate
    let mut my_pwa = PWABuilder::default()
                    .name("My Progressive WebApp".into())
                    .short_name("My PWA".into())
                    .prerendered(true)
                    .build();

    my_pwa.start_yew_app::<Model>() // It's now progressive! - Reload the page and watch how the state of the element is retained
}



fn new_webapp() {
    // Builder pattern for 
    let mut my_webapp = PWABuilder::default()
                    .name("My Progressive WebApp".into())
                    .short_name("My PWA".into())
                    .prerendered(true)
                    .build();

    my_webapp.start_yew_app::<Model>() // It's now progressive! - Reload the page and
}