use std::collections::HashMap;

use crate::service::qris::mpm_model::MPMResp;
use std::cell::RefCell;

pub struct MPMParser {
    var_map: RefCell<HashMap<String, String>>,
}

impl MPMParser {
    pub fn new() -> Self {
        MPMParser {
            var_map: RefCell::new(HashMap::new()),
        }
    }
}

thread_local!(static CONTENTS: MPMParser = MPMParser::new());

const SEPARATOR: String = String::new();
const MAX_INDEX: usize = 65;

fn parse_mpm(mut raw_data: &str, root_id: &str) {
    let mut index_id: usize = 0;

    while index_id <= MAX_INDEX {
        if raw_data.is_empty() {
            break;
        }

        let current_id = &raw_data[0..2];
        let expected_id = format!("{:02}", index_id);
        if current_id == expected_id {
            let data = get_content_mpm(&raw_data);
            put_content_mpm(&format!("{}{}{}", root_id, SEPARATOR, expected_id), &data);

            if root_id == "" && is_root_have_sub_id(current_id) {
                get_sub_content_mpm(&data.clone(), current_id);
            }
            raw_data = strip_content(raw_data, data.len());
        } else {
            put_content_mpm(&format!("{}{}{}", root_id, SEPARATOR, expected_id), "");
        }
        index_id += 1;
        if index_id == MAX_INDEX {
            break;
        }
    }
}

fn get_content_mpm(raw_data: &str) -> String {
    let mut str_result: String = String::new();

    if raw_data.len() >= 4 {
        let length_data_str = &raw_data[2..4];
        let length_data: usize = length_data_str.parse().unwrap_or(0);
        if raw_data.len() >= 4 + length_data {
            str_result = (&raw_data[4..4 + length_data]).to_string()
        }
    }
    str_result
}

fn get_sub_content_mpm(raw_data: &str, root_id: &str) {
    parse_mpm(raw_data, root_id);
}

fn put_content_mpm(key: &str, data: &str) {
    CONTENTS.with(|contents| {
        let mut borrow = contents.var_map.borrow_mut();
        borrow.insert(key.to_owned(), data.to_owned());
    });
}

fn is_root_have_sub_id(root_id: &str) -> bool {
    let root_id_int: usize = root_id.parse().unwrap_or(0);
    root_id_int > 2 && root_id_int < 51 || root_id_int == 62 || root_id_int == 64
}

fn strip_content(raw_data: &str, length: usize) -> &str {
    &raw_data[4 + length..]
}

pub(crate) fn get_result(raw_str: &str) -> MPMResp {
    parse_mpm(raw_str, "");
    let mut data: MPMResp = MPMResp::new();
    CONTENTS.with(|contents| {
        let map = contents.var_map.borrow();
        println!("{:?}", map);
        println!("{:?}", map.get("00"));
        data.payload_format_indicator = map.get("00").unwrap_or(&String::new()).to_string();
        data.point_of_initiation_method = map.get("01").unwrap_or(&String::new()).to_string();
        let mut i: usize = 2;
        while i <= 45 {
            let index = format!("{:02}", i);
            let value = map.get(&index).unwrap_or(&String::new()).to_string();
            if value.len() > 0 && value != "" {
                let guid = map.get(&format!("{}{}{}", index, SEPARATOR, "00")).unwrap_or(&String::new()).to_string();
                let mpan = map.get(&format!("{}{}{}", index, SEPARATOR, "01")).unwrap_or(&String::new()).to_string();
                let mid = map.get(&format!("{}{}{}", index, SEPARATOR, "02")).unwrap_or(&String::new()).to_string();
                let criteria = map.get(&format!("{}{}{}", index, SEPARATOR, "03")).unwrap_or(&String::new()).to_string();
                data.global_unique_identifier = guid;
                data.merchant_pan = mpan;
                data.merchant_id = mid;
                data.merchant_criteria = criteria;
            }
            i += 1;
        }

        data.merchant_account_information = map.get("51").unwrap_or(&String::new()).to_string();
        if data.merchant_account_information.len() > 0 && data.merchant_account_information != "" {
            data.global_unique_identifier = map.get(&format!("{}{}{}", "51", SEPARATOR, "00")).unwrap_or(&String::new()).to_string();
            data.merchant_id = map.get(&format!("{}{}{}", "51", SEPARATOR, "02")).unwrap_or(&String::new()).to_string();
            data.merchant_criteria = map.get(&format!("{}{}{}", "51", SEPARATOR, "00")).unwrap_or(&String::new()).to_string();
        }

        data.merchant_category_code = map.get("52").unwrap_or(&String::new()).to_string();
        data.transaction_currency = map.get("53").unwrap_or(&String::new()).to_string();
        data.transaction_amount = map.get("54").unwrap_or(&String::new()).to_string();
        let mut tmp = map.get("55").unwrap_or(&String::new()).to_string();
        if tmp.len() > 0 && tmp != "" {
            data.tip_indicator = tmp;
            if data.tip_indicator == "02" {
                data.tip_fixed_value = map.get("56").unwrap_or(&String::new()).to_string();
            } else if data.tip_indicator == "03" {
                data.tip_percentage_value = map.get("57").unwrap_or(&String::new()).to_string();
            }
        }

        data.country_code = map.get("58").unwrap_or(&String::new()).to_string();
        data.merchant_name = map.get("59").unwrap_or(&String::new()).to_string();
        data.merchant_city = map.get("60").unwrap_or(&String::new()).to_string();
        data.postal_code = map.get("61").unwrap_or(&String::new()).to_string();
        data.additional_data_field = map.get("62").unwrap_or(&String::new()).to_string();
        if data.additional_data_field.len() > 0 && data.additional_data_field != "" {
            data.bill_number = map.get(&format!("62{}{}", SEPARATOR, "01")).unwrap_or(&String::new()).to_string();
            data.mobile_number = map.get(&format!("62{}{}", SEPARATOR, "02")).unwrap_or(&String::new()).to_string();
            data.store_label = map.get(&format!("62{}{}", SEPARATOR, "03")).unwrap_or(&String::new()).to_string();
            data.loyalty_number = map.get(&format!("62{}{}", SEPARATOR, "04")).unwrap_or(&String::new()).to_string();
            data.reference_label = map.get(&format!("62{}{}", SEPARATOR, "05")).unwrap_or(&String::new()).to_string();
            data.customer_label = map.get(&format!("62{}{}", SEPARATOR, "06")).unwrap_or(&String::new()).to_string();
            data.terminal_label = map.get(&format!("62{}{}", SEPARATOR, "07")).unwrap_or(&String::new()).to_string();
            data.purpose_of_transaction = map.get(&format!("62{}{}", SEPARATOR, "08")).unwrap_or(&String::new()).to_string();
            data.additional_consumer_data_request = map.get(&format!("62{}{}", SEPARATOR, "09")).unwrap_or(&String::new()).to_string();
        }

        data.crc = map.get("63").unwrap_or(&String::new()).to_string();
        tmp = map.get("64").unwrap_or(&String::new()).to_string();
        if tmp.len() > 0 && tmp != "" {
            data.language_preference = map.get(&format!("64{}{}", SEPARATOR, "00")).unwrap_or(&String::new()).to_string();
            data.merchant_name_alt = map.get(&format!("64{}{}", SEPARATOR, "01")).unwrap_or(&String::new()).to_string();
            data.merchant_city_alt = map.get(&format!("64{}{}", SEPARATOR, "02")).unwrap_or(&String::new()).to_string();
        }
    });
    data
}
