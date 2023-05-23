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

Check the link: https://github.com/btcdomain/ord_btcdomain

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

***Note: This service will only select information that is currently valid! Any expired or invalid domains will be ignored!**

You can use our pre-built server by accessing the following endpoints:

```
http://btcdomains.io/open_api/domain/{domain_name}
http://btcdomains.io/open_api/domain_detail/{domain_name}
http://btcdomains.io/open_api/address/{address}

```
## Get Domain Address

### Endpoint: 

`http://btcdomains.io/open_api/domain/{domain_name}`

### Description:

BTC wallet address of the domain owner.

### Example usage with curl:

`curl http://btcdomains.io/open_api/domain/game.btc`

### Example response:

```
{
	"code": 0,
	"data": {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz"
	},
	"message": ""
}
```
## Get Domain Info

### Endpoint: 

`http://btcdomains.io/open_api/domain_detail/{domain_name}`

### Description:

Retrieve information about a specific domain.

### Example usage with curl:

`curl http://btcdomains.io/open_api/domain_detail/game.btc`

### Example response:

```
{
	"code": 0,
	"data": {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678872452676,
		"domain_name": "game.btc",
		"expire_date": 1710408452676,
		"img_url": "https://btcdomains.io/images/domain/game.jpeg",
		"inscribe_id": "612eefa7774714217c05331d325ec3d876c6348172e07df905ccda42ead7c0f6i0",
		"inscribe_num": 485202,
		"register_date": 1678872452676
	},
	"message": ""
}
```
### Img_url Description:

The image that could be displayed on wallets or exchanges should have the format:` https://btcdomains.io/images/domain/{domainname}.jpeg.`
For example, if the domain_name is game.btc, the relative img_url should be: `https://btcdomains.io/images/domain/game.jpeg` 

## Get All Domains in Address

### Endpoint: 

`http://btcdomains.io/open_api/address/{address}`

### Description: 

Retrieve all domain names associated with a specific address.

### Example usage with curl:

`curl http://btcdomains.io/open_api/address/bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz`

### Example response:

```
{
	"code": 0,
	"data": [{
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678871463968,
		"domain_name": "buybtc.btc",
		"expire_date": 1710407463968,
		"img_url": "https://btcdomains.io/images/domain/buybtc.jpeg",
		"inscribe_id": "1a933636c10dfab87a0baf0b8c7abdb4590a0c5c874d63b87fd91f2f3e8b4d5ei0",
		"inscribe_num": 485194,
		"register_date": 1678871463968
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678871812269,
		"domain_name": "bank.btc",
		"expire_date": 1710407812269,
		"img_url": "https://btcdomains.io/images/domain/bank.jpeg",
		"inscribe_id": "2a5073fb99c026b81b7781bcb3215366169ab6f948b228922c71ff51df7310d7i0",
		"inscribe_num": 485200,
		"register_date": 1678871812269
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678872141613,
		"domain_name": "hodl.btc",
		"expire_date": 1710408141613,
		"img_url": "https://btcdomains.io/images/domain/hodl.jpeg",
		"inscribe_id": "1b51eb19ca3206e2896c5b8c8c482c0f242d42343308a2382757e70839c7ba9bi0",
		"inscribe_num": 485196,
		"register_date": 1678872141613
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678872293327,
		"domain_name": "king.btc",
		"expire_date": 1710408293327,
		"img_url": "https://btcdomains.io/images/domain/king.jpeg",
		"inscribe_id": "403ff754bf587b48bb06ffc49215930e108b96d3b437cff8a5c17559c81e4e01i0",
		"inscribe_num": 485188,
		"register_date": 1678872293327
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678872452676,
		"domain_name": "game.btc",
		"expire_date": 1710408452676,
		"img_url": "https://btcdomains.io/images/domain/game.jpeg",
		"inscribe_id": "612eefa7774714217c05331d325ec3d876c6348172e07df905ccda42ead7c0f6i0",
		"inscribe_num": 485202,
		"register_date": 1678872452676
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678872614476,
		"domain_name": "market.btc",
		"expire_date": 1710408614476,
		"img_url": "https://btcdomains.io/images/domain/market.jpeg",
		"inscribe_id": "5a3980ff39d3ed170ad117920ef12da0b95a5774a204abc87693b872f713efa2i0",
		"inscribe_num": 485197,
		"register_date": 1678872614476
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678891063614,
		"domain_name": "blockchain.btc",
		"expire_date": 1710427063614,
		"img_url": "https://btcdomains.io/images/domain/blockchain.jpeg",
		"inscribe_id": "bd3f0ae545675f60841fb3e9a1a3c1f5648e96963cfd66a6e75a8c9132d7efbdi0",
		"inscribe_num": 485198,
		"register_date": 1678891063614
	}, {
		"address": "bc1p3pn9degqgcf4gdtly75ce9zgxdykzsvdmtv8jdqy5ay29f53wvdq9jrlgz",
		"create_date": 1678954441901,
		"domain_name": "news.btc",
		"expire_date": 1710490441901,
		"img_url": "https://btcdomains.io/images/domain/news.jpeg",
		"inscribe_id": "8ec45120dc69a7f32798c06fefc55d20fca40e34dd23574042c9ffc885d6a6e7i0",
		"inscribe_num": 487749,
		"register_date": 1678954441901
	}],
	"message": ""
}
```

These are the main API methods available for the BTCDomain Resolver.
By implementing these methods in your project, you can effectively manage and resolve BTCDomain names.



