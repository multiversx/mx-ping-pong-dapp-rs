use yew::prelude::*;

use crate::components::{AdminPanel, AdminPanelAtHome};

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    html! {
        <>
            <AdminPanel />
        </>
    }
}

#[function_component(AdminPageAtHome)]
pub fn admin_page_at_home() -> Html {
    html! {
        <>
            <AdminPanelAtHome />
        </>
    }
}
