zanox-api-client
--------------------

Sample usage:

```rust
extern crate tradedoubler_api_client;
extern crate tokio_core;
use tokio_core::reactor::Core;
use tradedoubler_api_client::{TradedoublerClient,TradedoublerVoucherClient};

fn main(){   
	let mut core = Core::new().unwrap();
    let handle = core.handle();

	let client=TradedoublerClient::new(String::from("TOKEN"),&handle);
	let work_products=client.get_products().page(2).page_size(10).run();

	let voucher_client=TradedoublerVoucherClient::new(String::from("VOUCHER_TOKEN"),&handle);
	let work_vouchers=voucher_client.get_vouchers().run();

	let vouchers=core.run(work_vouchers).unwrap();
	let products=core.run(work_products).unwrap();
}
```