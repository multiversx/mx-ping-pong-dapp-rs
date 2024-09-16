use std::rc::Rc;
use yew::prelude::*;

use crate::{
    components::Button,
    context::ConfigContext,
    requests::{query, transaction, TransactionType},
};

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let context = use_context::<ConfigContext>().expect("Failed to get config context");

    let setup_result = use_state(String::new);
    let transaction_result = use_state(String::new);

    let query_service = {
        let config = Rc::clone(&context.config);
        let set_deadline = context.set_deadline.clone();

        Callback::from(move |_| {
            let config = Rc::clone(&config);
            let set_deadline = set_deadline.clone();

            log::info!("Query request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                let config_borrowed = config.borrow().clone();
                match query::get_deadline(&config_borrowed).await {
                    Ok(result) => {
                        set_deadline.emit(result);
                    }
                    Err(err) => {
                        log::error!("Query failed: {:?}", err);
                    }
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
                <Button name="Query" class_name="query-btn" button_type = "button" on_click={query_service.clone()} />
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
                    html! {
                        <>
                        </>
                    }
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
                    html! {
                        <>
                        </>
                    }
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
                    html! {
                        <>
                        </>
                    }
                }
            }
        </div>
    }
}
