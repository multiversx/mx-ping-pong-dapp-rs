use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;
use yew_icons::IconId;

use crate::{
    components::{Button, ContractAddressModal, TxFormModal, TxStatusModal},
    context::ConfigContext,
    requests::{query, transaction, QueryType, TransactionType},
};

pub enum ModalType {
    DeployModal,
    PingModal,
}

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let context = use_context::<ConfigContext>().expect("Failed to get config context");

    let setup_result = use_state(String::new);
    let transaction_result = use_state(String::new);
    let tx_status_modal_visible = use_state(|| false);
    let tx_status = use_state(String::new);
    let status_icon_id = use_state(|| IconId::FontAwesomeRegularHourglass);
    let user_addresses_result = use_state(String::new);
    let contract_address_modal_extended = use_state(|| false);
    let addr_modal_arrow_id = use_state(|| IconId::LucideMaximize2);
    let is_loading: UseStateHandle<bool> = use_state(|| false);

    let deploy_modal_visible = use_state(|| false);
    let ping_deploy_modal_visible = use_state(|| false);

    let query_service = {
        let config = Rc::clone(&context.config);
        let user_address = user_addresses_result.clone();
        let set_deadline = context.set_deadline.clone();
        let set_timestamp = context.set_timestamp.clone();
        let set_max_funds = context.set_max_funds.clone();
        let set_ping_amount = context.set_ping_amount.clone();

        Callback::from(move |query_type: QueryType| {
            let config = Rc::clone(&config);
            let set_deadline = set_deadline.clone();
            let set_timestamp = set_timestamp.clone();
            let set_max_funds = set_max_funds.clone();
            let set_ping_amount = set_ping_amount.clone();
            let set_user_address = user_address.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match query_type {
                    QueryType::Deadline => match query::get_deadline(&config_borrowed).await {
                        Ok(result) => {
                            set_deadline.emit(format!(
                                "Deadline: {}",
                                result["response"].as_str().unwrap()
                            ));
                        }
                        Err(err) => {
                            log::error!("Query failed for deadline: {:?}", err);
                        }
                    },
                    QueryType::Timestamp => match query::get_timestamp(&config_borrowed).await {
                        Ok(result) => {
                            set_timestamp.emit(format!(
                                "Timestamp: {}",
                                result["response"].as_str().unwrap()
                            ));
                        }
                        Err(err) => {
                            log::error!("Query failed for timestamp: {:?}", err);
                        }
                    },
                    QueryType::MaxFunds => match query::get_max_funds(&config_borrowed).await {
                        Ok(result) => {
                            set_max_funds.emit(format!(
                                "MaxFunds: {}",
                                result["response"].as_str().unwrap()
                            ));
                        }
                        Err(err) => {
                            log::error!("Query failed for max funds: {:?}", err);
                        }
                    },
                    QueryType::PingAmount => match query::get_ping_amount(&config_borrowed).await {
                        Ok(result) => {
                            set_ping_amount.emit(format!(
                                "PingAmount: {}",
                                result["response"].as_str().unwrap()
                            ));
                        }
                        Err(err) => {
                            log::error!("Query failed for ping amount: {:?}", err);
                        }
                    },
                    QueryType::UserAddresses => {
                        match query::get_user_addresses(&config_borrowed).await {
                            Ok(result) => {
                                if let Some(addresses) = result["response"].as_array() {
                                    let formatted_addresses = addresses
                                        .iter()
                                        .filter_map(|address| address.as_str())
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    set_user_address.set(formatted_addresses);
                                } else {
                                    log::error!("'response' field is not an array or is missing.");
                                }
                            }
                            Err(err) => {
                                log::error!("Query failed for user addresses: {:?}", err);
                            }
                        }
                    }
                }
            });
        })
    };

    let transaction_service = {
        let transaction_result = transaction_result.clone();
        let config = Rc::clone(&context.config);
        let tx_status_modal_visible = tx_status_modal_visible.clone();
        let tx_status = tx_status.clone();
        let status_icon_id = status_icon_id.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |tx_type: TransactionType| {
            let transaction_result = transaction_result.clone();
            let config = Rc::clone(&config);
            let tx_status_modal_visible = tx_status_modal_visible.clone();
            let tx_status = tx_status.clone();
            let status_icon_id = status_icon_id.clone();
            let is_loading = is_loading.clone();

            if *is_loading {
                log::info!("Transaction is already in progress");
                return;
            }

            is_loading.set(true);

            log::info!("Transaction request triggered");
            tx_status_modal_visible.set(true);
            tx_status.set("In progress...".to_string());
            status_icon_id.set(IconId::FontAwesomeRegularHourglass);

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match tx_type {
                    TransactionType::Ping(amount) => match transaction::ping(&config_borrowed, amount).await {
                        Ok(result) => {
                            transaction_result.set(format!(
                                "Pinged successfully with {} EGLD",
                                result["amount"].as_str().unwrap()
                            ));
                            tx_status_modal_visible.set(true);
                            tx_status.set("Success".to_string());
                            status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Ping failed!".to_string());
                            tx_status_modal_visible.set(true);
                            tx_status.set("Failed".to_string());
                            status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                        }
                    },
                    TransactionType::Pong => match transaction::pong(&config_borrowed).await {
                        Ok(_result) => {
                            transaction_result.set("Ponged successfully".to_string());
                            tx_status_modal_visible.set(true);
                            tx_status.set("Success".to_string());
                            status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Pong failed!".to_string());
                            tx_status_modal_visible.set(true);
                            tx_status.set("Failed".to_string());
                            status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                        }
                    },
                    TransactionType::PongAll => match transaction::pong_all(&config_borrowed).await
                    {
                        Ok(_result) => {
                            transaction_result.set("Ponged all successfully".to_string());
                            tx_status_modal_visible.set(true);
                            tx_status.set("Success".to_string());
                            status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Pong all failed!".to_string());
                            tx_status_modal_visible.set(true);
                            tx_status.set("Failed".to_string());
                            status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                        }
                    },
                }
                is_loading.set(false);
            });
        })
    };

    let sc_setup_service = {
        let setup_result = setup_result.clone();
        let config = Rc::clone(&context.config);
        let tx_status_modal_visible = tx_status_modal_visible.clone();
        let tx_status = tx_status.clone();
        let status_icon_id = status_icon_id.clone();
        let set_contract_address = context.set_contract_address.clone();
        let is_loading = is_loading.clone();
        let deploy_modal_visible = deploy_modal_visible.clone();

        Callback::from(move |form_values: HashMap<String, String>| {
            let setup_result = setup_result.clone();
            let config = Rc::clone(&config);
            let tx_status_modal_visible = tx_status_modal_visible.clone();
            let tx_status = tx_status.clone();
            let status_icon_id = status_icon_id.clone();
            let set_contract_address = set_contract_address.clone();
            let is_loading = is_loading.clone();
            let deploy_modal_visible = deploy_modal_visible.clone();

            if *is_loading {
                log::info!("Transaction is already in progress");
                return;
            }

            is_loading.set(true);

            let ping_amount = form_values
                .get("Ping amount")
                .cloned()
                .unwrap_or_else(|| "0".to_string());
            let max_funds = form_values
                .get("Max funds")
                .cloned()
                .unwrap_or_else(|| "0".to_string());
            let activation_timestamp = form_values
                .get("Activation timestamp")
                .cloned()
                .unwrap_or_else(|| "None".to_string());
            let duration = form_values
                .get("Duration")
                .cloned()
                .unwrap_or_else(|| "0".to_string());

            deploy_modal_visible.set(false);

            log::info!("SC setup request triggered");
            tx_status_modal_visible.set(true);
            tx_status.set("In progress...".to_string());
            status_icon_id.set(IconId::FontAwesomeRegularHourglass);

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match transaction::sc_setup(
                    &config_borrowed,
                    ping_amount,
                    max_funds,
                    activation_timestamp,
                    duration,
                )
                .await
                {
                    Ok(result) => {
                        let new_addr = result["address"].as_str().unwrap().to_string();
                        setup_result.set(new_addr.clone());
                        set_contract_address.emit(new_addr);
                        tx_status_modal_visible.set(true);
                        tx_status.set("Success".to_string());
                        status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                    }
                    Err(err) => {
                        log::error!("SC Setup failed: {:?}", err);
                        setup_result.set("SC Setup failed!".to_string());
                        tx_status_modal_visible.set(true);
                        tx_status.set("Failed".to_string());
                        status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                    }
                }
                is_loading.set(false);
            });
        })
    };

    let close_tx_status = {
        let modal_visible = tx_status_modal_visible.clone();
        Callback::from(move |_| {
            modal_visible.set(false);
        })
    };

    let toggle_tx_form_modal = {
        let deploy_modal_visible = deploy_modal_visible.clone();
        let ping_modal_visible = ping_deploy_modal_visible.clone();
        Callback::from(move |modal_type: ModalType| match modal_type {
            ModalType::DeployModal => {
                let deploy_modal_visible = deploy_modal_visible.clone();
                deploy_modal_visible.set(!*deploy_modal_visible);
            }
            ModalType::PingModal => {
                let ping_modal_visible = ping_modal_visible.clone();
                ping_modal_visible.set(!*ping_modal_visible);
            }
        })
    };

    let change_addr_modal_visibility = {
        let contract_address_modal_extended = contract_address_modal_extended.clone();
        let addr_modal_arrow_id = addr_modal_arrow_id.clone();

        Callback::from(move |_| {
            let contract_address_modal_extended = contract_address_modal_extended.clone();
            let addr_modal_arrow_id = addr_modal_arrow_id.clone();

            contract_address_modal_extended.set(!*contract_address_modal_extended);
            if !*contract_address_modal_extended {
                addr_modal_arrow_id.set(IconId::LucideMinimize2);
            } else {
                addr_modal_arrow_id.set(IconId::LucideMaximize2);
            }
        })
    };

    let handle_submit = {
        let ping_modal_visible = ping_deploy_modal_visible.clone();
        let transaction_service = transaction_service.clone();
        Callback::from(move |form_values: HashMap<String, String>| {
            if let Some(amount) = form_values.get("Amount") {
                transaction_service.emit(TransactionType::Ping(amount.clone()));
            }
            ping_modal_visible.set(false);
        })
    };

    html! {
        <div class = "admin">
        <h2>{"Ping Pong Admin Panel"}</h2>
        <div class = "admin-panel-btns">
            <div class = "query-btns">
                <Button class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::Deadline)} text_content={"Deadline".to_string()} />
                <Button class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::Timestamp)} text_content={"Timestamp".to_string()} />
                <Button class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::MaxFunds)} text_content={"Max Funds".to_string()} />
                <Button class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::PingAmount)} text_content={"Ping Amount".to_string()} />
                <Button class_name = "query-btn" button_type="button" on_click={query_service.reform(|_| QueryType::UserAddresses)} text_content={"User Addresses".to_string()} />
            </div>
            <div class = "transaction-btns">
                <Button class_name = "transaction-btn" button_type = "button" on_click={toggle_tx_form_modal.reform(|_| ModalType::PingModal)} disabled={*is_loading} text_content={"Ping".to_string()} />
                <Button class_name = "transaction-btn" button_type = "button" on_click={transaction_service.reform(|_| TransactionType::Pong)} disabled={*is_loading} text_content={"Pong".to_string()} />
                <Button class_name = "transaction-btn" button_type = "button" on_click={transaction_service.reform(|_| TransactionType::PongAll)} disabled={*is_loading} text_content={"PongAll".to_string()} />
                <Button class_name = "transaction-btn" button_type = "button" on_click={toggle_tx_form_modal.reform(|_| ModalType::DeployModal)} disabled={*is_loading} text_content={"SC Setup".to_string()} />
            </div>
        </div>
            {
                if !context.deadline.is_empty() {
                    html! {
                        <>
                            <p>{ &context.deadline }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }
            {
                if !context.timestamp.is_empty() {
                    html! {
                        <>
                            <p>{ &context.timestamp }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }
            {
                if !context.max_funds.is_empty() {
                    html! {
                        <>
                            <p>{ &context.max_funds }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }
            {
                if !context.ping_amount.is_empty() {
                    html! {
                        <>
                            <p>{ &context.ping_amount }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }
            {
                if !(*user_addresses_result).is_empty() {
                    let addresses: Vec<&str> = (*user_addresses_result).split('\n').collect();
                    html! {
                        <ul>
                            { for addresses.iter().map(|address| html! { <li>{ address }</li> }) }
                        </ul>
                    }
                } else {
                    html! { <></> }
                }
            }
            {
                if !(*transaction_result).is_empty() {
                    html! {
                        <>
                            <p>{ (*transaction_result).clone() }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }
            {
                if !(*setup_result).is_empty() {
                    html! {
                        <>
                            <p>{ (*setup_result).clone() }</p>
                        </>
                    }
                }
                else {
                    html! { <></> }
                }
            }

            <TxStatusModal status={(*tx_status).clone()} on_close={close_tx_status.clone()} is_visible={*tx_status_modal_visible} icon={*status_icon_id} />

            <ContractAddressModal address={context.contract_address} is_extended={*contract_address_modal_extended}
            on_extend={change_addr_modal_visibility.clone()} arrow_id={*addr_modal_arrow_id} />

            <TxFormModal tx_name={"Deploy".to_string()} on_close={toggle_tx_form_modal.reform(|_| ModalType::DeployModal)} on_submit={sc_setup_service.clone()} is_visible={*deploy_modal_visible}
                        input_fields={vec!["Ping amount".to_string(), "Max funds".to_string(), "Activation timestamp".to_string(), "Duration".to_string()]}
            />

            <TxFormModal tx_name={"Ping".to_string()} on_close={toggle_tx_form_modal.reform(|_| ModalType::PingModal)} on_submit={handle_submit} is_visible={*ping_deploy_modal_visible}
                        input_fields={vec!["Amount".to_string()]}
            />

        </div>



    }
}
