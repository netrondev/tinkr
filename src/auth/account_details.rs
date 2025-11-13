use crate::{
    EmailAddress,
    components::{
        Alert, AlertSeverity, Button,
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
pub fn DeliveryDetailsForm(
    /// The delivery details to display/edit
    details: Signal<DeliveryDetails>,
    /// Whether the form fields should be editable
    editable: Signal<bool>,
    /// Optional callback when details change (only used when editable=true)
    #[prop(optional)]
    on_change: Option<Callback<DeliveryDetails>>,
) -> impl IntoView {
    let input_class = move || {
        if editable.get() {
            input_class_default()
        } else {
            format!("{} bg-gray-100 cursor-default", input_class_default())
        }
    };

    let handle_change = move |field: &str, value: String| {
        if !editable.get() {
            return;
        }

        if let Some(callback) = on_change {
            let mut updated_details = details.get();
            match field {
                "first_name" => updated_details.first_name = Some(value),
                "last_name" => updated_details.last_name = Some(value),
                "email" => updated_details.email = EmailAddress(value),
                "phone" => updated_details.phone = Some(value),
                "address1" => updated_details.address1 = Some(value),
                "address2" => updated_details.address2 = Some(value),
                "address3" => updated_details.address3 = Some(value),
                "postcode" => updated_details.postcode = Some(value),
                _ => {}
            }
            callback.run(updated_details);
        }
    };

    view! {
        <div class="grid grid-cols-12 gap-5">
            <FormField label="First Name".to_string() class="col-span-6".to_string()>
                <input
                    value=move || details.get().first_name.clone()
                    placeholder="First Name".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("first_name", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Last Name".to_string() class="col-span-6".to_string()>
                <input
                    value=move || details.get().last_name.clone()
                    placeholder="Last Name".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("last_name", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Email".to_string() class="col-span-6".to_string()>
                <input
                    value=move || details.get().email.to_string()
                    placeholder="Email".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("email", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Phone Number".to_string() class="col-span-6".to_string()>
                <input
                    value=move || details.get().phone.clone()
                    placeholder="Phone Number".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("phone", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Address".to_string() class="col-span-12".to_string()>
                <input
                    value=move || details.get().address1.clone()
                    placeholder="Address".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("address1", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Town/City".to_string() class="col-span-4".to_string()>
                <input
                    value=move || details.get().address2.clone()
                    placeholder="Town/City".to_string()
                    class=input_class
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("address2", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Province/State".to_string() class="col-span-4".to_string()>
                <input
                    class=input_class
                    value=move || details.get().address3.clone()
                    placeholder="Province/State".to_string()
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("address3", event_target_value(&ev));
                    }
                />
            </FormField>

            <FormField label="Postcode".to_string() class="col-span-4".to_string()>
                <input
                    class=input_class
                    value=move || details.get().postcode.clone()
                    placeholder="Postcode".to_string()
                    disabled=move || !editable.get()
                    on:change=move |ev| {
                        handle_change("postcode", event_target_value(&ev));
                    }
                />
            </FormField>
        </div>
    }
}

#[component]
pub fn AccountForm() -> impl IntoView {
    let user_delivery_details = LocalResource::new(get_account_details);

    let details = RwSignal::new(None as Option<DeliveryDetails>);

    Effect::new(move |_| {
        if let Some(Ok(data)) = user_delivery_details.get() {
            details.set(Some(data));
        }
    });

    let on_details_change = Callback::new(move |updated: DeliveryDetails| {
        details.set(Some(updated.clone()));
        leptos::task::spawn_local(async move {
            let result = update_account_details(updated).await;
        });
    });

    view! {
        <Transition>
            {move || match details.get() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(delivery_details) => {
                    let details_signal = Signal::derive(move || {
                        details.get().unwrap_or(delivery_details.clone())
                    });
                    let editable = Signal::derive(|| true);
                    view! {
                        <DeliveryDetailsForm
                            details=details_signal
                            editable=editable
                            on_change=on_details_change
                        />
                    }
                        .into_any()
                }
            }}
        </Transition>
    }
}
