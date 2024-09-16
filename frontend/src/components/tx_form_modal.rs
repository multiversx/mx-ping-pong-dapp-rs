use std::collections::HashMap;

use crate::components::Button;
use yew::prelude::*;
use yew_icons::IconId;

#[derive(Properties, PartialEq)]
pub struct TxFormModalProps {
    pub tx_name: String,
    pub on_close: Callback<MouseEvent>,
    pub on_submit: Callback<HashMap<String, String>>,
    pub is_visible: bool,
    pub input_fields: Vec<String>,
}

#[function_component(TxFormModal)]
pub fn tx_form_modal(props: &TxFormModalProps) -> Html {
    let form_values = use_state(HashMap::<String, String>::new);

    let handle_input = {
        let form_values = form_values.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let field_name = input.name();
            let value = input.value();
            form_values.set({
                let mut updated_values = (*form_values).clone();
                updated_values.insert(field_name, value);
                updated_values
            });
        })
    };

    let handle_submit = {
        let form_values = form_values.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            println!("Form values: {:?}", (*form_values).clone());
            on_submit.emit((*form_values).clone());
        })
    };

    let visibility_class = if props.is_visible { "show" } else { "hide" };

    html! {
        <div id={format!("{}Modal", props.tx_name.to_lowercase())} class={Classes::from(vec!["form-modal", visibility_class])}>
            <div class="form-modal-content">
                <div class="modal-header">
                    <h2> {format!("{} Transaction", &props.tx_name)} </h2>
                    <Button class_name="close-btn-form" button_type="button" on_click={props.on_close.clone()} icon={IconId::FontAwesomeSolidXmark} icon_class={"icon-close".to_string()} />
                </div>
                <form id={format!("{}Form", &props.tx_name.to_lowercase())} class={"tx-form"}>

                    { for props.input_fields.iter().enumerate().map(|(index, field)| html! {
                        <div key={index}> // Add a key to satisfy Yew's list rendering
                            <label for={format!("{}{}", field.clone().to_lowercase(), props.tx_name)}>{field.clone()}</label>
                            <input type="text" id={format!("{}{}", field.clone().to_lowercase(), props.tx_name)} name={field.clone()} oninput={handle_input.clone()} required=true /><br />
                        </div>
                    }) }
                    <div class="submit-button-wrapper">
                        <Button class_name="submit-tx-btn" button_type="button" on_click={handle_submit} text_content={"Submit".to_string()} />
                    </div>
                </form>
            </div>
        </div>
    }
}