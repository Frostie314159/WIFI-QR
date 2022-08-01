/*
Copyright 2022 Frostie314159

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(name="WIFI-QR", author="Frostie314159", version="0.0.1", about="Creates QR-codes for logging into a WIFI-network.", long_about = None)]
struct Args{
    /// WIFI SSID
    #[clap(short, long, value_parser)]
    ssid: String,
    
    /// WIFI password
    #[clap(short, long, value_parser)]
    psw: Option<String>,

    /// WIFI security: If the Wifi is open omit this argument.
    #[clap(arg_enum, long, value_parser)]
    sec: Option<SecurityTypes>,

    /// Mark the WIFI as hidden.
    #[clap(short, long, action)]
    hidden: bool,

    /// Set the QR-Code ECC-Level. Low is the default.
    #[clap(arg_enum, short, long, value_parser, default_value_t=ECCLevel::Low)]
    ecc: ECCLevel
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ArgEnum)]
enum SecurityTypes{
    Wep,
    Wpa,
    Wpa2,
    Wpa3
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ArgEnum)]
enum ECCLevel{
    Low,
    Medium,
    Quartile,
    High
}
impl std::fmt::Display for SecurityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_bool_matrix_as_string(mat: Vec<Vec<bool>>) -> String {
    let mut output:String = String::new();
    for x in mat {
        for y in x{
            output += if y {"██"}else{"  "};
        }
        output += "\n";
    }
    output
}
fn assemble_qr_string(args: &Args) -> String{
    let psw:String = match args.psw.clone() {
        Some(x) => x,
        None => String::new()
    };
    let sec:String;
    if args.sec.is_none() && !psw.is_empty(){
        sec = SecurityTypes::Wpa2.to_string();
    }else if args.sec.is_some() && psw.is_empty(){
        panic!("No password was provided, but a security-standard was provided! Provided security-standard {}.", args.sec.unwrap());
    }else if args.sec.is_none() && psw.is_empty(){
        sec = String::new();
    }else{
        sec = args.sec.unwrap().to_string();
    }
    format!("WIFI:T:{};S:{};P:{};H:{};;", sec, args.ssid, psw, args.hidden)
}
fn main(){
    let args:Args = Args::parse();
    let qr_code:String = assemble_qr_string(&args);
    let qr_code:Vec<Vec<bool>> = qrcode_generator::to_matrix(qr_code, match args.ecc{
        ECCLevel::Low => qrcode_generator::QrCodeEcc::Low,
        ECCLevel::Medium => qrcode_generator::QrCodeEcc::Medium,
        ECCLevel::Quartile => qrcode_generator::QrCodeEcc::Quartile,
        ECCLevel::High => qrcode_generator::QrCodeEcc::High
    }).unwrap();
    
    print!("{}", get_bool_matrix_as_string(qr_code));
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_bool_matrix_string_conversion() {
        /*
         * Check that the bool matrix:
         * [true, false]
         * [false, true]
         * outputs as:
         * ██   
         *   ██
         */
        let test_matrix: Vec<Vec<bool>> = vec![vec![true, false], vec![false, true]];
        let correct_result = "██  \n  ██\n";
        assert_eq!(get_bool_matrix_as_string(test_matrix), correct_result);
    }
    #[test]
    fn test_no_psw() {
        let mut args = Args{
            ssid: String::from("Martin Router King"),
            psw: None,
            sec: None,
            hidden: false,
            ecc: ECCLevel::Low
        };
        assert_eq!(assemble_qr_string(&args), "WIFI:T:;S:Martin Router King;P:;H:false;;");
        args.hidden = true;
        assert_eq!(assemble_qr_string(&args), "WIFI:T:;S:Martin Router King;P:;H:true;;");
    }
    #[test]
    fn test_no_sec_with_psw() {
        let mut args = Args{
            ssid: String::from("Martin Router King"),
            psw: Some(String::from("password")),
            sec: None,
            hidden: false,
            ecc: ECCLevel::Low
        };
        assert_eq!(assemble_qr_string(&args), "WIFI:T:Wpa2;S:Martin Router King;P:password;H:false;;");
        args.hidden = true;
        assert_eq!(assemble_qr_string(&args), "WIFI:T:Wpa2;S:Martin Router King;P:password;H:true;;");
    }
    #[test]
    fn test_sec_with_psw() {
        let mut args = Args{
            ssid: String::from("Martin Router King"),
            psw: Some(String::from("password")),
            sec: Some(SecurityTypes::Wpa2),
            hidden: false,
            ecc: ECCLevel::Low
        };
        assert_eq!(assemble_qr_string(&args), "WIFI:T:Wpa2;S:Martin Router King;P:password;H:false;;");
        args.hidden = true;
        assert_eq!(assemble_qr_string(&args), "WIFI:T:Wpa2;S:Martin Router King;P:password;H:true;;");
    }
    #[should_panic]
    #[test]
    fn test_protected_with_no_psw() {
        
        let args = Args{
            ssid: String::from("Martin Router King"),
            psw: None,
            sec: Some(SecurityTypes::Wpa2),
            hidden: false,
            ecc: ECCLevel::Low
        };
        assemble_qr_string(&args);
    }
}