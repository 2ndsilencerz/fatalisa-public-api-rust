use std::collections::HashMap;
use std::string::ToString;

const EMPTY_STRING: &str = "";

pub struct Data {
    payload_format_indicator: String,
    application_template: String,
    application_definition_file_name: String,
    application_label: String,
    track2equivalent_data: String,
    application_pan: String,
    card_holder_name: String,
    language_preference: String,
    issuer_url: String,
    application_version_number: String,
    token_requester_id: String,
    payment_account_reference: String,
    last4digit_pan: String,
    application_specific_transparent_template: String,
    application_cryptogram: String,
    cryptogram_information_data: String,
    issuer_application_data: String,
    application_transaction_counter: String,
    application_interchange_profile: String,
    unpredictable_number: String,
    issuer_qrisdata: String
}

impl Data {
    pub fn set_contents(&mut self, qris_parsed: HashMap<String, String>) {
        self.payload_format_indicator = qris_parsed.get(&String::from("85"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_template = qris_parsed.get(&String::from("61"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_definition_file_name = qris_parsed.get(&String::from("4F"))
            .cloned().unwrap_or_else(||EMPTY_STRING.clone().to_string()).to_string();
        self.application_label = qris_parsed.get(&String::from("50"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.track2equivalent_data = qris_parsed.get(&String::from("57"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_pan = qris_parsed.get(&String::from("5A"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.card_holder_name = qris_parsed.get(&String::from("5F20"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.language_preference = qris_parsed.get(&String::from("5F2D"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.issuer_url = qris_parsed.get(&String::from("5F50"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_version_number = qris_parsed.get(&String::from("9F08"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.token_requester_id = qris_parsed.get(&String::from("9F19"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.payment_account_reference = qris_parsed.get(&String::from("9F24"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.last4digit_pan = qris_parsed.get(&String::from("9F25"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        //self.ApplicationSpecificTransparentTemplate = qrisParsed["63"]
        self.application_cryptogram = qris_parsed.get(&String::from("9F26"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.cryptogram_information_data = qris_parsed.get(&String::from("9F27"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.issuer_application_data = qris_parsed.get(&String::from("9F10"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_transaction_counter = qris_parsed.get(&String::from("9F36"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.application_interchange_profile = qris_parsed.get(&String::from("82"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.unpredictable_number = qris_parsed.get(&String::from("9F37"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.issuer_qrisdata = qris_parsed.get(&String::from("9F74"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
    }
}