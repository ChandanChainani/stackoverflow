# from time import time
# >>> round(time() * 1000)
# 1645371289795
# >>> from datetime import datetime
# >>> dt = datetime.now()
# >>> dt.microsecond
# 445985
# >>> def TimestampMillisec64():
# ...     return int((datetime.utcnow() - datetime(1970, 1, 1)).total_seconds() * 1000)
# >>> TimestampMillisec64()
# 1645371362680


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
    if kwargs["cmd"] == "account_balance":
        urlpath = kwargs["urlpath"]
        data = {
            "nonce": kwargs["nonce"],
        }
        secret = kwargs["secret"]
    elif kwargs["cmd"] == "send_order":
        urlpath = kwargs["urlpath"]
        data = {
            "nonce": kwargs["nonce"],
            "ordertype": kwargs["ordertype"],
            "pair": kwargs["pair"],
            "price": kwargs["price"],
            "type": kwargs["type"],
            "volume": kwargs["volume"],
        }
        secret = kwargs["secret"]
    else:
        exit(0)
    print("data", data)
    postdata = urllib.parse.urlencode(data)
    print("postdata", postdata)
    encoded = (str(data['nonce']) + postdata).encode()
    print("encoded", encoded)
    print("haslib 256: ", hashlib.sha256(encoded).digest())
    message = urlpath.encode() + hashlib.sha256(encoded).digest()
    print("message", message)
    mac = hmac.new(base64.b64decode(secret), message, hashlib.sha512)
    sigdigest = base64.b64encode(mac.digest())
    print("API-Sign: {}".format(sigdigest.decode()))
    return sigdigest.decode()

urlpath = "/0/private/Balance"
api_sec = "<REPLACE>"
# nonce =  SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
nonce = 1645371362680
ordertype = "limit"
pair = "XBTUSD"
price = 37500
type_ = "buy"
volume = 1.25

get_kraken_signature(
    cmd="account_balance",
    urlpath=urlpath,
    api_sec=api_sec,
    nonce=nonce,
    ordertype=ordertype,
    pair=pair,
    type=type_,
    secret=base64.b64encode(b"secret"),
    volume=volume,
    price=price
)
