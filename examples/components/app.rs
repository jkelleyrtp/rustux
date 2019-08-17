#![recursion_limit="128"]
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use std::collections::VecDeque;
// use std::collections::

struct StateController {
    state: State,
    all_states: VecDeque<State>
}


pub enum updates {
    Increment,
    Decrement,
    Undo,
    Redo
}

impl StateController {
    fn new() -> Self {
        Self {
            state: State { value: 0 },
            all_states: VecDeque::new()
        }    
    }

    fn update(&mut self, update: updates) {
        match update {
            updates::Increment => {
                self.state.value = self.state.value + 1;
            },
            updates::Decrement => {
                self.state.value = self.state.value - 1;
            },
            updates::Undo => {
                self.state = self.all_states.pop_front().unwrap();
                return 
            },
            // updates::Redo
            _ => {},
        }
        self.all_states.push_front(self.state.clone());
    }
}


#[derive(Clone, Default)]
pub struct State {
    value: i32
}


impl State {
    fn increment(mut self) -> Self {
        self.value = self.value + 1 ;
        self
    }
}


pub struct Model { 
    state_controller: StateController
}

// enum Msg {
//     Apply,
//     Undo
// }

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = updates;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state_controller: StateController::new()
            // state: State { value: 0 }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state_controller.update(msg);
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <div>
                <button onclick=|_| updates::Increment>{"Increment value "} </button>
                <button onclick=|_| updates::Decrement> {"Decrement value "} </button>
                <button onclick=|_| updates::Undo> {"Undo"} </button>
                <button onclick=|_| updates::Redo> {"Redo"} </button>

                <p> { format!("Current value: {:?} ",self.state_controller.state.value )  } </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}