use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct Annotation {
    pub id: String,
    pub page: u32,
    pub kind: String,
    pub x: f64, pub y: f64, pub width: f64, pub height: f64,
    pub text: String, pub color: String,
    #[serde(default)] pub applied: bool,
    pub stamp: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct FormField {
    pub id: String, pub page: u32, pub name: String, pub kind: String,
    pub x: f64, pub y: f64, pub width: f64, pub height: f64,
    pub value: String, #[serde(default)] pub required: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SaveRequest {
    pub id: String, pub source_path: Option<String>, pub data: Option<String>,
    pub annotations: Vec<Annotation>, pub fields: Vec<FormField>, pub rotation: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PrintOptions {
    pub printer: String, pub range: String, pub custom_range: String,
    pub orientation: String, pub scaling: String, pub scale: f64,
    pub rows: u32, pub columns: u32, pub order: String,
    pub booklet: bool, pub booklet_binding: String, pub grayscale: bool,
    pub content: String, pub copies: u32, pub collate: bool,
    #[serde(default)] pub current_page: u32,
}
impl Default for PrintOptions {
    fn default()->Self{Self{printer:String::new(),range:"all".into(),custom_range:String::new(),orientation:"auto".into(),scaling:"fit".into(),scale:100.,rows:1,columns:1,order:"horizontal".into(),booklet:false,booklet_binding:"left".into(),grayscale:false,content:"all".into(),copies:1,collate:true,current_page:1}}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DocumentInfo {
    pub id: String, pub name: String, pub path: Option<String>, pub page_count: u32,
    pub text: String, pub is_xfa: bool, pub encrypted: bool, pub file_size: u64,
    pub page_sizes: Vec<PageSize>, pub bookmarks: Vec<Bookmark>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSize { pub width:f64, pub height:f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark { pub label:String, pub page:u32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult { pub path:String, pub name:String, pub size:u64, pub data:String }
