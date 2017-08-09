zanox-api-client
--------------------

Sample usage:

```rust
extern crate tradedoubler_api_client;
extern crate tokio_core;
use tokio_core::reactor::Core;
use tradedoubler_api_client::{TradedoublerClient,TradedoublerVoucherClient};


#[test]
fn should_download_products(){   
	let mut core = Core::new().unwrap();
    let handle = core.handle();

	let client=TradedoublerClient::new(String::from("TOKEN"),&handle);
	let work=client.get_products().page(2).page_size(10).run();

	let client=TradedoublerVoucherClient::new(String::from("VOUCHER_TOKEN"),&handle);
	let work=client.get_vouchers().run();

	let vouchers=core.run(work).unwrap();
	let products=core.run(work).unwrap();
}
```