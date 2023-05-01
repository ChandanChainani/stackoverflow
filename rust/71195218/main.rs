//> extern crate url;
//> extern crate base64;
//> use std::time::{SystemTime, UNIX_EPOCH};
//> use url::form_urlencoded;
//> use sha2::{Sha256, Sha512, Digest};
//> // use sha2::digest::KeyInit;
//> // use hmac::{Hmac, Mac};
//>
//>
//> extern crate ring;
//> extern crate data_encoding;
//>
//> use ring::{digest, hmac};
//> use data_encoding::BASE64;
//>
//> // type HmacSha512 = Hmac<Sha512>;
//>
//> fn main() {
//>     // import urllib.parse
//>     // import hashlib
//>     // import hmac
//>     // import base64
//>     // postdata = urllib.parse.urlencode(data)
//>     // encoded = (str(data['nonce']) + postdata).encode()
//>     // message = urlpath.encode() + hashlib.sha256(encoded).digest()
//>     // mac = hmac.new(base64.b64decode(secret), message, hashlib.sha512)
//>     // sigdigest = base64.b64encode(mac.digest())
//>     // print(\"API-Sign: {}\".format(sigdigest.decode()))
//>     // return sigdigest.decode()
//>     let urlpath = String::from("/0/private/Balance");
//>     println!("{}", urlpath);
//>     let api_sec = "<REPLACE>";
//>     println!("{}", api_sec);
//>     println!("Time: {:?}", UNIX_EPOCH);
//>     let nonce = SystemTime::now()
//>         .duration_since(UNIX_EPOCH)
//>         .expect("Time went backwards")
//>         .as_millis();
//>     println!("{}", nonce);
//>     let ordertype = "limit";
//>     println!("{}", ordertype);
//>     let pair = "XBTUSD";
//>     println!("{}", pair);
//>     let price: i32 = 37500;
//>     println!("{}", price);
//>     let type_ = "buy";
//>     println!("{}", type_);
//>     let volume = 1.25;
//>     println!("{}", volume);
//>     let secret = String::from("secret");
//>     let postdata: String = form_urlencoded::Serializer::new(urlpath).finish();
//>     let encoded = format!("{}{}", nonce, postdata);
//>     println!("{:?}", encoded);
//>     // let mut sha256_hasher = Sha256::new();
//>     // sha256_hasher.update(encoded.as_bytes());
//>     // let message = sha256_hasher.finalize();
//>     // println!("{:x}", message);
//>     // let b = base64::encode(secret.as_bytes());
//>     // let mut mac = HmacSha512::new_from_slice(format!("{}{}", b, message.into_bytes().to_string()).as_bytes());
//>     // let mut mac: String = HmacSha512::new(b.as_bytes());
//>
//>     let message = Sha256::digest(encoded.as_bytes());
//>     println!("{:x}", message);
//>     // let b = base64::encode(secret.as_bytes());
//>     // let mut mac = HmacSha512::new_from_slice(b);
//>     // let mut mac = HmacSha512::new_from_slice(format!("{}{}", b, str::from_utf8(message)).as_bytes());
//>     // println!("{}", mac.finalize());
//>     let secret_key = "secret";
//>     let payload = "test";
//>     // let signed_key = hmac::SigningKey::new(&digest::SHA256, secret_key.as_bytes());
//>     let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_ref());
//>     let signature = hmac::sign(&signed_key, payload.as_bytes());
//>     let b64_encoded_sig = BASE64.encode(signature.as_ref());
//>     println!("Base64: {}", b64_encoded_sig);
//>     assert_eq!(b64_encoded_sig, "Aymga2LNFrM+tnkr6MYLFY2Jou46h2/Omogeu0iMCRQ=");
//>
//> }





//>> extern crate url;
//>> extern crate base64;
//>> use std::time::{SystemTime, UNIX_EPOCH};
//>> use url::form_urlencoded;
//>> use sha2::{Sha256, Sha512, Digest};
//>> // use sha2::digest::KeyInit;
//>> // use hmac::{Hmac, Mac};
//>>
//>>
//>> extern crate ring;
//>> extern crate data_encoding;
//>>
//>> use ring::{digest, hmac};
//>> use data_encoding::BASE64;
//>> use std::collections::HashMap;
//>>
//>>
//>>
//>> // type HmacSha512 = Hmac<Sha512>;
//>>
//>> fn main() {
//>>     // import urllib.parse
//>>     // import hashlib
//>>     // import hmac
//>>     // import base64
//>>     // postdata = urllib.parse.urlencode(data)
//>>     // encoded = (str(data['nonce']) + postdata).encode()
//>>     // message = urlpath.encode() + hashlib.sha256(encoded).digest()
//>>     // mac = hmac.new(base64.b64decode(secret), message, hashlib.sha512)
//>>     // sigdigest = base64.b64encode(mac.digest())
//>>     // print(\"API-Sign: {}\".format(sigdigest.decode()))
//>>     // return sigdigest.decode()
//>>     let urlpath = String::from("/0/private/Balance");
//>>     println!("{}", urlpath);
//>>     let api_sec = "<REPLACE>";
//>>     println!("{}", api_sec);
//>>     println!("Time: {:?}", UNIX_EPOCH);
//>>     // let nonce = SystemTime::now()
//>>     //     .duration_since(UNIX_EPOCH)
//>>     //     .expect("Time went backwards")
//>>     //     .as_millis();
//>>     let nonce: i64 = 1645371362680;
//>>     println!("{}", nonce);
//>>     let ordertype = "limit";
//>>     println!("{}", ordertype);
//>>     let pair = "XBTUSD";
//>>     println!("{}", pair);
//>>     let price: i32 = 37500;
//>>     println!("{}", price);
//>>     let type_ = "buy";
//>>     println!("{}", type_);
//>>     let volume = 1.25;
//>>     println!("{}", volume);
//>>     let secret_key = String::from("secret");
//>>     // let postdata: String = form_urlencoded::Serializer::new(urlpath).finish();
//>>     // let encoded = format!("{}{}", nonce, postdata);
//>>
//>>     let mut data = HashMap::new();
//>>     data.insert("nonce", nonce.to_string());
//>>
//>>      let postdata: String = form_urlencoded::Serializer::new(String::new())
//>>         // .append_pair("nonce", &nonce.to_string())
//>>         .extend_pairs(data.iter())
//>>         .finish();
//>>     println!("Postdata: {:?}", postdata);
//>>     let encoded = format!("{}{}", nonce.to_string(), postdata);
//>>     println!("Encoded: {:?}", encoded);
//>>     // let mut sha256_hasher = Sha256::new();
//>>     // sha256_hasher.update(encoded.as_bytes());
//>>     // let message = sha256_hasher.finalize();
//>>     // println!("{:x}", message);
//>>     // let b = base64::encode(secret.as_bytes());
//>>     // let mut mac = HmacSha512::new_from_slice(format!("{}{}", b, message.into_bytes().to_string()).as_bytes());
//>>     // let mut mac: String = HmacSha512::new(b.as_bytes());
//>>     // let message = format!("{}{}", urlpath, Sha256::digest(encoded.as_bytes()));
//>>     // let message = urlpath.as_bytes().append(encoded.as_bytes());
//>>     let message: Vec<u8> = urlpath.as_bytes().iter().copied()
//>>         .chain(
//>>             Sha256::digest(encoded.as_bytes()).iter()
//>>                 .flat_map(|s| std::array::IntoIter::new(s.to_be_bytes()))
//>>         )
//>>         .collect();
//>>     println!("{:?}", message);
//>>     // let b = base64::encode(secret.as_bytes());
//>>     // let mut mac = HmacSha512::new_from_slice(b);
//>>     // let mut mac = HmacSha512::new_from_slice(format!("{}{}", b, str::from_utf8(message)).as_bytes());
//>>     // println!("{}", mac.finalize());
//>>     // let signed_key = hmac::SigningKey::new(&digest::SHA256, secret_key.as_bytes());
//>>
//>>     let signed_key = hmac::Key::new(hmac::HMAC_SHA512, secret_key.as_ref());
//>>     let signature = hmac::sign(&signed_key, &message);
//>>     let b64_encoded_sig = BASE64.encode(signature.as_ref());
//>>     println!("Base64: {}", b64_encoded_sig);
//>>     // assert_eq!(b64_encoded_sig, "Aymga2LNFrM+tnkr6MYLFY2Jou46h2/Omogeu0iMCRQ=");
//>>
//>> }



extern crate url;
extern crate base64;
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;
use sha2::{Sha256, Digest};
// use ::byte_strings::concat_bytes;

extern crate ring;
extern crate data_encoding;

use ring::hmac;
use data_encoding::BASE64;
use std::collections::HashMap;

fn main() {
    let urlpath = String::from("/0/private/Balance");
    // let nonce = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards")
    //     .as_millis();
    let nonce: &str = &(1645371362680 as i64).to_string();

    let mut data = HashMap::new();
    data.insert("nonce", nonce);

     let postdata: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(data.iter())
        .finish();
    let encoded = format!("{}{}", nonce, postdata);
    // let message: Vec<u8> = urlpath.as_bytes().iter().copied()
    //     .chain(
    //         Sha256::digest(encoded.as_bytes()).iter()
    //             .flat_map(|s| std::array::IntoIter::new(s.to_be_bytes()))
    //     )
    //     .collect();
    // println!("{:?}", concat_bytes!(urlpath.as_bytes(), Sha256::digest(encoded.as_bytes())));
    let message: Vec<u8> = [urlpath.as_bytes(), Sha256::digest(encoded.as_bytes()).as_ref()].concat();
    // println!("{:?}", message);

    let secret_key = String::from("secret");
    let signed_key = hmac::Key::new(hmac::HMAC_SHA512, secret_key.as_bytes());
    let signature = hmac::sign(&signed_key, &message);
    let b64_encoded_sig = BASE64.encode(signature.as_ref());
    println!("Output: {}", b64_encoded_sig);
}
