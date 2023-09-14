use std::collections::HashMap;

const EMPTY_STRING: &str = "";
const SEPARATOR: &str = "";

pub struct Data {
    payload_format_indicator: String,
    point_of_initiation_method: String,
    global_unique_identifier: String,
    merchant_pan: String,
    merchant_id: String,
    merchant_criteria: String,
    merchant_account_information: String,
    merchant_category_code: String,
    transaction_currency: String,
    transaction_amount: String,
    tip_indicator: String,
    tip_fixed_value: String,
    tip_percentage_value: String,
    country_code: String,
    merchant_name: String,
    merchant_city: String,
    postal_code: String,
    additional_data_field: String,
    bill_number: String,
    mobile_number: String,
    store_label: String,
    loyalty_number: String,
    reference_label: String,
    customer_label: String,
    terminal_label: String,
    purpose_of_transaction: String,
    additional_consumer_data_request: String,
    crc: String,
    language_preference: String,
    merchant_name_alt: String,
    merchant_city_alt: String,
}

impl Data {
    pub fn set_contents(&mut self, qris_parsed: HashMap<String, String>) {
        self.payload_format_indicator = qris_parsed.get(&String::from("00"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.point_of_initiation_method = qris_parsed.get(&String::from("01"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        // loop from index 2 to 45
        for i in 2..=45 {
            // rust format i with %02d
            let index = format!("{:0>2}",i);
            let current_index_content = qris_parsed.get(&index)
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
            if current_index_content.len() > 0 && current_index_content != "" {
                let guid = qris_parsed.get(&(index.clone()+SEPARATOR+"00"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
                let mpan = qris_parsed.get(&(index.clone()+SEPARATOR+"01"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
                let mid = qris_parsed.get(&(index.clone()+SEPARATOR+"02"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
                let criteria = qris_parsed.get(&(index.clone()+SEPARATOR+"03"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
                self.global_unique_identifier = guid;
                self.merchant_pan = mpan;
                self.merchant_id = mid;
                self.merchant_criteria = criteria;
                break
            }
        }

        self.merchant_account_information = qris_parsed.get(&String::from("51"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        if self.merchant_account_information.len() > 0 && self.merchant_account_information != "" {
            self.global_unique_identifier = qris_parsed.get(&("51".to_owned()+SEPARATOR+"00"))
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
            self.merchant_id = qris_parsed.get(&("51".to_owned()+SEPARATOR+"02"))
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
            self.merchant_criteria = qris_parsed.get(&("51".to_owned()+SEPARATOR+"03"))
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        }

        self.merchant_category_code = qris_parsed.get(&String::from("52"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.transaction_currency = qris_parsed.get(&String::from("53"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.transaction_amount = qris_parsed.get(&String::from("54"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();

        let tag_55_content = qris_parsed.get(&String::from("55"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        if tag_55_content.len() > 0 && tag_55_content!= "" {
            self.tip_indicator = tag_55_content;
            if self.tip_indicator == "02" {
                self.tip_fixed_value = qris_parsed.get(&String::from("56"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
            } else if self.tip_indicator == "03" {
                self.tip_percentage_value = qris_parsed.get(&String::from("57"))
                    .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
            }
        }

        self.country_code = qris_parsed.get(&String::from("58"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.merchant_name = qris_parsed.get(&String::from("59"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.merchant_city = qris_parsed.get(&String::from("60"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        self.postal_code = qris_parsed.get(&String::from("61"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();

        self.additional_data_field = qris_parsed.get(&String::from("62"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        if self.additional_data_field.len() > 0 && self.additional_data_field != "" {
            self.get_bit62contents(qris_parsed.clone());
            if self.terminal_label.len() == 0 || self.terminal_label == "" {
                fn tmp(qris_parsed: HashMap<String, String>) -> String {
                    let key = format!("{}{}{}","62",SEPARATOR,"07");
                    let tmp = qris_parsed.get(&key)
                        .cloned().unwrap_or_else(|| EMPTY_STRING.to_string()).to_string();
                    if tmp.len() > 0 && tmp != "" {
                        return tmp;
                    }
                    return qris_parsed.get(&String::from("62"))
                        .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
                }
                // rust format self.terminal_label with %-16s
                self.terminal_label = format!("{:<16}", tmp(qris_parsed.clone()));
            }
        }

        self.crc = qris_parsed.get(&String::from("63"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        let tag_64_content = qris_parsed.get(&String::from("64"))
            .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        if tag_64_content.len() > 0 && tag_64_content!= "" {
            self.get_bit64contents(qris_parsed.clone());
        }
    }

    fn get_bit62contents(&mut self, qris_parsed: HashMap<String, String>) {
        // rust init array of 9 String
        let mut contents: Vec<String> = Vec::new();
        for i in 1..10 {
            let key: String = format!("{}{}{}","62",SEPARATOR,format!("{:0>2}",i));
            contents[i-1] = qris_parsed.get(&key)
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        }
        self.bill_number = contents[0].to_owned();
        self.mobile_number = contents[1].to_owned();
        self.store_label = contents[2].to_owned();
        self.loyalty_number = contents[3].to_owned();
        self.reference_label = contents[4].to_owned();
        self.customer_label = contents[5].to_owned();
        self.terminal_label = contents[6].to_owned();
        self.purpose_of_transaction = contents[7].to_owned();
        self.additional_consumer_data_request = contents[8].to_owned();
    }

    fn get_bit64contents(&mut self, qris_parsed: HashMap<String, String>) {
        let mut contents: Vec<String> = Vec::new();
        for i in 0..3 {
            let current_index = format!("{}{}{}","64",SEPARATOR,format!("{:0>2}",i));
            contents[i] = qris_parsed.get(&current_index)
                .cloned().unwrap_or_else(||EMPTY_STRING.to_string()).to_string();
        }
        self.language_preference = contents[0].to_owned();
        self.merchant_name_alt = contents[1].to_owned();
        self.merchant_city_alt = contents[2].to_owned();
    }
}