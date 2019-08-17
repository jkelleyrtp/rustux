## Things to do to make a progressive WebApp:

- [ ] Store assets in LocalStorage
- [ ] Handle offline mode
- [ ] Handle routing
- [ ] Handle falling offline and then back online
- [ ] Handle browser-related things like Favicon, title text, etc

## Rustux Stores

States of elements and pages are handled using Rustux Stores. Stores can be subscribed to

Three types of stores exist - Global, Window, and Local:
- Global stores manage


## More!

RxJS pattern for subscribing to application events.

Secretly wraps Yew's Agent-Worker model into something a little bit more ergonomic.

Multithreaded out of the box.

``` rust
struct Popup {
    // Attach the component to the state management system
    rx_link: Rustux::link,
}

impl Component for Popup {
    type Message = PageActions;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // Subscribe to events in the channel and trigger an update response
        link.subscribe( Rustux::Global::PageReloaded(meta), Self::Message::Reload(meta) );

        // Automatically mirror internal updates to the channel
        link.pubscribe(Self::Message::Update);
        Popup {}
    }

    fn update(&mut self, interaction: Self::Message) -> ShouldRender {
        use PageActions::*;
        match interaction {
            PageActions::Reload(meta) => {

            },
            UpdateCounter => {

            }
        }
        true
    }
}
impl Renderable<Popup> for Popup {    

}



let event_channel = Redux::from_event()


}

```