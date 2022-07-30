use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::{EnumString, Display};

use crate::route::Route;

// all included projects
#[derive(Clone, Debug, PartialEq, EnumString, Display)]
pub enum ProjectEnum {
    SortingVisual,
    MediaPlayer,
    Portfolio,
    Contacts,
}
 
#[function_component(ProjectList)]
pub fn project_list() -> Html {
    html! {
        <div>
            <h1>{ "Projects" }</h1>
            <ul>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::SortingVisual}}>
                    {"Sorting Visualizer"}
                </Link<Route>></li>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::MediaPlayer}}>
                    {"Media Player"} 
                </Link<Route>></li>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::Portfolio}}>
                    {"Portfolio"} 
                </Link<Route>></li>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::Contacts}}>
                    {"Contacts"} 
                </Link<Route>></li>
             </ul>
        </div>
    }
}
