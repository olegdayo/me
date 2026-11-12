use crate::pages::{cv::CV, general::General, not_found::NotFound, talks::Talks};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    General,
    #[at("/cv")]
    CV,
    #[at("/talks")]
    Talks,
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
        Route::CV => html! {
            <>
                <CV/>
            </>
        },
        Route::Talks => html! {
            <>
                <Talks/>
            </>
        },
        Route::NotFound => html! {
            <>
                <NotFound/>
            </>
        },
    }
}
