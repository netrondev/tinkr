use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{Button, button::BtnColor};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PayFastOptions {
    pub action_url: String,
    pub merchant_id: String,
    pub merchant_key: String,
}

impl Default for PayFastOptions {
    fn default() -> Self {
        Self {
            action_url: "https://sandbox.payfast.co.za/eng/process".into(),
            merchant_id: "10000100".into(),
            merchant_key: "notset".into(),
        }
    }
}

impl PayFastOptions {
    #[cfg(feature = "ssr")]
    pub fn from_env() -> Self {
        Self {
            action_url: std::env::var("TINKR_PAYFAST_URL").unwrap_or_default(),
            merchant_id: std::env::var("TINKR_PAYFAST_MERCHANT_ID").unwrap_or_default(),
            merchant_key: std::env::var("TINKR_PAYFAST_MERCHANT_KEY").unwrap_or_default(),
        }
    }
}

#[server]
async fn get_payfast_options() -> Result<PayFastOptions, ServerFnError> {
    Ok(PayFastOptions::from_env())
}

#[component]
pub fn PayFastButton(
    payment_order_title: String,
    payment_order_description: String,
    payment_uuid: String,
    payment_first_name: String,
    payment_last_name: String,
    payment_email: String,
    payment_telephone: String,
    payment_confirm_amount: f64,
    address: String,
    city: String,
    province: String,
    postal_code: String,
) -> impl IntoView {
    let originget = RwSignal::new(String::new());
    let optionsget = OnceResource::new(get_payfast_options());

    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            originget.set(
                window()
                    .location()
                    .origin()
                    .unwrap_or_else(|_| String::new()),
            );
        }
    });

    move || {
        let origin = originget.get();

        let payment_id = payment_uuid
            .split(':')
            .nth(1)
            .unwrap_or(&payment_uuid)
            .to_string();

        let return_url = format!("{}/payment/{}/success", origin, payment_id);
        let cancel_url = format!("{}/cart", origin);
        let notify_url = format!("{}/api/payment/notify", origin);
        let full_address = format!("{} , {}", address, province);
        let amount_formatted = format!("{:.2}", payment_confirm_amount);

        let options = match optionsget.get() {
            Some(Ok(zxc)) => zxc,
            _ => return view! { <div>"Loading..."</div> }.into_any(),
        };

        view! {
            <form action=options.action_url method="POST">
                <input type="hidden" name="merchant_id" prop:value=options.merchant_id />
                <input type="hidden" name="merchant_key" value=options.merchant_key />

                <input
                    type="hidden"
                    id="paymentFirstName"
                    name="name_first"
                    value=payment_first_name.clone()
                />
                <input
                    type="hidden"
                    id="paymentLastName"
                    name="name_last"
                    value=payment_last_name.clone()
                />
                <input
                    type="hidden"
                    id="paymentEmail"
                    name="email_address"
                    value=payment_email.clone()
                />
                <input
                    type="hidden"
                    id="paymentTelephone"
                    name="custom_int1"
                    value=payment_telephone.clone()
                />

                <input
                    type="hidden"
                    id="paymentConfirmAmount"
                    name="amount"
                    value=amount_formatted.clone()
                />

                <input type="hidden" name="item_name" value=payment_order_title.clone() />
                <input
                    type="hidden"
                    name="item_description"
                    value=payment_order_description.clone()
                />

                <input
                    type="hidden"
                    id="paymentUuid"
                    name="custom_str1"
                    value=payment_uuid.clone()
                />

                <input type="hidden" id="address" name="custom_str2" value=full_address.clone() />

                <input type="hidden" id="city" name="custom_str3" value=city.clone() />

                <input
                    type="hidden"
                    id="paymentTelephone2"
                    name="custom_str4"
                    value=payment_telephone.clone()
                />

                <input
                    type="hidden"
                    id="postal_code"
                    name="custom_str5"
                    value=postal_code.clone()
                />

                <input type="hidden" name="return_url" value=return_url.clone() />
                <input type="hidden" name="cancel_url" value=cancel_url.clone() />
                <input type="hidden" name="notify_url" value=notify_url.clone() />

                <Button prop:id="payment" prop:type="submit" color=BtnColor::Primary>
                    "Pay with Payfast"
                </Button>
            </form>
        }
        .into_any()
    }
}
