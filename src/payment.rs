use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct AmountWrapper {
    pub value: String,
    pub currency_code: String,
}

#[derive(Deserialize)]
pub(crate) struct PaymentEvent {
    pub id: String,
    pub status: String,
    pub amount: AmountWrapper,
    pub invoice_id: String,
    pub create_time: String,
}
