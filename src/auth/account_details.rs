use crate::{
    EmailAddress,
    components::{
        Button,
        button::{BtnColor, BtnVariant, ButtonIcon},
        input::FormField,
        input_class_default,
    },
    session::get_user,
    user::{AdapterUser, DeliveryDetails},
};
use leptos::prelude::*;

#[server]
async fn get_account_details() -> Result<DeliveryDetails, ServerFnError> {
    let user = get_user().await?;
    let output: DeliveryDetails = user.into();
    Ok(output)
}

#[server]
async fn update_account_details(details: DeliveryDetails) -> Result<(), ServerFnError> {
    use crate::db_init;

    let user = get_user().await?;
    let db = db_init().await?;

    let mut query = db
        .query(
            "UPDATE $userid SET
            firstName = $first_name,
            lastName = $last_name,
            email = $email,
            phone = $phone,
            address1 = $address1,
            address2 = $address2,
            address3 = $address3,
            postcode = $postcode,
            telephone = $telephone
            RETURN AFTER;",
        )
        .bind(("userid", user.id.clone()))
        .bind(("first_name", details.first_name))
        .bind(("last_name", details.last_name))
        .bind(("email", details.email.to_string()))
        .bind(("phone", details.phone))
        .bind(("address1", details.address1))
        .bind(("address2", details.address2))
        .bind(("address3", details.address3))
        .bind(("postcode", details.postcode))
        .bind(("telephone", details.telephone))
        .await?;

    let _updated_user: Option<AdapterUser> = query.take(0)?;

    Ok(())
}

#[component]
pub fn AccountForm() -> impl IntoView {
    let user_delivery_details = LocalResource::new(get_account_details);

    let details = RwSignal::new(None as Option<DeliveryDetails>);

    // Update parent signal whenever any field changes
    Effect::new(move |_| {
        if let Some(Ok(data)) = user_delivery_details.get() {
            details.set(Some(data));
        }
    });

    let update_action = ServerAction::<UpdateAccountDetails>::new();
    let is_saving = update_action.pending();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(delivery_details) = details.get() {
            update_action.dispatch(UpdateAccountDetails {
                details: delivery_details,
            });
        }
    };

    view! {
        <form on:submit=on_submit>
            <Transition>
                {move || match details.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(delivery_details) => {
                        view! {
                            <div class="grid grid-cols-12 gap-5">
                                <FormField
                                    label="First Name".to_string()
                                    class="col-span-6".to_string()
                                >
                                    <input
                                        value=delivery_details.first_name.clone()
                                        placeholder="First Name".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.first_name = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Last Name".to_string()
                                    class="col-span-6".to_string()
                                >
                                    <input
                                        value=delivery_details.last_name.clone()
                                        placeholder="Last Name".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.last_name = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField label="Email".to_string() class="col-span-6".to_string()>
                                    <input
                                        value=delivery_details.email.to_string()
                                        placeholder="Email".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.email = EmailAddress(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Phone Number".to_string()
                                    class="col-span-6".to_string()
                                >
                                    <input
                                        value=delivery_details.phone.clone()
                                        placeholder="Phone Number".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.phone = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Address".to_string()
                                    class="col-span-12".to_string()
                                >
                                    <input
                                        value=delivery_details.address1.clone()
                                        placeholder="Address".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.address1 = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Town/City".to_string()
                                    class="col-span-4".to_string()
                                >
                                    <input
                                        value=delivery_details.address2.clone()
                                        placeholder="Town/City".to_string()
                                        class=input_class_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.address2 = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Province/State".to_string()
                                    class="col-span-4".to_string()
                                >
                                    <input
                                        class=input_class_default()
                                        value=delivery_details.address3.clone()
                                        placeholder="Province/State".to_string()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.address3 = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <FormField
                                    label="Postcode".to_string()
                                    class="col-span-4".to_string()
                                >
                                    <input
                                        class=input_class_default()
                                        value=delivery_details.postcode.clone()
                                        placeholder="Postcode".to_string()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            details
                                                .update(|d| {
                                                    if let Some(delivery) = d {
                                                        delivery.postcode = Some(value);
                                                    }
                                                });
                                        }
                                    />
                                </FormField>

                                <div class="col-span-12 flex justify-end gap-3 pt-10">
                                    {move || {
                                        let is_saving_bool = is_saving.get();

                                        view! {
                                            <Button
                                                variant=BtnVariant::CallToAction
                                                color=BtnColor::Primary
                                                button_type="submit"
                                                disabled=is_saving_bool
                                                icon=ButtonIcon::Icon(phosphor_leptos::FLOPPY_DISK)
                                            >
                                                {move || {
                                                    if is_saving.get() { "Saving..." } else { "Save Changes" }
                                                }}
                                            </Button>
                                        }
                                    }}

                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Transition>

            {move || {
                if let Some(Ok(_)) = update_action.value().get() {
                    view! {
                        <div class="mt-4 p-4 bg-green-100 text-green-800 rounded">
                            "Account details saved successfully!"
                        </div>
                    }
                        .into_any()
                } else if let Some(Err(e)) = update_action.value().get() {
                    view! {
                        <div class="mt-4 p-4 bg-red-100 text-red-800 rounded">
                            "Error saving: " {e.to_string()}
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </form>
    }
}
