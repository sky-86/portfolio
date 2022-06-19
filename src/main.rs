mod routes;
mod solutions;
mod external;

use yew::prelude::*;
use yew_router::prelude::*;

use routes::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <div class="navbar">
                    <ul>
                        <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Problems}>{"Problems"}</Link<Route>></li>
                    </ul>
                </div>
                <div>
                    <Switch<Route> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Root>();
}
