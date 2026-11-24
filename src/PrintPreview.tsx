import React from 'react';
import {PrintOptions,DocState} from './api';
import {PdfImage,SamplePage} from './PdfCanvas';
import Icon from './Icon';

export function bookletSheets(source:number[],right:boolean){
 const count=Math.ceil(source.length/4)*4;const pages=[...source];while(pages.length<count)pages.push(0);
 const sheets:[number,number][]=[];
 for(let i=0;i<count/4;i++){
  const a=count-2*i,b=2*i+1,c=b+1,d=a-1;
  const value=(n:number)=>n>0&&n<=pages.length?pages[n-1]:0;
  sheets.push(right?[value(b),value(a)]:[value(a),value(b)]);
  sheets.push(right?[value(d),value(c)]:[value(c),value(d)]);
 }
 return sheets;
}

export function rangePages(o:PrintOptions,total:number,current:number){
 let arr:number[]=[];
 if(o.range==='all')arr=Array.from({length:total},(_,i)=>i+1);
 else if(o.range==='current')arr=[Math.min(total,Math.max(1,current))];
 else if(o.range==='odd')arr=Array.from({length:total},(_,i)=>i+1).filter(x=>x%2===1);
 else if(o.range==='even')arr=Array.from({length:total},(_,i)=>i+1).filter(x=>x%2===0);
 else for(const part of o.customRange.split(',')){
  const [a,b]=part.trim().split('-').map(Number);
  if(Number.isFinite(a)&&Number.isFinite(b)&&b!==0){const step=a<=b?1:-1;for(let n=Math.max(1,Math.min(total,a));step>0?n<=Math.min(total,b):n>=Math.max(1,b);n+=step)arr.push(n);}
  else if(Number.isFinite(a)&&a>=1&&a<=total)arr.push(a);
 }
 return [...new Set(arr)];
}

export default function PrintPreview({doc,options,sheet=0}:{doc:DocState;options:PrintOptions;sheet?:number}){
 const selected=rangePages(options,doc.pageCount,doc.page);const columns=options.booklet?2:options.columns;const rows=options.booklet?1:options.rows;
 const perSheet=columns*rows;const groups=options.booklet?bookletSheets(selected,options.bookletBinding==='right'):Array.from({length:Math.ceil(selected.length/perSheet)},(_,i)=>selected.slice(i*perSheet,(i+1)*perSheet));
 const pages=groups[Math.min(sheet,Math.max(0,groups.length-1))]||[];
 const size=doc.pageSizes[doc.page-1]||{width:612,height:792};const landscape=options.orientation==='landscape'||(options.orientation==='auto'&&size.width>size.height&&!options.booklet);const ratio=landscape?11/8.5:8.5/11;
 return <div className="print-preview-wrap"><div className={`print-paper ${options.grayscale?'print-grayscale':''}`} style={{aspectRatio:ratio}}>
  <div className="print-grid" style={{gridTemplateColumns:`repeat(${columns},1fr)`,gridTemplateRows:`repeat(${rows},1fr)`,padding:options.scaling==='actual'?'5%':'3%'}}>
   {pages.map((p,index)=><div className="print-cell" key={`${p}-${index}`}><div className={`print-cell-content ${options.scaling==='actual'?'print-actual':options.scaling==='custom'?'print-custom':''}`} style={options.scaling==='custom'?{transform:`scale(${Math.max(10,Math.min(400,options.scale))/100})`}:undefined}>{p!==0&&(doc.id==='welcome'?<SamplePage page={p}/>:<PdfImage data={doc.data} path={doc.path} page={p} width={380} grayscale={options.grayscale}/>)}</div></div>)}
  </div>
  {options.booklet&&<div className="booklet-fold"/>}
 </div><div className="preview-caption"><Icon name="document" size={13}/> Sheet {groups.length?Math.min(sheet+1,groups.length):0} of {groups.length} <span>•</span> {landscape?'Letter, Landscape':'Letter, Portrait'} <span>•</span> {options.booklet?'Booklet':`${perSheet} page${perSheet>1?'s':''} per sheet`}</div></div>
}
