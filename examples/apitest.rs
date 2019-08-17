extern crate rustux;

fn main() {
    let state = MyState {};
    // AppActions::apply(AppActions)

}


pub struct MyState {
    
}

impl MyState {
    
}



pub struct ActionHistory {
    fn apply( ) {

    }

    fn undo() {

    }
}


pub enum AppActions {
    Action1(MyState),
    Action2(MyState),
    Action3(MyState),
    Action4(MyState),
    Action5(MyState),
    Action6(MyState),
}


impl AppActions {
}