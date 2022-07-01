mod external;
mod components;
mod route;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::route::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <>
        // Implements browser function like history;
        <BrowserRouter>
            <Navbar />

            // renders the selected route here
            <div class={classes!("flex", "justify-center")}>
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
