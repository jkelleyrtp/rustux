// #[macro_use]
// extern crate derive_builder;

// Just copy how rocket does routes with guards and shit, but instead of a webserver, it's specifically for the frontend
// Your project would have a routes.rs which is built into the Yew Router webworker. Yew Router would check 


#[RustuxGlobal]
struct ApplicationState {
    token: i32,
    special_info: i32,
    // routes! {
    //     "/home?a={}?b={jon}?c={fizzbuzz}" => Homepage(String, i32, u64),
    //     "/home?a={}?b={jon}?c={fizzbuzz}" => Homepage(String, i32, u64),
    //     "/home?a={}?b={jon}?c={fizzbuzz}" => Homepage(String, i32, u64),
    //     "/home?a={}?b={jon}?c={fizzbuzz}" => Homepage(String, i32, u64),
    // }
}
enum Pages {
    Homepage,
}
impl ApplicationState {

    #[ Derive (Routeable = "/<asdads>/<asdasd>/aasd")]
    fn homepage { self.router.set_view(Pages::Homepage) }

    #[ Derive (Routeable = "/<asdads>/<asdasd>/aasd")]
    fn homepage2() { self.router.set_view(Pages::Homepage) }

}

fn main() {

}


