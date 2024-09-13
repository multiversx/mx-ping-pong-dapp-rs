use std::rc::Rc;
use yew::prelude::*;

use crate::{
    components::Button,
    context::ConfigContext,
    requests::{query, transaction, TransactionType, QueryType},
};

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let context = use_context::<ConfigContext>().expect("Failed to get config context");

    let setup_result: UseStateHandle<String> = use_state(String::new);
    let transaction_result: UseStateHandle<String> = use_state(String::new);
    let user_addresses_result: UseStateHandle<String> = use_state(String::new);

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
                            set_deadline.emit(format!("Deadline: {}", result["response"].as_str().unwrap()));
                        }
                        Err(err) => {
                            log::error!("Query failed for deadline: {:?}", err);
                        }
                    },
                    QueryType::Timestamp => match query::get_timestamp(&config_borrowed).await {
                        Ok(result) => {
                            set_timestamp.emit(format!("Timestamp: {}", result["response"].as_str().unwrap()));
                        }
                        Err(err) => {
                            log::error!("Query failed for timestamp: {:?}", err);
                        }
                    },
                    QueryType::MaxFunds => match query::get_max_funds(&config_borrowed).await {
                        Ok(result) => {
                            set_max_funds.emit(format!("MaxFunds: {}", result["response"].as_str().unwrap()));
                        }
                        Err(err) => {
                            log::error!("Query failed for max funds: {:?}", err);
                        }
                    },
                    QueryType::PingAmount => match query::get_ping_amount(&config_borrowed).await {
                        Ok(result) => {
                            set_ping_amount.emit(format!("PingAmount: {}", result["response"].as_str().unwrap()));
                        }
                        Err(err) => {
                            log::error!("Query failed for ping amount: {:?}", err);
                        }
                    },
                    QueryType::UserAddresses => match query::get_user_addresses(&config_borrowed).await {
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
                    },
                }
            });
        })
    };

    let transaction_service = {
        let transaction_result = transaction_result.clone();
        let config = Rc::clone(&context.config);

        Callback::from(move |tx_type: TransactionType| {
            let transaction_result = transaction_result.clone();
            let config = Rc::clone(&config);

            log::info!("Transaction request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match tx_type {
                    TransactionType::Ping => match transaction::ping(&config_borrowed).await {
                        Ok(result) => {
                            transaction_result.set(format!(
                                "Pinged successfully with {} EGLD",
                                result["amount"].as_str().unwrap()
                            ));
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Ping failed!".to_string());
                        }
                    },
                    TransactionType::Pong => match transaction::pong(&config_borrowed).await {
                        Ok(_result) => {
                            transaction_result.set("Ponged successfully".to_string());
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Pong failed!".to_string());
                        }
                    },
                    TransactionType::PongAll => match transaction::pong_all(&config_borrowed).await {
                        Ok(_result) => {
                            transaction_result.set("Ponged all successfully".to_string());
                        }
                        Err(err) => {
                            log::error!("Transaction failed: {:?}", err);
                            transaction_result.set("Pong all failed!".to_string());
                        }
                    },
                }
            });
        })
    };

    let sc_setup_service = {
        let setup_result = setup_result.clone();
        let config = Rc::clone(&context.config);

        Callback::from(move |_| {
            let setup_result = setup_result.clone();
            let config = Rc::clone(&config);
            

            log::info!("SC setup request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();

                match transaction::sc_setup(&config_borrowed).await {
                    Ok(result) => {
                        setup_result.set(format!(
                            "New deployed address: {}",
                            result["address"].as_str().unwrap()
                        ));
                    }
                    Err(err) => {
                        log::error!("SC Setup failed: {:?}", err);
                        setup_result.set("SC Setup failed!".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class = "admin">
        <h2>{"Ping Pong Admin Panel"}</h2>
        <div class = "admin-panel-btns">
            <Button name = "Deadline" class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::Deadline)} />
            <Button name = "Timestamp" class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::Timestamp)} />
            <Button name = "Max Funds" class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::MaxFunds)} />
            <Button name = "Ping Amount" class_name = "query-btn" button_type = "button" on_click={query_service.reform(|_| QueryType::PingAmount)} />
            <Button name = "User Addresses" class_name = "query-btn" button_type="button" on_click={query_service.reform(|_| QueryType::UserAddresses)} />
            <Button name = "Ping" class_name = "transaction-btn" button_type = "button" on_click={transaction_service.reform(|_| TransactionType::Ping)} />
            <Button name = "Pong" class_name = "transaction-btn" button_type = "button" on_click={transaction_service.reform(|_| TransactionType::Pong)} />
            <Button name = "Pong all" class_name = "transaction-btn" button_type = "button" on_click={transaction_service.reform(|_| TransactionType::PongAll)} />
            <Button name = "SC Setup" class_name = "transaction-btn" button_type = "button" on_click={sc_setup_service.clone()} />
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
        </div>
    }
}
