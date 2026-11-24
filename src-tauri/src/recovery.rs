use crate::pdf::atomic_write;
use serde::Serialize;
use std::{fs,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};

#[derive(Debug,Serialize)]
#[serde(rename_all="camelCase")]
pub struct RecoveryInfo {pub path:String,pub saved_at:u64,pub original:String}
fn folder()->Result<PathBuf,String>{let p=dirs::data_local_dir().ok_or("No local application-data folder is available.")?.join("NitroCraft").join("Recovery");fs::create_dir_all(&p).map_err(|e|e.to_string())?;Ok(p)}
pub fn save(id:&str,original:&str,bytes:&[u8])->Result<RecoveryInfo,String>{
    let p=folder()?;let stem=PathBuf::from(original).file_stem().map(|s|s.to_string_lossy().to_string()).unwrap_or_else(||"Untitled".into());let mut n=0;
    let dest=loop{let name=if n==0{format!("{stem}(Autosaved).pdf")}else{format!("{stem}(Autosaved{n}).pdf")};let candidate=p.join(name);if !candidate.exists(){break candidate;}n+=1;};
    atomic_write(&dest,bytes)?;let saved_at=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    fs::write(p.join(format!("{id}.json")),serde_json::to_vec(&serde_json::json!({"path":dest.to_string_lossy(),"savedAt":saved_at,"original":original})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    Ok(RecoveryInfo{path:dest.to_string_lossy().into(),saved_at,original:original.into()})
}
pub fn clear(id:&str){if let Ok(p)=folder(){let _=fs::remove_file(p.join(format!("{id}.json")));}}
pub fn list()->Vec<RecoveryInfo>{let mut out=Vec::new();if let Ok(p)=folder(){if let Ok(entries)=fs::read_dir(p){for entry in entries.flatten(){if entry.path().extension().and_then(|x|x.to_str())==Some("json"){if let Ok(text)=fs::read_to_string(entry.path()){if let Ok(v)=serde_json::from_str::<RecoveryInfoOwned>(&text){out.push(RecoveryInfo{path:v.path,saved_at:v.saved_at,original:v.original});}}}}}}out}
#[derive(serde::Deserialize)]#[serde(rename_all="camelCase")]struct RecoveryInfoOwned{path:String,saved_at:u64,original:String}
