use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::AppError;

/// PayFast IPN (Instant Payment Notification) payload schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayFastNotify {
    pub m_payment_id: String,
    pub pf_payment_id: String,
    pub payment_status: String,
    pub item_name: String,
    pub item_description: String,
    pub amount_gross: String,
    pub amount_fee: String,
    pub amount_net: String,
    pub custom_str1: String, // order UUID
    pub custom_str2: String, // address
    pub custom_str3: String, // city
    pub custom_str4: String, // phone number
    pub custom_str5: String, // postal code
    pub custom_int1: String,
    #[serde(default)]
    pub custom_int2: String,
    #[serde(default)]
    pub custom_int3: String,
    #[serde(default)]
    pub custom_int4: String,
    #[serde(default)]
    pub custom_int5: String,
    pub name_first: String,
    pub name_last: String,
    #[serde(default)]
    pub cell_number: String,
    pub email_address: String,
    pub merchant_id: String,
    pub signature: String,
}

/// Payment record to be stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub order: crate::RecordId,
    pub amount_fee: String,
    pub amount_gross: String,
    pub amount_net: String,
    pub email_address: String,
    pub item_description: String,
    pub item_name: String,
    pub m_payment_id: String,
    pub merchant_id: String,
    pub name_first: String,
    pub name_last: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub payment_status: String,
    pub pf_payment_id: String,
    pub signature: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub extra: PayFastNotify,
}

#[cfg(feature = "ssr")]
pub async fn handle_payfast_notify(notify: PayFastNotify) -> Result<(), AppError> {
    use crate::RecordId;

    // Extract order ID from custom_str1 (format: "order:ID")
    let order_id = notify
        .custom_str1
        .split(':')
        .nth(1)
        .ok_or_else(|| AppError::GenericError("Invalid order UUID format".into()))?;

    // Initialize database connection
    let db = crate::db_init().await?;

    // Create payment record
    let payment_record = PaymentRecord {
        order: RecordId::from(("order", order_id)),
        amount_fee: notify.amount_fee.clone(),
        amount_gross: notify.amount_gross.clone(),
        amount_net: notify.amount_net.clone(),
        email_address: notify.email_address.clone(),
        item_description: notify.item_description.clone(),
        item_name: notify.item_name.clone(),
        m_payment_id: notify.m_payment_id.clone(),
        merchant_id: notify.merchant_id.clone(),
        name_first: notify.name_first.clone(),
        name_last: notify.name_last.clone(),
        phone_number: notify.custom_str4.clone(),
        payment_status: notify.payment_status.clone(),
        pf_payment_id: notify.pf_payment_id.clone(),
        signature: notify.signature.clone(),
        address: notify.custom_str2.clone(),
        city: notify.custom_str3.clone(),
        postal_code: notify.custom_str5.clone(),
        extra: notify.clone(),
    };

    // Insert payment record into database
    let _: Option<PaymentRecord> = db.create("payments").content(payment_record).await?;

    // If payment is complete, update order status and send confirmation email
    if notify.payment_status == "COMPLETE" {
        // Update order paid status
        let query = format!("UPDATE order:{} SET paid = true;", order_id);
        let _result = db.query(query).await?;

        // Send order confirmation email
        send_order_confirmation_email(&notify.email_address, &notify.name_first, order_id).await?;
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn send_order_confirmation_email(
    to_email: &str,
    client_name: &str,
    order_ref: &str,
) -> Result<(), AppError> {
    use crate::email::{EmailAddress, send_email};

    let email_addr = EmailAddress::from(to_email);
    let subject = format!("Scratch Fix Pro Order Confirmation REF: {}", order_ref);

    // Create email body (plain text for now - you can enhance this with HTML templates)
    let body = format!(
        r#"Hi {},

Thank you for your order!

Your order reference number is: {}

We'll send you another email once your order has been shipped.

Best regards,
Scratch Fix Pro Team
"#,
        client_name, order_ref
    );

    send_email(email_addr, &subject, &body).await?;

    Ok(())
}

/// Axum handler for PayFast IPN webhook
/// This receives POST form data from PayFast
#[cfg(feature = "ssr")]
pub async fn payfast_notify_handler(
    axum::Form(notify): axum::Form<PayFastNotify>,
) -> impl axum::response::IntoResponse {
    tracing::info!("Received PayFast notification: {:?}", notify);

    match handle_payfast_notify(notify).await {
        Ok(_) => {
            tracing::info!("PayFast notification processed successfully");
            (axum::http::StatusCode::OK, "OK")
        }
        Err(e) => {
            tracing::error!("Failed to process PayFast notification: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ERROR")
        }
    }
}
