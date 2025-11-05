use crate::components::{Button, button::BtnColor};
use leptos::prelude::*;

#[component]
pub fn DevAdmin() -> impl IntoView {
    let delete_tables = ServerAction::<DeleteAllTables>::new();

    view! {
        <div class="p-8">
            <h1 class="text-2xl font-bold mb-6">"Dev Admin"</h1>

            <div class="space-y-4">
                <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-4">
                    <h2 class="text-lg font-semibold mb-2">"Database Operations"</h2>

                    <ActionForm action=delete_tables>
                        <Button color=BtnColor::Error>
                            // button_type="submit"
                            "Delete Token Tables"
                        </Button>
                    </ActionForm>

                    {move || {
                        if let Some(result) = delete_tables.value().get() {
                            match result {
                                Ok(msg) => {
                                    view! {
                                        <div class="mt-2 text-green-600 dark:text-green-400">
                                            {msg}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <div class="mt-2 text-red-600 dark:text-red-400">
                                            {format!("Error: {}", e)}
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[server]
async fn delete_all_tables() -> Result<String, ServerFnError> {
    use crate::db_init;

    let db = db_init().await?;

    // Execute the DELETE query
    // let _result = db
    //     .query(
    //         "REMOVE table chain;
    //             REMOVE table key;
    //             REMOVE table optimized_media;
    //             REMOVE table session;
    //             REMOVE table test;
    //             REMOVE table token;
    //             REMOVE table token_pair;
    //             REMOVE table trading_bot;
    //             REMOVE table trading_bot_task;
    //             REMOVE table user;
    //             REMOVE table verificationToken;
    //             REMOVE table wallet;
    //             REMOVE table watch;
    //         ",
    //     )
    //     .await;

    let _result = db
        .query(
            "              
                REMOVE table token;
                REMOVE table token_pair;
            ",
        )
        .await;

    Ok("All tokens have been deleted successfully".to_string())
}
