use crate::{components::Button, requests::QueryType};
use yew::prelude::*;
use yew_icons::IconId;

#[derive(Properties, PartialEq)]
pub struct QueryResponseModalProps {
    pub query_type: Option<QueryType>,
    pub on_close: Callback<MouseEvent>,
    pub is_visible: bool,
    pub response: String,
}

#[function_component(QueryResponseModal)]
pub fn query_response_modal(props: &QueryResponseModalProps) -> Html {
    let visibility_class = if props.is_visible { "show" } else { "hide" };
    let query_name = match props.query_type {
        Some(QueryType::Timestamp) => "Timestamp".to_string(),
        Some(QueryType::Deadline) => "Deadline".to_string(),
        Some(QueryType::UserAddresses) => "User Addresses".to_string(),
        Some(QueryType::MaxFunds) => "Max Funds".to_string(),
        Some(QueryType::PingAmount) => "Ping Amount".to_string(),
        None => "".to_string(),
    };

    html! {
      <div id={format!("{}Modal", query_name.clone().to_lowercase())} class={Classes::from(vec!["query-modal", visibility_class])}>
            <div class="query-modal-content">
                <div class="query-modal-header">
                    <h2> {format!("{} Response", &query_name)} </h2>
                    <Button class_name="close-btn-query" button_type="button" on_click={props.on_close.clone()} icon={IconId::FontAwesomeSolidXmark} icon_class={"icon-close".to_string()} />
                </div>

                <div class="query-modal-body">
                {
                    match query_name.as_str() {
                        "User Addresses" => {
                            if props.response == "No address has pinged yet".to_string() {
                                html! {
                                    <p>{&props.response}</p>
                                }
                            }
                            else {
                                let user_address_split = &props.response.split("\n").collect::<Vec<&str>>();
                                html! {
                                    <ul class="user-addresses-list">
                                        {for user_address_split.iter().map(|address| html! {
                                            <li>{address}</li>
                                        })}
                                    </ul>
                                }
                            }
                        }
                        _ => {
                            html! {
                                <p>{&props.response}</p>
                            }
                        }
                    }
                }
                </div>
            </div>
      </div>
    }
}
