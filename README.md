# BTCDomain Resolver
## Overview
Opensource Resolver is a resolver service used to resolve btcdomain written in Rust. 
You could check domain's unquiness and validity via this service.
Another usage is to provide private resolving service for your project.

## Prepartion

### Prepare a fully synced bitcoin fullnode first.

To do so, check https://bitcoin.org/en/full-node#other-linux-daemon

We have tested on bitcore 24.0.1, you could download from:

https://bitcoincore.org/en/download/

To check bitcore is fully synced, using:

`bitcoin-cli getblockcount`

if it returns the newest block number, bitcore is working well and fully synced!

### Prepare indexed ord wallet

We add some useful function for ord wallet to support the resolver.

See: https://github.com/btcdomain/ord_btcdomain

You could install from the released version

To check ord is installed well, try command:

`ord --version`

it should return 

`0.5.1 btcdomain`



## Install

You could install directly 
using install.sh

or build from source code.


## Build

```
cargo build --release
```

## API Method

###  ** Notice: This service will only select info that is currently valid! Any expire domain or invalid domain will be ignored! **

### Get Domain Info

To use this api in curl

`
curl localhost:8088/open_api/domain/game.btc
`

Result:

```
{
    "code":0,
    "data":[
        {
            "id":248,
            "inscribe_num":485202,
            "inscribe_id":"612eefa7774714217c05331d325ec3d876c6348172e07df905ccda42ead7c0f6i0",
            "sat":0,
            "domain_name":"game.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204429,
            "update_time":1678978204429
        }
    ],
    "message":""
}
```

## Get All Domain in address

`
127.0.0.1:8088/open_api/address/bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz
`

```
{
    "code":0,
    "data":[
        {
            "id":234,
            "inscribe_num":485188,
            "inscribe_id":"403ff754bf587b48bb06ffc49215930e108b96d3b437cff8a5c17559c81e4e01i0",
            "sat":0,
            "domain_name":"king.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978203968,
            "update_time":1678978203968
        },
        {
            "id":240,
            "inscribe_num":485194,
            "inscribe_id":"1a933636c10dfab87a0baf0b8c7abdb4590a0c5c874d63b87fd91f2f3e8b4d5ei0",
            "sat":0,
            "domain_name":"buybtc.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204153,
            "update_time":1678978204153
        },
        {
            "id":242,
            "inscribe_num":485196,
            "inscribe_id":"1b51eb19ca3206e2896c5b8c8c482c0f242d42343308a2382757e70839c7ba9bi0",
            "sat":0,
            "domain_name":"hodl.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204215,
            "update_time":1678978204215
        },
        {
            "id":243,
            "inscribe_num":485197,
            "inscribe_id":"5a3980ff39d3ed170ad117920ef12da0b95a5774a204abc87693b872f713efa2i0",
            "sat":0,
            "domain_name":"market.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204246,
            "update_time":1678978204246
        },
        {
            "id":244,
            "inscribe_num":485198,
            "inscribe_id":"bd3f0ae545675f60841fb3e9a1a3c1f5648e96963cfd66a6e75a8c9132d7efbdi0",
            "sat":0,
            "domain_name":"blockchain.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204278,
            "update_time":1678978204278
        },
        {
            "id":246,
            "inscribe_num":485200,
            "inscribe_id":"2a5073fb99c026b81b7781bcb3215366169ab6f948b228922c71ff51df7310d7i0",
            "sat":0,
            "domain_name":"bank.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204356,
            "update_time":1678978204356
        },
        {
            "id":248,
            "inscribe_num":485202,
            "inscribe_id":"612eefa7774714217c05331d325ec3d876c6348172e07df905ccda42ead7c0f6i0",
            "sat":0,
            "domain_name":"game.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978204429,
            "update_time":1678978204429
        },
        {
            "id":251,
            "inscribe_num":487749,
            "inscribe_id":"8ec45120dc69a7f32798c06fefc55d20fca40e34dd23574042c9ffc885d6a6e7i0",
            "sat":0,
            "domain_name":"news.btc",
            "address":"bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
            "create_time":1678978351482,
            "update_time":1678978351482
        }
    ],
    "message":""
}

```


