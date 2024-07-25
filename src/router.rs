use crate::pages::{general::General, not_found::NotFound};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    General,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::General => html! {
            <>
                <General/>
            </>
        },
        Route::NotFound => html! {
            <>
                <NotFound/>
            </>
        },
    }
}
