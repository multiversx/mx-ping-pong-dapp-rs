use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;
use yew_icons::IconId;

use crate::{
    components::{
        Button, CallbackStruct, CallbackType, ContractAddressModal, ModalType, QueryType,
        TransactionType, TxFormModal, TxStatusModal,
    },
    context::ConfigContext,
    requests::{query, transaction},
};

#[function_component(AdminPanelAtHome)]
pub fn admin_panel_at_home() -> Html {
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

    let callback_service = {
        let config = Rc::clone(&context.config);
        let transaction_result = transaction_result.clone();
        let setup_result = setup_result.clone();
        let tx_status_modal_visible = tx_status_modal_visible.clone();
        let tx_status = tx_status.clone();
        let status_icon_id = status_icon_id.clone();
        let user_addresses_result = user_addresses_result.clone();
        let set_contract_address = context.set_contract_address.clone();
        let is_loading = is_loading.clone();
        let deploy_modal_visible = deploy_modal_visible.clone();
        let ping_modal_visible = ping_deploy_modal_visible.clone();

        Callback::from(move |callback_struct: CallbackStruct| {
            let config = Rc::clone(&config);
            let transaction_result = transaction_result.clone();
            let setup_result = setup_result.clone();
            let tx_status_modal_visible = tx_status_modal_visible.clone();
            let tx_status = tx_status.clone();
            let status_icon_id = status_icon_id.clone();
            let user_addresses_result = user_addresses_result.clone();
            let set_contract_address = set_contract_address.clone();
            let is_loading = is_loading.clone();
            let deploy_modal_visible = deploy_modal_visible.clone();
            let ping_modal_visible = ping_modal_visible.clone();

            if *is_loading {
                log::info!("A request is already in progress");
                return;
            }

            is_loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match callback_struct.callback_type {
                    CallbackType::Query(query_type) => match query_type {
                        QueryType::Deadline => {
                            if let Ok(result) = query::get_deadline(&config_borrowed).await {
                                log::info!("Received deadline: {:?}", result);
                            }
                        }
                        QueryType::Timestamp => {
                            if let Ok(result) = query::get_timestamp(&config_borrowed).await {
                                log::info!("Received timestamp: {:?}", result);
                            }
                        }
                        QueryType::MaxFunds => {
                            if let Ok(result) = query::get_max_funds(&config_borrowed).await {
                                log::info!("Received max funds: {:?}", result);
                            }
                        }
                        QueryType::PingAmount => {
                            if let Ok(result) = query::get_ping_amount(&config_borrowed).await {
                                log::info!("Received ping amount: {:?}", result);
                            }
                        }
                        QueryType::UserAddresses => {
                            if let Ok(result) = query::get_user_addresses(&config_borrowed).await {
                                if let Some(addresses) = result["response"].as_array() {
                                    let formatted_addresses = addresses
                                        .iter()
                                        .filter_map(|address| address.as_str())
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    user_addresses_result.set(formatted_addresses.clone());
                                    log::info!("Received addresses: {:?}", formatted_addresses);
                                }
                            }
                        }
                    },

                    CallbackType::Transaction(tx_type) => match tx_type {
                        TransactionType::Ping => {
                            if let Some(data) = callback_struct.callback_data {
                                let amount = data
                                    .get("Amount")
                                    .cloned()
                                    .unwrap_or_else(|| "0".to_string());
                                match transaction::ping(&config_borrowed, amount).await {
                                    Ok(result) => {
                                        transaction_result.set(format!(
                                            "Pinged successfully with {} EGLD",
                                            result["amount"].as_str().unwrap()
                                        ));
                                        tx_status.set("Success".to_string());
                                        status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                                    }
                                    Err(err) => {
                                        log::error!("Ping transaction failed: {:?}", err);
                                        transaction_result.set("Ping failed!".to_string());
                                        tx_status.set("Failed".to_string());
                                        status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                                    }
                                }
                            }
                        }
                        TransactionType::Deploy => {
                            if let Some(data) = callback_struct.callback_data {
                                let ping_amount = data
                                    .get("Ping amount")
                                    .cloned()
                                    .unwrap_or_else(|| "0".to_string());
                                let max_funds = data
                                    .get("Max funds")
                                    .cloned()
                                    .unwrap_or_else(|| "0".to_string());
                                let activation_timestamp = data
                                    .get("Activation timestamp")
                                    .cloned()
                                    .unwrap_or_else(|| "None".to_string());
                                let duration = data
                                    .get("Duration")
                                    .cloned()
                                    .unwrap_or_else(|| "0".to_string());

                                log::info!("{ping_amount} || {max_funds} || {activation_timestamp} || {duration}");

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
                                        let new_addr =
                                            result["address"].as_str().unwrap().to_string();
                                        setup_result.set(new_addr.clone());
                                        set_contract_address.emit(new_addr);
                                        tx_status.set("Success".to_string());
                                        status_icon_id.set(IconId::FontAwesomeSolidCircleCheck);
                                    }
                                    Err(err) => {
                                        log::error!("SC Setup failed: {:?}", err);
                                        setup_result.set("SC Setup failed!".to_string());
                                        tx_status.set("Failed".to_string());
                                        status_icon_id.set(IconId::FontAwesomeRegularCircleXmark);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },

                    CallbackType::Modal(modal_type) => match modal_type {
                        ModalType::DeployModal => {
                            deploy_modal_visible.set(!*deploy_modal_visible);
                        }
                        ModalType::PingModal => {
                            ping_modal_visible.set(!*ping_modal_visible);
                        }
                    },

                    _ => {}
                }

                is_loading.set(false);
            });
        })
    };

    // let close_tx_status = {
    //     let modal_visible = tx_status_modal_visible.clone();
    //     Callback::from(move |_| {
    //         modal_visible.set(false);
    //     })
    // };

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

    // let change_addr_modal_visibility = {
    //     let contract_address_modal_extended = contract_address_modal_extended.clone();
    //     let addr_modal_arrow_id = addr_modal_arrow_id.clone();

    //     Callback::from(move |_| {
    //         let contract_address_modal_extended = contract_address_modal_extended.clone();
    //         let addr_modal_arrow_id = addr_modal_arrow_id.clone();

    //         contract_address_modal_extended.set(!*contract_address_modal_extended);
    //         if !*contract_address_modal_extended {
    //             addr_modal_arrow_id.set(IconId::LucideMinimize2);
    //         } else {
    //             addr_modal_arrow_id.set(IconId::LucideMaximize2);
    //         }
    //     })
    // };

    html! {
            <div class="admin">
                <h2>{"Ping Pong Admin Panel"}</h2>
                <div class="admin-panel-btns">
                    <div class="query-btns">
                        <Button class_name="query-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_query(QueryType::Deadline))} text_content={"Deadline".to_string()} />
                        <Button class_name="query-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_query(QueryType::Timestamp))} text_content={"Timestamp".to_string()} />
                        <Button class_name="query-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_query(QueryType::MaxFunds))} text_content={"Max Funds".to_string()} />
                        <Button class_name="query-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_query(QueryType::PingAmount))} text_content={"Ping Amount".to_string()} />
                        <Button class_name="query-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_query(QueryType::UserAddresses))} text_content={"User Addresses".to_string()} />
                    </div>
                    <div class="transaction-btns">
                        <Button class_name="transaction-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_modal(ModalType::PingModal))} disabled={*is_loading} text_content={"Ping".to_string()} />
                        <Button class_name="transaction-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_transaction(TransactionType::Pong))} disabled={*is_loading} text_content={"Pong".to_string()} />
                        <Button class_name="transaction-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_transaction(TransactionType::PongAll))} disabled={*is_loading} text_content={"PongAll".to_string()} />
                        <Button class_name="transaction-btn" button_type="button" on_click={callback_service.reform(|_| CallbackStruct::new_modal(ModalType::DeployModal))} disabled={*is_loading} text_content={"SC Setup".to_string()} />
                    </div>
                </div>

                <TxFormModal
                    tx_name={"Deploy".to_string()}
                    on_close={callback_service.reform(|_| CallbackStruct::new_modal(ModalType::DeployModal))}
                    on_submit={callback_service.reform(|data| CallbackStruct::new_transaction_with_data(TransactionType::Deploy, Some(data)))}
                    is_visible={*deploy_modal_visible}
                    input_fields={vec!["Ping amount".to_string(), "Max funds".to_string(), "Activation timestamp".to_string(), "Duration".to_string()]}
                />

                <TxFormModal
                    tx_name={"Ping".to_string()}
                    on_close={callback_service.reform(|_| CallbackStruct::new_modal(ModalType::PingModal))}
                    on_submit={callback_service.reform(|data| CallbackStruct::new_transaction_with_data(TransactionType::Ping, Some(data)))}
                    is_visible={*ping_deploy_modal_visible}
                    input_fields={vec!["Amount".to_string()]}
                />
            </div>
        }
}
