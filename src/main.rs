mod routes;
mod solutions;
mod external;
mod navbar;
mod components;

use yew::prelude::*;

use navbar::Navbar;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <>
            <Navbar />
            // footer goes here
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
