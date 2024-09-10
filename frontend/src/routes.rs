use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::AdminPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <AdminPage /> }, // currently no homepage
        Route::Admin => html! { <AdminPage /> },
    }
}
