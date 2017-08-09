
extern crate tradedoubler_api_client;
extern crate tokio_core;
extern crate pretty_env_logger;
use tokio_core::reactor::Core;
use tradedoubler_api_client::TradedoublerClient;
use tradedoubler_api_client::TradedoublerVoucherClient;

const TOKEN:&'static str="96CC0E0A10851500F10431D64EC5585BFC8597DF";  // DO NOT WORRY, THAT IS  FROM TD DOCUMENTATION
const VOUCHER_TOKEN:&'static str="B73AE30C600218523B4DE65A97C01A8309535FD5";  // DO NOT WORRY, THAT IS  FROM TD DOCUMENTATION

#[test]
fn setup_test() {
	pretty_env_logger::init().unwrap();
}

#[test]
fn should_download_products(){
	let mut core = Core::new().unwrap();
    let handle = core.handle();
	let client=TradedoublerClient::new(String::from(TOKEN),&handle);
	let work=client.get_products().page(2).page_size(10).run();
	let products=core.run(work).unwrap();
	assert_eq!(products.products.len(),10);
}
#[test]
fn should_download_vouchers(){
	let mut core = Core::new().unwrap();
    let handle = core.handle();
	let client=TradedoublerVoucherClient::new(String::from(VOUCHER_TOKEN),&handle);
	let work=client.get_vouchers().run();
	let vouchers=core.run(work).unwrap();
	assert!(vouchers.len()>=1);
}