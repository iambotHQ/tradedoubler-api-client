#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
pub mod structs;
use  tokio_core::reactor::Handle;
use structs::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;
use hyper::{Client,StatusCode};
use std::clone::Clone;
use std::fmt;
use futures::Stream;
use std::collections::HashMap;
use std::error::Error;
use chrono::prelude::*;
const ALL_PRODUCTS_URL:&'static str="http://api.tradedoubler.com/1.0/productsUnlimited.json";
const ALL_VOUCHERS_URL:&'static str="https://api.tradedoubler.com/1.0/vouchers.json";

type RESTClient=Client<HttpsConnector<HttpConnector>>;
#[derive(Debug)]
pub struct TradedoublerClient{
 client:RESTClient,
 token:String
}

#[derive(Debug)]
pub struct TradedoublerVoucherClient{
 client:RESTClient,
 token:String
}

impl Clone for  TradedoublerClient{
    fn clone(&self) -> TradedoublerClient{
        TradedoublerClient{
        client:self.client.clone(),
        token:self.token.clone()
    }
}

    fn clone_from(&mut self, source: &Self) { 
        self.client=source.client.clone();
        self.token=source.token.clone();
     }
}


impl Clone for  TradedoublerVoucherClient{
    fn clone(&self) -> TradedoublerVoucherClient{
        TradedoublerVoucherClient{
        client:self.client.clone(),
        token:self.token.clone()
    }
}

    fn clone_from(&mut self, source: &Self) { 
        self.client=source.client.clone();
        self.token=source.token.clone();
     }
}

fn fetch_using(url:String,client:&RESTClient)->Box<Future<Item=String, Error=TradedoublerClientError>>{
       let uri = url.parse::<hyper::Uri>().unwrap();
       let out=client.get(uri).map_err(|e|{
            TradedoublerClientError::SendError(format!("{}",e))
        }).and_then(|r|{
            let status:StatusCode=r.status();
            r.body().concat2().map_err(|e|{
                TradedoublerClientError::ResponseReadError(format!("{}",e))
            }).map(move |d|{(status,d)})
        }).and_then(|(status,data)|{
           String::from_utf8((&data).to_vec()).map_err(|e|{
                TradedoublerClientError::ResponseReadError(format!("{}",e))
            }).map(move |str|{
                (status,str)
            })
        }).and_then(|(status,str)|{
            match status{
                StatusCode::Ok => Ok(str),
                _ => {
                    Err(TradedoublerClientError::ServerError(status,str))
                }
            }
        });
        Box::new(out)
}

trait ToMatrixParams {
    fn get_params(&self)->String;
}

impl ToMatrixParams for HashMap<String,String>{
    fn get_params(&self)->String{
        let mut out:String=String::new();
        for (k,v) in self{
            out.push_str(&format!(";{}={}",k,v))
        }
        out
    }
}

#[derive(Debug, Clone)]
pub enum TradedoublerClientError{
	SendError(String),
	ResponseReadError(String),
	DeserializationError(String),
	ServerError(StatusCode,String)
}

impl fmt::Display for TradedoublerClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self)
    }
}

impl Error for TradedoublerClientError {
    fn description(&self) -> &str {
        "Tradedouble client error!"
    }

}
impl TradedoublerClient{

    pub fn new(token:String,handle:&Handle)->TradedoublerClient{    
        let client = hyper::Client::configure().connector(HttpsConnector::new(4, handle).unwrap()).build(handle);
         TradedoublerClient{
            client:client,
            token:token
        }
    }
    pub fn get_products(&self)->TradedoublerProductsRequest{
        TradedoublerProductsRequest{
            client:self,
            url:ALL_PRODUCTS_URL,
            token:self.token.clone(),
            params:HashMap::new()
        }
    }

    fn get_products_using(&self,url:String)->Box<Future<Item=Products, Error=TradedoublerClientError>>{
        info!("Fetching products using URL {:?}",url);

       let out=fetch_using(url,&self.client).and_then(|str|{
             let out:Result<Products,TradedoublerClientError>=serde_json::from_str(&str).map_err(|e|{
                TradedoublerClientError::DeserializationError(format!("{} {:?}",e,str))
            });
             out
        });
        Box::new(out)
    }

}

#[derive(Debug, Clone)]
pub struct TradedoublerProductsRequest<'a>{
    client:&'a TradedoublerClient,
    url:&'a str,
    token:String,
    params:HashMap<String,String>
}

impl <'a> TradedoublerProductsRequest<'a>{
    pub fn run(self)->Box<Future<Item=Products, Error=TradedoublerClientError>>{
        self.client.get_products_using(format!("{}{}?token={}",self.url,self.params.get_params(),self.token))
    }

    pub fn param(mut self,key:&str,value:String)->TradedoublerProductsRequest<'a>{ //expopsing function in order to fully utilize TD API
        self.params.insert(String::from(key),value);
        self
    }

    pub fn page(self,page:i32)->TradedoublerProductsRequest<'a>{
        self.param("page",page.to_string())
    }

    pub fn page_size(self,page_size:i32)->TradedoublerProductsRequest<'a>{
        self.param("pageSize",page_size.to_string())
    }

    pub fn min_update_date (self,date:DateTime<Utc>)->TradedoublerProductsRequest<'a>{
        let stamp=date.timestamp()*1000+date.timestamp_subsec_millis() as i64;
        self.param("minUpdateDate",stamp.to_string())
    }

}

impl TradedoublerVoucherClient{
    pub fn new(token:String,handle:&Handle)->TradedoublerVoucherClient{    
        let client = hyper::Client::configure().connector(HttpsConnector::new(4, handle).unwrap()).build(handle);
         TradedoublerVoucherClient{
            client:client,
            token:token
        }
    }
    pub fn get_vouchers(&self)->TradedoublerVouchersRequest{
        TradedoublerVouchersRequest{
            client:self,
            url:ALL_VOUCHERS_URL,
            token:self.token.clone()
        }
    }
    fn get_vouchers_using(&self,url:String)->Box<Future<Item=Vec<Voucher>, Error=TradedoublerClientError>>{
        info!("Fetching vouchers using URL {:?}",url);
       let out=fetch_using(url,&self.client).and_then(|str|{
             let out:Result<Vec<Voucher>,TradedoublerClientError>=serde_json::from_str(&str).map_err(|e|{
                TradedoublerClientError::DeserializationError(format!("{} {:?}",e,str))
            });
             out
        });
        Box::new(out)
    }
}


#[derive(Debug, Clone)]
pub struct TradedoublerVouchersRequest<'a>{
    client:&'a TradedoublerVoucherClient,
    url:&'a str,
    token:String
}

impl <'a> TradedoublerVouchersRequest<'a>{
    pub fn run(self)->Box<Future<Item=Vec<Voucher>, Error=TradedoublerClientError>>{
        self.client.get_vouchers_using(format!("{}?token={}",self.url,self.token))
    }
}
