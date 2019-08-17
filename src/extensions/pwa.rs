use crate::webapp;
use yew::prelude::*;






// #[derive(Builder)]
pub struct PWABuilder {
    pub short_name: String,
    pub name: String,
    pub start_url: String,
    pub background_color: String,
    pub display: String,
    pub scope: String,
    pub theme_color: String,
    pub favicon: String,
    pub prerendered: bool
}

impl PWABuilder {
    pub fn default() -> Self {
        Self {
            short_name: "DEFAULT PWA NAME".to_string(),
            name: "DEFAULT PWA NAME".to_string(),
            start_url: "localhost:8000/".to_string(),
            background_color: "#FFFFFF".to_string(),
            display: "standalone".to_string(),
            scope: "/".to_string(),
            theme_color: "#FFFFFF".to_string(),
            favicon: "../static/assets/favicon.png".to_string(),
            prerendered: false
        }
    }

    // Support the builder pattern
    pub fn short_name(mut self, short_name: String ) -> Self { self.short_name = short_name; self }
    pub fn name(mut self, name: String ) -> Self { self.name = name; self }
    pub fn start_url(mut self, start_url: String ) -> Self { self.start_url = start_url; self }
    pub fn background_color(mut self, background_color: String ) -> Self { self.background_color = background_color; self }
    pub fn display(mut self, display: String ) -> Self { self.display = display; self }
    pub fn scope(mut self, scope: String ) -> Self { self.scope = scope; self }
    pub fn theme_color(mut self, theme_color: String ) -> Self { self.theme_color = theme_color; self }
    pub fn favicon(mut self, favicon_path: String) -> Self {self.favicon = favicon_path; self}
    pub fn prerendered(mut self, p: bool) -> Self {self.prerendered = p; self}


    // Build that shiz
    pub fn build(mut self) -> PWA {
        PWA {
            
        }
    }
}

pub struct PWA {

}

impl PWA {
    pub fn start_yew_app<T: Component>(&mut self) {
        use yew;
        // yew::start_app::<Model>();

    }
}


// fn main() {
//     yew::start_app::<Model>();
// }

// impl webapp::WebApplication for PWA {

// }



// {
//   "short_name": "Maps",
//   "name": "Google Maps",
//   "icons": [
//     {
//       "src": "/images/icons-192.png",
//       "type": "image/png",
//       "sizes": "192x192"
//     },
//     {
//       "src": "/images/icons-512.png",
//       "type": "image/png",
//       "sizes": "512x512"
//     }
//   ],
//   "start_url": "/maps/?source=pwa",
//   "background_color": "#3367D6",
//   "display": "standalone",
//   "scope": "/maps/",
//   "theme_color": "#3367D6"
// }