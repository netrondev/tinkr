use crate::{
    components::{heading::Heading, label::Label},
    RecordId,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Extra payment details from PayFast
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentExtra {
    pub amount_fee: String,
    pub amount_gross: String,
    pub amount_net: String,
    pub custom_int1: String,
    pub custom_int2: String,
    pub custom_int3: String,
    pub custom_int4: String,
    pub custom_int5: String,
    pub custom_str1: String,
    pub custom_str2: String,
    pub custom_str3: String,
    pub custom_str4: String,
    pub custom_str5: String,
    pub email_address: String,
    pub item_description: String,
    pub item_name: String,
    pub m_payment_id: String,
    pub merchant_id: String,
    pub name_first: String,
    pub name_last: String,
    pub payment_status: String,
    pub pf_payment_id: String,
    pub signature: String,
}

/// Payment record from database
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub id: RecordId,
    pub order: RecordId,
    pub address: String,
    pub amount_fee: String,
    pub amount_gross: String,
    pub amount_net: String,
    pub city: String,
    pub email_address: String,
    pub extra: PaymentExtra,
    pub item_description: String,
    pub item_name: String,
    pub m_payment_id: String,
    pub merchant_id: String,
    pub name_first: String,
    pub name_last: String,
    pub payment_status: String,
    pub pf_payment_id: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub postal_code: String,
    pub signature: String,
}

/// Fetch a payment from the database by order RecordId
#[cfg(feature = "ssr")]
pub async fn get_payment_by_order(order_id: RecordId) -> Result<Option<Payment>, ServerFnError> {
    use crate::db_init;

    let db = db_init().await?;

    let mut query = db
        .query("SELECT * FROM payments WHERE order = $order_id LIMIT 1")
        .bind(("order_id", order_id))
        .await?;

    let payment: Option<Payment> = query.take(0)?;

    Ok(payment)
}

#[cfg(feature = "ssr")]
pub async fn get_payment_by_id(payment_id: RecordId) -> Result<Option<Payment>, ServerFnError> {
    use crate::db_init;

    let db = db_init().await?;

    let payment: Option<Payment> = db.select(payment_id).await?;

    Ok(payment)
}

// #[cfg(feature = "ssr")]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::RecordId;

//     #[tokio::test]
//     #[ignore] // Remove this to run the test with a real database
//     async fn test_get_payment_by_order() -> Result<(), ServerFnError> {

//         let order_id = RecordId::from_table_key("order", "2ygpmy9d1nr00vksova8");
//         let payment = get_payment_by_order(order_id).await?;

//         assert!(payment.is_some());
//         if let Some(payment) = payment {
//             assert_eq!(payment.payment_status, "COMPLETE");
//             assert_eq!(payment.item_name, "ScratchFixPro");
//         }

//         Ok(())
//     }
// }

#[component]
pub fn PaymentView(payment: Payment) -> impl IntoView {
    view! {
        <div class="pt-5">
            <Heading>"Payment Details"</Heading>

            <section class="grid grid-cols-2">

                <div class="flex flex-col gap-3 py-10">

                    <div>
                        <Label>"Payment Status"</Label>
                        <span>{payment.payment_status}</span>
                    </div>

                    <div>
                        <Label>"Address"</Label>
                        <span>{payment.address}</span>
                    </div>

                    <div>
                        <Label>"City"</Label>
                        <span>{payment.city}</span>
                    </div>

                    <div>
                        <Label>"Postal Code"</Label>
                        <span>{payment.postal_code}</span>
                    </div>

                    <div>
                        <Label>"Client Email"</Label>
                        <span>{payment.email_address}</span>
                    </div>

                    <div>
                        <Label>"Phone"</Label>
                        <span>{payment.phone_number}</span>
                    </div>
                </div>

                <div class="flex flex-col gap-3 py-10">
                    <div>
                        <Label>"Amount Net"</Label>
                        <span>{payment.amount_net}</span>
                    </div>
                    <div>
                        <Label>"Amount Fee"</Label>
                        <span>{payment.amount_fee}</span>
                    </div>
                    <div>
                        <Label>"Amount Gross"</Label>
                        <span>{payment.amount_gross}</span>
                    </div>
                </div>

            </section>
        </div>
    }
    .into_view()
}
