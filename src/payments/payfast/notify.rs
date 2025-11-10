use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::AppError;

use crate::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Complete,
    Failed,
    Cancelled,
}

/// PayFast IPN (Instant Payment Notification) payload schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandlePayfastResult {
    pub payment_status: PaymentStatus,
    pub order_id: RecordId,
    pub payment: Option<PaymentRecord>,
}

#[cfg(feature = "ssr")]
pub async fn handle_payfast_notify_internal(
    notify: PayFastNotify,
) -> Result<HandlePayfastResult, AppError> {
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
    let payment = db.create("payments").content(payment_record).await?;

    // If payment is complete, update order status
    if notify.payment_status == "COMPLETE" {
        // Update order paid status
        let query = format!("UPDATE order:{} SET paid = true;", order_id);
        let _result = db.query(query).await?;

        return Ok(HandlePayfastResult {
            payment_status: PaymentStatus::Complete,
            order_id: RecordId::from(("order", order_id)),
            payment,
        });
    }

    Ok(HandlePayfastResult {
        payment_status: PaymentStatus::Failed,
        order_id: RecordId::from(("order", order_id)),
        payment,
    })
}

#[cfg(feature = "ssr")]
pub async fn handle_payfast_notify(notify: PayFastNotify) -> Result<(), AppError> {
    handle_payfast_notify_internal(notify).await?;

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
