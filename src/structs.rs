use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field{
	pub name:Option<String>,
	pub value:String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price{
	pub value: String,
	pub currency: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceHistoryEvent{
	pub date: f64,
	pub price:Price

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category{
	pub id: Option<i32>,
	pub name: Option<String>,
	#[serde(rename="tdCategoryName")]
	pub td_category_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteImage{
	pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Offer{
	#[serde(rename="feedId")]
	pub feed_id: i32,
	#[serde(rename="productUrl")]
	pub product_url: String,
	#[serde(rename="priceHistory")]
	pub price_history:Vec<PriceHistoryEvent>,
	pub modified:  f64,
	#[serde(rename="sourceProductId")]
	pub source_product_id: Option<String>,
	#[serde(rename="programLogo")]
	pub program_logo:Option<String>,
	#[serde(rename="programName")]
	pub program_name:Option<String>,
	pub id:String,
	pub availability:Option<String>,
	pub condition:Option<String>
}

pub type Identifier=String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product{
	pub name: Option<String>,
	pub description: String,
	pub fields: Option<Vec<Field>>,
	pub offers: Vec<Offer>,
	pub categories: Option<Vec<Category>>,
	#[serde(rename="productImage")]
	pub product_image: RemoteImage,
	pub language: Option<String>,
	pub size: Option<String>,
	pub weight: Option<String>,
	pub brand: Option<String>,
	pub identifiers: Option<HashMap<Identifier,String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Products{
	pub products:Vec<Product>
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voucher{
	pub id: i32,
	#[serde(rename="programId")]
	pub program_id: i32,
	#[serde(rename="programName")]
	pub program_name: String,
	pub code: Option<String>,
	#[serde(rename="updateDate")]
	pub update_date: String,
	#[serde(rename="publishStartDate")]
	pub publish_start_date: String,
	#[serde(rename="publishEndDate")]
	pub publish_end_date: String,
	#[serde(rename="startDate")]
	pub start_date: String,
	#[serde(rename="endDate")]
	pub end_date: String,
	pub title: String,
	#[serde(rename="shortDescription")]
	pub short_description: String,
	pub description: String,
	#[serde(rename="voucherTypeId")]
	pub voucher_type_id: i32,
	#[serde(rename="defaultTrackUri")]
	pub default_track_uri: String,
	#[serde(rename="siteSpecific")]
	pub site_specific: bool,
	#[serde(rename="discountAmount")]
	pub discount_amount: Option<f32>,
	#[serde(rename="isPercentage")]
	pub is_percentage: bool,
	#[serde(rename="publisherInformation")]
	pub publisher_information: String,
	#[serde(rename="languageId")]
	pub language_id: String,
	pub exclusive: bool,
	#[serde(rename="currencyId")]
	pub currency_id: String,
	#[serde(rename="logoPath")]
	pub logo_path: String
}