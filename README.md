# Rustux

Rustux - think Rust-Redux - is a global state manager for user-interactive applications that makes:

- State serialization
- Backup and loading
- Real-time collaboration

work out of the box for your application.

Rustux is currently intended for use on the web and provides many convenience methods for interacting with stdweb and Yew. Rustux currently only supports Yew.


Some funcionality is not tied directly to state management but still integrates well with the web. Some extensions include:
- [ ] PWA Builder 
- [ ] Undo/Redo system
- [ ] Frontend routing (powered by Yew-Router)
- [ ] Static site generator
- [ ] Serverside rendering 
- [ ] Direct access with cookies using native rust structs


## Usaage:
Let's make a new progressive web app:


``` rust
use Rustux::Extensions::PWABuilder;
use Rustux::Exentsions::TodoApp;

fn main () {
    let my_app = PWABuilder::new()
                    .with_favicon()
                    .with_name()
                    .build();

    my_app.start_yew_app::<TodoApp>() // It's now progressive! - Reload the page and watch how the state of the element is retained
}
```


Let's make a normal website with client authorization:

``` rust
struct GlobalStore {
    username: String,
    logged_in: bool,
    client_token: String
}

impl GlobalStore {
    fn log_in(&mut self, api: Rustux::API) {
        if api.try_login(
            self.username.clone(),
            self.client_token.clone(),
        ) && self.logged_in == false {
            self.logged_in = true
        }
    }
}

fn main() {

    let my_app = Rustux::WebBuilder::new()
                    .with_favicon()
                    .with_name()
                    .with_global_store(
                        Rustux::GlobalStore::<GlobalStore>::new()
                    );

    my_app.start_yew_app::<LoginUserApp>();
}
```




It would also be nice to enable some server-side rendering, something like the primary component is rendered first and the rest of the page is rendered second.

It would also be nice to add a routing service with dynamic routes that are passed in as properties to the component they're routing to.

Something like:

``` rust
routes! {
    Home => "/home",
    DynamicRoute(String, i32, Enum::Thing) => "/home/?name={}/{}/zane={}",
    HellaDynamicRoute(String, i32, Enum::Thing) => "/home/?name={}?bob={}
    
}

g

pub struct RootComponent {

}
impl Model for RootComponent




```
