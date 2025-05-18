use serde::{Deserialize, Serialize};

// pub(crate) struct MPMReq {
//     raw: String,
// }

#[derive(Serialize, Deserialize)]
pub(crate) struct MPMResp {
    pub(crate) payload_format_indicator: String,
    pub(crate) point_of_initiation_method: String,
    pub(crate) global_unique_identifier: String,
    pub(crate) merchant_pan: String,
    pub(crate) merchant_id: String,
    pub(crate) merchant_criteria: String,
    pub(crate) merchant_account_information: String,
    pub(crate) merchant_category_code: String,
    pub(crate) transaction_currency: String,
    pub(crate) transaction_amount: String,
    pub(crate) tip_indicator: String,
    pub(crate) tip_fixed_value: String,
    pub(crate) tip_percentage_value: String,
    pub(crate) country_code: String,
    pub(crate) merchant_name: String,
    pub(crate) merchant_city: String,
    pub(crate) postal_code: String,
    pub(crate) additional_data_field: String,
    pub(crate) bill_number: String,
    pub(crate) mobile_number: String,
    pub(crate) store_label: String,
    pub(crate) loyalty_number: String,
    pub(crate) reference_label: String,
    pub(crate) customer_label: String,
    pub(crate) terminal_label: String,
    pub(crate) purpose_of_transaction: String,
    pub(crate) additional_consumer_data_request: String,
    pub(crate) crc: String,
    pub(crate) language_preference: String,
    pub(crate) merchant_name_alt: String,
    pub(crate) merchant_city_alt: String,
}

impl MPMResp {
    pub(crate) fn new() -> MPMResp {
        MPMResp {
            payload_format_indicator: String::new(),
            point_of_initiation_method: String::new(),
            global_unique_identifier: String::new(),
            merchant_pan: String::new(),
            merchant_id: String::new(),
            merchant_criteria: String::new(),
            merchant_account_information: String::new(),
            merchant_category_code: String::new(),
            transaction_currency: String::new(),
            transaction_amount: String::new(),
            tip_indicator: String::new(),
            tip_fixed_value: String::new(),
            tip_percentage_value: String::new(),
            country_code: String::new(),
            merchant_name: String::new(),
            merchant_city: String::new(),
            postal_code: String::new(),
            additional_data_field: String::new(),
            bill_number: String::new(),
            mobile_number: String::new(),
            store_label: String::new(),
            loyalty_number: String::new(),
            reference_label: String::new(),
            customer_label: String::new(),
            terminal_label: String::new(),
            purpose_of_transaction: String::new(),
            additional_consumer_data_request: String::new(),
            crc: String::new(),
            language_preference: String::new(),
            merchant_name_alt: String::new(),
            merchant_city_alt: String::new(),
        }
    }
}