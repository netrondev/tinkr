use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct PayFastOptions {
    pub action_url: &'static str,
    pub merchant_id: &'static str,
    pub merchant_key: &'static str,
}

impl PayFastOptions {
    pub fn new(sandbox: bool) -> Self {
        if sandbox {
            Self {
                action_url: "https://sandbox.payfast.co.za/eng/process",
                merchant_id: "10000100",
                merchant_key: "46f0cd694581a",
            }
        } else {
            Self {
                action_url: "https://www.payfast.co.za/eng/process",
                // scratchfixpro live payfast
                merchant_id: "12944071",
                merchant_key: "xvlw5hqgtqknh",
            }
        }
    }
}

#[component]
pub fn PayFastButton(
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
    #[prop(optional, default = false)] sandbox: bool,
) -> impl IntoView {
    let options = PayFastOptions::new(sandbox);

    // Get origin for return/cancel/notify URLs
    let origin = window()
        .location()
        .origin()
        .unwrap_or_else(|_| String::new());

    // Extract payment ID from UUID (split on ":" and take second part)
    let payment_id = payment_uuid
        .split(':')
        .nth(1)
        .unwrap_or(&payment_uuid)
        .to_string();

    let return_url = format!("{}/payment/{}/success", origin, payment_id);
    let cancel_url = format!("{}/cart", origin);
    let notify_url = format!("{}/api/payment/notify", origin);

    // Combine address with province
    let full_address = format!("{} , {}", address, province);

    // Format amount to 2 decimal places
    let amount_formatted = format!("{:.2}", payment_confirm_amount);

    view! {
        <form action=options.action_url method="POST">
            <input type="hidden" name="merchant_id" value=options.merchant_id />
            <input type="hidden" name="merchant_key" value=options.merchant_key />

            <input
                type="hidden"
                id="paymentFirstName"
                name="name_first"
                value=payment_first_name
            />
            <input
                type="hidden"
                id="paymentLastName"
                name="name_last"
                value=payment_last_name
            />
            <input
                type="hidden"
                id="paymentEmail"
                name="email_address"
                value=payment_email
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
                value=amount_formatted
            />

            <input type="hidden" name="item_name" value="ScratchFixPro" />
            <input type="hidden" name="item_description" value="PaintKit Order" />

            <input
                type="hidden"
                id="paymentUuid"
                name="custom_str1"
                value=payment_uuid
            />
            <input
                type="hidden"
                id="address"
                name="custom_str2"
                value=full_address
            />
            <input type="hidden" id="city" name="custom_str3" value=city />
            <input
                type="hidden"
                id="paymentTelephone2"
                name="custom_str4"
                value=payment_telephone
            />
            <input
                type="hidden"
                id="postal_code"
                name="custom_str5"
                value=postal_code
            />

            <input
                type="hidden"
                name="return_url"
                value=return_url
            />
            <input
                type="hidden"
                name="cancel_url"
                value=cancel_url
            />
            <input
                type="hidden"
                name="notify_url"
                value=notify_url
            />

            <button
                id="payment"
                class="rounded bg-blue-900 p-4 text-white hover:bg-blue-800"
            >
                {move || if sandbox {
                    "PAYMENT sandbox"
                } else {
                    "PAYMENT"
                }}
            </button>
        </form>
    }
}
