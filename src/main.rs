mod external;
mod components;
mod route;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::route::*;

// helper function for creating classes with tailwind
// allows one long string of space seperated classes and makes a vec
pub fn class_helper(s: &str) -> Vec<&str> {
    s.split(' ').collect()
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <>
        // Implements browser function like history;
        <BrowserRouter>
            <Navbar />

            // renders the selected route here
            <main class={classes!(class_helper(
                "flex justify-center bg-darker h-full"
            ))}>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
