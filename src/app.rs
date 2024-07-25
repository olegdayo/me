use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <body>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </body>
    }
}
