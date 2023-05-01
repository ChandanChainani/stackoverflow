use pyo3::prelude::*;
use std::{time::{SystemTime, UNIX_EPOCH}};
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {

    let urlpath = "/0/private/Balance";
    println!("{}", urlpath);
    let api_sec = "<REPLACE>";
    println!("{}", api_sec);
    let nonce =  SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    println!("{}", nonce);
    let ordertype = "limit";
    println!("{}", ordertype);
    let pair = "XBTUSD";
    println!("{}", pair);
    let price: i32 = 37500;
    println!("{}", price);
    let type_ = "buy";
    println!("{}", type_);
    let volume = 1.25;
    println!("{}", volume);
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "
import urllib.parse
import hashlib
import hmac
import base64

def get_kraken_signature(*args, **kwargs):
    if args != ():
        print('called with args', args)
    if kwargs != {}:
        print('called with kwargs', kwargs)
    if args == () and kwargs == {}:
        print('called with no arguments')
    if kwargs[\"cmd\"] == \"account_balance\":
        urlpath = kwargs[\"urlpath\"]
        data = {
            \"nonce\": kwargs[\"nonce\"],
        }
        secret = kwargs[\"secret\"]
    elif kwargs[\"cmd\"] == \"send_order\":
        urlpath = kwargs[\"urlpath\"]
        data = {
            \"nonce\": kwargs[\"nonce\"],
            \"ordertype\": kwargs[\"ordertype\"],
            \"pair\": kwargs[\"pair\"],
            \"price\": kwargs[\"price\"],
            \"type\": kwargs[\"type\"],
            \"volume\": kwargs[\"volume\"],
        }
        secret = kwargs[\"secret\"]
    else:
        exit(0)

    postdata = urllib.parse.urlencode(data)
    encoded = (str(data['nonce']) + postdata).encode()
    message = urlpath.encode() + hashlib.sha256(encoded).digest()
    mac = hmac.new(base64.b64decode(secret), message, hashlib.sha512)
    sigdigest = base64.b64encode(mac.digest())
    print(\"API-Sign: {}\".format(sigdigest.decode()))
    return sigdigest.decode()
",
            "",
            "",
        )?.getattr("get_kraken_signature")?.into();

        let a = urlpath.to_string();
        let b = nonce.to_string();
        let c = ordertype.to_string();
        let d = pair.to_string();
        let e = price.to_string();
        let f = type_.to_string();
        let g = volume.to_string();
        let h = api_sec.to_string();
        let kwargs = vec![("cmd", "account_balance"), ("urlpath", &a), ("nonce", &b), ("ordertype", &c), ("pair", &d), ("price", &e), ("type", &f), ("volume", &g), ("secret", &h)];
        let result = fun.call(py, (), Some(kwargs.into_py_dict(py)))?;

        println!("{}", result);
        Ok(())
    })
}
