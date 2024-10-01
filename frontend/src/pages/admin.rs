use yew::prelude::*;

use crate::components::AdminPanel;

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    html! {
        <>
            <AdminPanel />
        </>
    }
}
