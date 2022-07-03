use yew::prelude::*;
use yew_router::prelude::*;
use yew_octicons::{IconKind, Icon};

use crate::Route;

// render a navigation bar on the left side
#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <>
            <div class={classes!("fixed", "top-0", "left-0", 
                "h-screen", "w-16", "flex", "flex-col",
                "bg-darkest", "shadow-lg")}>

                <div>
                    <Link<Route> to={Route::Home}>
                        <SideBarIcon icon={IconKind::Home} 
                            text={Some("Home".to_string())} />
                    </Link<Route>>
                    <Link<Route> to={Route::Problems}>
                        <SideBarIcon icon={IconKind::CodeReview} 
                            text={Some("Leetcode Problems".to_string())} />
                    </Link<Route>>
                    <Link<Route> to={Route::Projects}>
                        <SideBarIcon icon={IconKind::Gear} 
                            text={Some("Projects".to_string())} />
                    </Link<Route>>
                    <a href="https://github.com/sky-86">
                        <SideBarIcon icon={IconKind::GitMerge} 
                            text={Some("GitHub".to_string())} />
                    </a>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SideBarIconProps {
    icon: IconKind,
    text: Option<String>,
}

#[function_component(SideBarIcon)]
fn side_bar_icon(props: &SideBarIconProps) -> Html {
    html! {
        <div class={classes!("sidebar-icon", "group")}>
            {Icon::new_big(props.icon)}

            if let Some(text) = &props.text {
                <span class={classes!("sidebar-tooltip", "group-hover:visible")}>
                    {text}
                </span>
            }
        </div>
    }
}
