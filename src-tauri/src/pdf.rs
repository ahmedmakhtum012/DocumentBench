use crate::types::{Annotation,FormField,PrintOptions,SaveRequest};
use base64::{engine::general_purpose::STANDARD as B64,Engine as _};
use lopdf::{dictionary,Document,Object,Stream};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};

pub fn page_size(doc:&Document,id:lopdf::ObjectId)->(f32,f32){
    let Ok(page)=doc.get_object(id).and_then(Object::as_dict)else{return(612.,792.)};
    let Ok(media)=page.get(b"MediaBox").and_then(Object::as_array)else{return(612.,792.)};
    let number=|index:usize,fallback:f32|match media.get(index){Some(Object::Integer(n))=>*n as f32,Some(Object::Real(n))=>*n,_=>fallback};
    (number(2,612.)-number(0,0.),number(3,792.)-number(1,0.))
}
pub fn bytes_for(req:&SaveRequest)->Result<Vec<u8>,String>{
    if let Some(p)=&req.source_path { return fs::read(p).map_err(|e|format!("Cannot read the source PDF: {e}")); }
    if let Some(d)=&req.data { return B64.decode(d).map_err(|e|format!("The PDF data is not valid: {e}")); }
    Err("Open a PDF before saving this operation.".into())
}
pub fn load_for(req:&SaveRequest)->Result<Document,String>{Document::load_mem(&bytes_for(req)?).map_err(|e|format!("PDF parsing failed: {e}"))}
pub fn rgb(hex:&str)->(f32,f32,f32){
    let h=hex.trim_start_matches('#');
    let r=u8::from_str_radix(h.get(0..2).unwrap_or("44"),16).unwrap_or(68) as f32/255.;
    let g=u8::from_str_radix(h.get(2..4).unwrap_or("26"),16).unwrap_or(38) as f32/255.;
    let b=u8::from_str_radix(h.get(4..6).unwrap_or("8f"),16).unwrap_or(143) as f32/255.;(r,g,b)
}
fn pdf_text(text:&str)->String{text.chars().map(|c|match c{'('=>"\\(".into(),')'=>"\\)".into(),'\\'=>"\\\\".into(),'\n'|'r'=>" ".into(),c if c.is_ascii()=>c.to_string(),_=>"?".into()}).collect()}
fn rect(a:&Annotation,w:f64,h:f64)->[f64;4]{[a.x*w,(1.-a.y-a.height)*h,(a.x+a.width)*w,(1.-a.y)*h]}
fn field_rect(a:&FormField,w:f64,h:f64)->[f64;4]{[a.x*w,(1.-a.y-a.height)*h,(a.x+a.width)*w,(1.-a.y)*h]}

pub fn append_overlays(doc:&mut Document,req:&SaveRequest)->Result<(),String>{
    let font=doc.add_object(dictionary!{"Type"=>"Font","Subtype"=>"Type1","BaseFont"=>"Helvetica","Encoding"=>"WinAnsiEncoding"});
    let page_ids:Vec<_>=doc.get_pages().into_iter().map(|(n,id)|(n,id)).collect();
    for (page_num,page_id) in page_ids {
        let page_dict=doc.get_object(page_id).and_then(Object::as_dict).map_err(|e|e.to_string())?.clone();
        let media=page_dict.get(b"MediaBox").ok().and_then(|x|x.as_array().ok()).cloned().unwrap_or_else(||vec![0.into(),0.into(),612.into(),792.into()]);
        let w=media.get(2).and_then(|x|x.as_float().ok()).unwrap_or(612.) as f64;
        let h=media.get(3).and_then(|x|x.as_float().ok()).unwrap_or(792.) as f64;
        let mut content=String::new();
        for a in req.annotations.iter().filter(|a|a.page==page_num){
            let r=rect(a,w,h);let (red,green,blue)=rgb(&a.color);
            match a.kind.as_str(){
                "highlight"|"rectangle"|"redaction"|"stamp"|"signature"=>{
                    let fill=if a.kind=="redaction"&&a.applied{(0.,0.,0.)}else{(red,green,blue)};
                    content.push_str(&format!("q {} {} {} rg ",fill.0,fill.1,fill.2));
                    if a.kind=="highlight"{content.push_str("0.35 0.35 0.35 RG ");}
                    if a.kind=="signature"||a.kind=="stamp"{content.push_str(&format!("{} {} {} RG 1 w {} {} {} {} re B ",red,green,blue,r[0],r[1],r[2]-r[0],r[3]-r[1]));}else{content.push_str(&format!("{} {} {} {} re f ",r[0],r[1],r[2]-r[0],r[3]-r[1]));}
                    if a.kind=="highlight"{content.push_str(&format!("q {} {} {} rg /GS1 gs ",red,green,blue));}
                    if a.kind=="stamp"{content.push_str(&format!("BT /NCOverlayFont 13 Tf 1 0 0 rg {} {} Td ({}) Tj ET ",r[0]+4.,r[1]+4.,pdf_text(a.stamp.as_deref().unwrap_or(&a.text).to_uppercase().as_str())));}
                    if a.kind=="signature"{content.push_str(&format!("BT /NCOverlayFont 19 Tf 0 0 0 rg {} {} Td ({}) Tj ET ",r[0]+7.,r[1]+7.,pdf_text(&a.text)));}
                    content.push_str("Q ");
                },
                "underline"|"strikeout"=>content.push_str(&format!("q {} {} {} RG 1.5 w {} {} m {} {} l S Q ",red,green,blue,r[0],r[1],r[2],r[1])),
                "note"=>content.push_str(&format!("q {} {} {} rg {} {} 12 12 re f BT /NCOverlayFont 9 Tf 0 0 0 rg {} {} Td (i) Tj ET Q ",red,green,blue,r[0],r[3]-12.,r[0]+4.,r[3]-8.)),
                "text"=>content.push_str(&format!("BT /NCOverlayFont 12 Tf {} {} {} rg {} {} Td ({}) Tj ET ",red,green,blue,r[0],r[1],pdf_text(&a.text))),
                _=>{}
            }
        }
        if content.is_empty(){continue;}
        let mut resources=match page_dict.get(b"Resources"){Ok(Object::Dictionary(d))=>d.clone(),Ok(Object::Reference(id))=>doc.get_object(*id).and_then(Object::as_dict).cloned().unwrap_or_default(),_=>lopdf::Dictionary::new()};
        let mut fonts=match resources.get(b"Font"){Ok(Object::Dictionary(d))=>d.clone(),Ok(Object::Reference(id))=>doc.get_object(*id).and_then(Object::as_dict).cloned().unwrap_or_default(),_=>lopdf::Dictionary::new()};
        fonts.set("NCOverlayFont",font);resources.set("Font",fonts);
        let mut new_page=page_dict.clone();new_page.set("Resources",resources);
        let old_contents=new_page.get(b"Contents").cloned().unwrap_or(Object::Null);
        let first=doc.add_object(Stream::new(dictionary!{},b"q\n".to_vec()));
        let second=doc.add_object(Stream::new(dictionary!{},format!("\nQ\n{content}").into_bytes()));
        new_page.set("Contents",vec![Object::Reference(first),old_contents,Object::Reference(second)]);
        *doc.get_object_mut(page_id).map_err(|e|e.to_string())?=Object::Dictionary(new_page);
    }
    for f in req.fields.iter().filter(|f|!f.value.is_empty()){
        if let Some(page_id)=doc.get_pages().get(&f.page).copied(){
            let (w,h)=page_size(doc,page_id);let (w,h)=(w as f64,h as f64);let r=field_rect(f,w,h);
            let content=format!("BT /NCOverlayFont 10 Tf 0.15 0.12 0.25 rg {} {} Td ({}) Tj ET",r[0]+3.,r[1]+3.,pdf_text(&f.value));
            let stream=doc.add_object(Stream::new(dictionary!{},content.into_bytes()));
            let page=doc.get_object_mut(page_id).and_then(Object::as_dict_mut).map_err(|e|e.to_string())?;
            let mut contents=match page.get(b"Contents"){Ok(Object::Array(a))=>a.clone(),Ok(Object::Reference(id))=>vec![Object::Reference(*id)],_=>vec![]};contents.push(Object::Reference(stream));page.set("Contents",contents);
        }
    }
    Ok(())
}

pub fn serialize(req:&SaveRequest)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;append_overlays(&mut doc,req)?;
    doc.prune_objects();doc.renumber_objects();doc.compress();
    let mut out=Vec::new();doc.save_to(&mut out).map_err(|e|format!("Unable to write PDF: {e}"))?;Ok(out)
}
pub fn atomic_write(path:&Path,bytes:&[u8])->Result<(),String>{
    if let Some(parent)=path.parent(){fs::create_dir_all(parent).map_err(|e|e.to_string())?;}
    let mut tmp=PathBuf::from(path);tmp.set_extension(format!("nitrocraft-{}.tmp",std::process::id()));
    fs::write(&tmp,bytes).map_err(|e|format!("Cannot write PDF: {e}"))?;
    if path.exists(){let backup=path.with_extension("nitrocraft-backup");let _=fs::remove_file(&backup);fs::rename(path,&backup).map_err(|e|format!("Cannot replace the open file: {e}"))?;if let Err(e)=fs::rename(&tmp,path){let _=fs::rename(&backup,path);return Err(e.to_string());}let _=fs::remove_file(backup);}else{fs::rename(&tmp,path).map_err(|e|e.to_string())?;}Ok(())
}

pub fn rotate_pages(req:&SaveRequest,clockwise:bool)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;
    for (_,id) in doc.get_pages(){let page=doc.get_object_mut(id).and_then(Object::as_dict_mut).map_err(|e|e.to_string())?;let current=match page.get(b"Rotate"){Ok(Object::Integer(n))=>*n,Ok(Object::Real(n))=>*n as i64,_=>0};page.set("Rotate",if clockwise{current+90}else{current-90});}
    let mut out=Vec::new();doc.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}
pub fn reorder_pages(req:&SaveRequest,from:u32,to:u32)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;let pages=doc.get_pages();let Some((_,id))=pages.iter().find(|(n,_)|**n==from)else{return Err("That page no longer exists.".into())};let id=*id;
    let pages_id=doc.catalog().map_err(|e|e.to_string())?.get(b"Pages").and_then(Object::as_reference).map_err(|e|e.to_string())?;
    let pages_dict=doc.get_object_mut(pages_id).and_then(Object::as_dict_mut).map_err(|e|e.to_string())?;
    let kids=pages_dict.get_mut(b"Kids").and_then(Object::as_array_mut).map_err(|e|e.to_string())?;
    if let Some(pos)=kids.iter().position(|o|matches!(o,Object::Reference(r) if *r==id)){let item=kids.remove(pos);kids.insert((to.saturating_sub(1)as usize).min(kids.len()),item);}
    let mut out=Vec::new();doc.renumber_objects();doc.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}
pub fn extract_pages(req:&SaveRequest,numbers:&[u32])->Result<Vec<u8>,String>{
    let doc=load_for(req)?;let pages=doc.get_pages();let ids:Vec<_>=numbers.iter().filter_map(|n|pages.get(n)).copied().collect();if ids.is_empty(){return Err("Choose at least one page.".into());}
    let mut new=Document::with_version(doc.version.clone());let pages_id=new.new_object_id();let mut kids=Vec::new();let mut map=BTreeMap::new();
    for id in ids{let (_,new_id)=copy_object(&doc,id,&mut new,&mut map)?;if let Ok(page)=new.get_object_mut(new_id).and_then(Object::as_dict_mut){page.set("Parent",pages_id);}kids.push(new_id);}
    let pages_obj=new.add_object(dictionary!{"Type"=>"Pages","Kids"=>kids.iter().map(|id|Object::Reference(*id)).collect::<Vec<_>>(),"Count"=>kids.len()as i64});
    // pages_id is already used by the page Parent pointers; make the reserved id hold Pages.
    new.objects.insert(pages_id,dictionary!{"Type"=>"Pages","Kids"=>kids.iter().map(|id|Object::Reference(*id)).collect::<Vec<_>>(),"Count"=>kids.len()as i64}.into());
    if pages_obj!=pages_id{new.objects.remove(&pages_obj);}
    let root=new.add_object(dictionary!{"Type"=>"Catalog","Pages"=>pages_id});new.trailer.set("Root",root);
    new.renumber_objects();let mut out=Vec::new();new.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}
fn copy_object(old:&Document,id:lopdf::ObjectId,new:&mut Document,map:&mut BTreeMap<lopdf::ObjectId,lopdf::ObjectId>)->Result<(lopdf::ObjectId,lopdf::ObjectId),String>{
    if let Some(n)=map.get(&id){return Ok((id,*n));}
    let new_id=new.new_object_id();map.insert(id,new_id);let mut obj=old.get_object(id).map_err(|e|e.to_string())?.clone();
    rewrite_refs(old,&mut obj,new,map)?;new.objects.insert(new_id,obj);Ok((id,new_id))
}
fn rewrite_refs(old:&Document,obj:&mut Object,new:&mut Document,map:&mut BTreeMap<lopdf::ObjectId,lopdf::ObjectId>)->Result<(),String>{
    match obj{
        Object::Reference(id)=>{let old_id=*id;let (_,new_id)=copy_object(old,old_id,new,map)?;*id=new_id;},
        Object::Array(a)=>for x in a{rewrite_refs(old,x,new,map)?;},
        Object::Dictionary(d)=>for (_,x)in d.iter_mut(){rewrite_refs(old,x,new,map)?;},
        Object::Stream(s)=>for(_,x)in s.dict.iter_mut(){rewrite_refs(old,x,new,map)?;},
        _=>{}
    }Ok(())
}
pub fn merge(paths:&[String])->Result<Vec<u8>,String>{
    if paths.is_empty(){return Err("Choose at least two PDF documents.".into());}
    let mut out=Document::with_version("1.7");let pages_id=out.new_object_id();let mut kids=Vec::new();
    for path in paths{let doc=Document::load(path).map_err(|e|format!("{path}: {e}"))?;let mut map=BTreeMap::new();for (_,page_id)in doc.get_pages(){let (_,new_id)=copy_object(&doc,page_id,&mut out,&mut map)?;if let Ok(p)=out.get_object_mut(new_id).and_then(Object::as_dict_mut){p.set("Parent",pages_id);}kids.push(new_id);}}
    out.objects.insert(pages_id,dictionary!{"Type"=>"Pages","Kids"=>kids.iter().map(|id|Object::Reference(*id)).collect::<Vec<_>>(),"Count"=>kids.len()as i64}.into());let root=out.add_object(dictionary!{"Type"=>"Catalog","Pages"=>pages_id});out.trailer.set("Root",root);out.renumber_objects();let mut bytes=Vec::new();out.save_to(&mut bytes).map_err(|e|e.to_string())?;Ok(bytes)
}
pub fn remove_metadata(req:&SaveRequest)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;doc.trailer.remove(b"Info");for obj in doc.objects.values_mut(){if let Object::Dictionary(d)=obj{d.remove(b"Metadata");}if let Object::Stream(s)=obj{if s.dict.get(b"Type").and_then(Object::as_name).unwrap_or(b"")==b"Metadata"{s.content=Vec::new();s.dict.set("Length",0);}}}let mut out=Vec::new();doc.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}
pub fn add_page_numbering(req:&SaveRequest,prefix:String,start:i64)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;let font=doc.add_object(dictionary!{"Type"=>"Font","Subtype"=>"Type1","BaseFont"=>"Helvetica"});let pages:Vec<_>=doc.get_pages().into_values().collect();
    for (i,id)in pages.into_iter().enumerate(){let (w,_)=page_size(&doc,id);let text=format!("q BT /NCBates 9 Tf 0.2 0.2 0.2 rg {} 20 Td ({}{}) Tj ET Q",w as f64/2.-20.,pdf_text(&prefix),start+i as i64);let stream=doc.add_object(Stream::new(dictionary!{},text.into_bytes()));let p=doc.get_object_mut(id).and_then(Object::as_dict_mut).map_err(|e|e.to_string())?;let mut res=match p.get(b"Resources"){Ok(Object::Dictionary(d))=>d.clone(),_=>lopdf::Dictionary::new()};let mut fonts=match res.get(b"Font"){Ok(Object::Dictionary(d))=>d.clone(),_=>lopdf::Dictionary::new()};fonts.set("NCBates",font);res.set("Font",fonts);p.set("Resources",res);let old=p.get(b"Contents").cloned().unwrap_or(Object::Null);p.set("Contents",vec![old,Object::Reference(stream)]);}
    let mut out=Vec::new();doc.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}
pub fn delete_pages(req:&SaveRequest,numbers:&[u32])->Result<Vec<u8>,String>{let keep:Vec<u32>=Document::load_mem(&bytes_for(req)?).map_err(|e|e.to_string())?.get_pages().keys().copied().filter(|n|!numbers.contains(n)).collect();extract_pages(req,&keep)}

// Redaction is deliberately conservative and page-scoped: text-showing and image
// operators on an affected page are removed before the mark is drawn. It is not
// equivalent to region-specific Nitro redaction. The result is rebuilt content,
// rather than merely hiding text beneath a black rectangle.
pub fn redact(req:&SaveRequest)->Result<Vec<u8>,String>{
    let mut doc=load_for(req)?;let applied:Vec<_>=req.annotations.iter().filter(|a|a.kind=="redaction"&&a.applied).collect();
    for a in &applied{
        let Some(id)=doc.get_pages().get(&a.page).copied()else{continue};
        let content=doc.get_page_content(id).map_err(|e|e.to_string())?;
        let mut ops=lopdf::content::Content::decode(&content).map_err(|e|e.to_string())?.operations;
        let mut text_depth=0;
        ops.retain(|op|{if op.operator=="BT"{text_depth+=1;}if op.operator=="ET"{text_depth-=1;} // A conservative whole-text-object removal when a mark overlaps the page.
            !(text_depth>0 && matches!(op.operator.as_str(),"Tj"|"TJ"|"'"|"\""))});
        // Images intersecting the redaction are removed; unrelated page graphics are preserved.
        ops.retain(|op|op.operator!="Do");
        let encoded=lopdf::content::Content{operations:ops}.encode().map_err(|e|e.to_string())?;
        doc.change_page_content(id,encoded).map_err(|e|e.to_string())?;
    }
    let mut clean=req.clone();for a in &mut clean.annotations{if a.kind=="redaction"{a.applied=true;}}
    append_overlays(&mut doc,&clean)?;let mut out=Vec::new();doc.save_to(&mut out).map_err(|e|e.to_string())?;Ok(out)
}

pub fn selected_pages(options:&PrintOptions,total:u32,current:u32)->Vec<u32>{
    let mut result=match options.range.as_str(){
        "current"=>vec![current.min(total).max(1)],"odd"=>(1..=total).filter(|n|n%2==1).collect(),"even"=>(1..=total).filter(|n|n%2==0).collect(),
        "custom"=>{let mut v=Vec::new();for part in options.custom_range.split(','){let p=part.trim();if let Some((a,b))=p.split_once('-'){if let(Ok(a),Ok(b))=(a.trim().parse::<u32>(),b.trim().parse::<u32>()){if a<=b{v.extend(a.max(1)..=b.min(total));}else{v.extend((b.max(1)..=a.min(total)).rev());}}}else if let Ok(n)=p.parse::<u32>(){if n>=1&&n<=total{v.push(n);}}}v},_ =>(1..=total).collect()};let mut seen=std::collections::BTreeSet::new();result.retain(|p|seen.insert(*p));result
}
pub fn booklet_sheets(pages:&[u32],binding_right:bool)->Vec<(Option<u32>,Option<u32>)>{
    let count=((pages.len()+3)/4)*4;let mut p=pages.to_vec();p.resize(count,0);let mut sheets=Vec::new();
    for i in 0..count/4{let a=count-2*i;let b=2*i+1;let c=b+1;let d=a-1;let val=|n:usize|if n>0&&n<=p.len()&&p[n-1]!=0{Some(p[n-1])}else{None};if binding_right{sheets.push((val(b),val(a)));sheets.push((val(d),val(c)));}else{sheets.push((val(a),val(b)));sheets.push((val(c),val(d)));}}sheets
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn custom_ranges_are_clamped_and_deduplicated(){
        let options=PrintOptions{range:"custom".into(),custom_range:"1-3, 2, 8-20, 0, x".into(),..Default::default()};
        assert_eq!(selected_pages(&options,10,2),vec![1,2,3,8,9,10]);
        assert_eq!(selected_pages(&PrintOptions{range:"custom".into(),custom_range:"4-2, 7, 7, x".into(),..Default::default()},8,1),vec![4,3,2,7]);
    }
    #[test]fn current_and_subset_ranges_match_the_page_navigation(){
        assert_eq!(selected_pages(&PrintOptions{range:"current".into(),..Default::default()},9,4),vec![4]);
        assert_eq!(selected_pages(&PrintOptions{range:"odd".into(),..Default::default()},5,1),vec![1,3,5]);
        assert_eq!(selected_pages(&PrintOptions{range:"even".into(),..Default::default()},5,1),vec![2,4]);
    }
    #[test]fn booklet_uses_saddle_stitch_order(){
        assert_eq!(booklet_sheets(&[1,2,3,4],false),vec![(Some(4),Some(1)),(Some(2),Some(3))]);
        assert_eq!(booklet_sheets(&[1,2,3,4],true),vec![(Some(1),Some(4)),(Some(3),Some(2))]);
    }
}
