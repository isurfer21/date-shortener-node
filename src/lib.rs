// #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Import
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// Export
const SYMBOLS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZàèìòùÀÈÌÒÙáéíóúÁÉÍÓÚâêîôûÂÊÔÛÎäëïöüÄËÏ";

#[wasm_bindgen]
pub fn encode(day:i32, month:i32, year:i32, steps:bool) -> String {
    let symbols: Vec<char> = SYMBOLS.chars().collect();
    let symbols_count = SYMBOLS.len();
    let enc_day = symbols[(day % symbols_count as i32) as usize];
    let enc_month = symbols[(month % symbols_count as i32) as usize];
    let enc_year: String;
    if year > 100 {
        let long_year: i32 = year;
        let century: i32 = long_year / 100;
        let short_year: i32 = long_year % 100;
        let enc_century = symbols[(century % symbols_count as i32) as usize];
        let enc_short_year = symbols[(short_year % symbols_count as i32) as usize];
        enc_year = format!("{}{}", enc_century, enc_short_year);
    } else {
        enc_year = symbols[(year % symbols_count as i32) as usize].to_string();
    }
    let dmy = format!("{}{}{}", enc_day, enc_month, enc_year);
    let dmy_with_steps = format!("{}-{}-{} -> {}.{}.{} -> {}.{}.{} -> {}", day, month, year, day, month, year, enc_day, enc_month, enc_year, dmy);
    return if steps { dmy_with_steps } else { dmy };
}

#[wasm_bindgen]
pub fn decode(encoded_date:String, steps:bool) -> String {
    let enc_date_sym: Vec<char> = encoded_date.chars().collect();
    let enc_day = enc_date_sym[0];
    let enc_month = enc_date_sym[1];
    let enc_year = &encoded_date[2..encoded_date.len()];
    let dec_day = SYMBOLS.find(enc_day).unwrap();
    let dec_month = SYMBOLS.find(enc_month).unwrap();
    let enc_date_len = enc_date_sym.len();
    let mut dec_year = SYMBOLS.find(enc_date_sym[enc_date_len-1]).unwrap();
    if enc_date_len > 3 {
        let dec_century = SYMBOLS.find(enc_date_sym[enc_date_len-2]).unwrap();
        dec_year = ((dec_century as i32 * 100) + dec_year as i32) as usize;
    }
    let dmy = format!("{}-{}-{}", dec_day, dec_month, dec_year);
    let dmy_with_steps = format!("{} -> {}.{}.{} -> {}.{}.{} -> {}", encoded_date, enc_day, enc_month, enc_year, dec_day, dec_month, dec_year, dmy);
    return if steps { dmy_with_steps } else { dmy };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        assert_eq!(encode(31, 12, 2035, false), "vckz");
        assert_eq!(encode(31, 12, 35, false), "vcz");
    }

    #[test]
    fn decode_works() {
        assert_eq!(decode("vckz".to_string(), false), "31-12-2035");
        assert_eq!(decode("vcz".to_string(), false), "31-12-35");
    }
}