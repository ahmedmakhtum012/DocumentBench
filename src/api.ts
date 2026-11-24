import { invoke } from '@tauri-apps/api/core';
import { open as nativeOpen, save as nativeSave } from '@tauri-apps/plugin-dialog';
import { makeSamplePdf } from './samplePdf';

export type Annotation = { id: string; page: number; kind: 'note'|'highlight'|'underline'|'strikeout'|'rectangle'|'text'|'signature'|'redaction'|'stamp'; x: number; y: number; width: number; height: number; text: string; color: string; applied?: boolean; stamp?: string; };
export type FormField = { id: string; page: number; name: string; kind: 'text'|'checkbox'|'signature'; x: number; y: number; width: number; height: number; value: string; required: boolean; };
export type DocState = { id: string; name: string; path: string | null; data: string | null; pageCount: number; page: number; zoom: number; fit: 'custom'|'page'|'width'; rotation: number; annotations: Annotation[]; fields: FormField[]; dirty: boolean; text: string; isXfa: boolean; encrypted: boolean; pageSizes: {width:number;height:number}[]; bookmarks: {label:string;page:number}[]; fileSize: number; };
export type PrintOptions = { printer: string; range: 'all'|'current'|'custom'|'odd'|'even'; customRange: string; orientation: 'auto'|'portrait'|'landscape'; scaling: 'fit'|'actual'|'custom'|'shrink'; scale: number; rows: number; columns: number; order: 'horizontal'|'vertical'; booklet: boolean; bookletBinding: 'left'|'right'; grayscale: boolean; content: 'all'|'document'|'markups'|'forms'; copies: number; collate: boolean; currentPage: number; };
export type HistoryEntry = { doc: DocState; label: string; time: number; };
export type ExtractionRow = { cells: string[] };
export type ToolResult = { message: string; data?: string; rows?: ExtractionRow[]; fields?: FormField[]; };

const isNative = '__TAURI_INTERNALS__' in window;
const bytesToBase64 = (bytes: Uint8Array) => { let s = ''; for (let i=0;i<bytes.length;i+=0x8000) s += String.fromCharCode(...bytes.subarray(i,i+0x8000)); return btoa(s); };
const base64ToBytes = (s: string) => Uint8Array.from(atob(s), c => c.charCodeAt(0));
export const uid = () => Math.random().toString(36).slice(2,10) + Date.now().toString(36);

const sampleText = `PROJECT BRIEF\nWorkspace transformation\n\nA practical plan for better document workflows.\n\nPrepared by the Operations Team\n22 November 2026\n\n01  EXECUTIVE SUMMARY\n\nThis document outlines a focused approach to improving how teams create, review, and share important documents. The goal is a simpler workflow that gives people more time for meaningful work.\n\nThree priorities guide the proposal: consistent document standards, clear collaboration, and secure information handling. Each priority has a measurable outcome and a named owner.\n\n02  KEY OBJECTIVES\n\n• Reduce repetitive document preparation.\n• Make reviews clear, accountable, and easy to track.\n• Protect personal and confidential information.\n• Create accessible, well-organized final documents.\n\n03  IMPLEMENTATION PLAN\n\nDiscovery and planning        Week 1–2\nPilot and feedback             Week 3–4\nOrganization-wide rollout     Week 5–6\n\nThe pilot will include a small cross-functional group. Their feedback will shape training materials and the final rollout checklist.\n\n04  SUCCESS MEASURES\n\nWe will measure time saved, review completion, and user confidence. A short monthly review will identify opportunities to improve templates and guidance.\n\nContact: operations@example.com\nTelephone: +1 (206) 555-0148`;

export function makeSample(): DocState {
  return { id:'welcome', name:'Project brief.pdf', path:null, data:null, pageCount:4, page:1, zoom:100, fit:'custom', rotation:0, dirty:false, text:sampleText, isXfa:false, encrypted:false, fileSize:284672,
    pageSizes:Array.from({length:4},()=>({width:612,height:792})),
    bookmarks:[{label:'Executive summary',page:1},{label:'Key objectives',page:2},{label:'Implementation plan',page:3},{label:'Success measures',page:4}],
    annotations:[{id:'welcome-note',page:1,kind:'note',x:.76,y:.67,width:.055,height:.055,text:'Review the rollout dates with the project team.',color:'#f4bf4f'}],
    fields:[{id:'welcome-field',page:4,name:'Approved by',kind:'text',x:.13,y:.67,width:.40,height:.035,value:'',required:false}],
  };
}

export const samplePageText = (page:number) => [
  sampleText,
  `02  KEY OBJECTIVES\n\nA consistent, confident way to work.\n\nSTANDARDIZE\nUse shared templates and clear naming to make every document easier to understand.\n\nCOLLABORATE\nBring feedback together in one place. Give every comment an owner and a clear next step.\n\nPROTECT\nIdentify sensitive information early and apply the right level of protection before sharing.\n\nA better document workflow\n\nBefore                         After\nMultiple versions              One trusted source\nScattered feedback             Organized comments\nManual checks                  Guided reviews\nUnclear ownership              Named contributors\n\nDesign principles\n\n1. Keep the primary task within easy reach.\n2. Show the effect of a choice before committing it.\n3. Preserve a clear path back through undo and recovery.\n4. Make document security part of the everyday workflow.`,
  `03  IMPLEMENTATION PLAN\n\nSix weeks from discovery to rollout.\n\nPHASE 01 — DISCOVERY\nInterview stakeholders and map the current document journey.\nOwner: Maya Chen                 Timing: Week 1\n\nPHASE 02 — PILOT\nTest templates, review workflows, and training with a small group.\nOwner: Jordan Lee                Timing: Week 2–3\n\nPHASE 03 — REFINEMENT\nResolve feedback and publish the final operating guide.\nOwner: Sam Rivera                Timing: Week 4\n\nPHASE 04 — ROLLOUT\nIntroduce the new process with supported team sessions.\nOwner: Operations Team           Timing: Week 5–6\n\nReadiness checklist\n\n☑ Sponsors confirmed\n☑ Pilot group selected\n☐ Training sessions scheduled\n☐ Success baseline recorded\n\nNotes\nAdd your team's milestones and any dependencies here.`,
  `04  SUCCESS MEASURES\n\nA simple scorecard for continuous improvement.\n\nMEASURE                    TARGET             FREQUENCY\nDocument preparation        −30% time         Monthly\nReviews completed on time    90%              Weekly\nUser confidence              4.5 / 5           Quarterly\nProtected external files     100%             Monthly\n\nREVIEW AND APPROVAL\n\nThis proposal is ready for stakeholder review. Comments can be added directly to this document; approvals are tracked below.\n\nApproved by\n\n________________________________\nName\n\n________________________________\nDate\n\nNEXT STEPS\n\nConfirm the pilot group, agree on the baseline, and schedule the kickoff session.\n\noperations@example.com\n+1 (206) 555-0148\n\nDocumentBench sample document • 4 pages`
][Math.min(3,Math.max(0,page-1))];

export async function openPdf(): Promise<DocState|null> {
  if (isNative) {
    const path = await nativeOpen({multiple:false,filters:[{name:'PDF documents',extensions:['pdf']}]});
    if (!path) return null;
    const d = await invoke<any>('open_document',{path});
    return {...d,data:null,page:1,zoom:100,fit:'custom',rotation:0,annotations:[],fields:[],dirty:false,text:d.text || ''};
  }
  const input = document.createElement('input'); input.type='file'; input.accept='.pdf,application/pdf';
  return new Promise(resolve=>{ input.onchange=async()=>{ const f=input.files?.[0]; if(!f){resolve(null);return;} const data=bytesToBase64(new Uint8Array(await f.arrayBuffer())); resolve({id:uid(),name:f.name,path:null,data,pageCount:1,page:1,zoom:100,fit:'custom',rotation:0,annotations:[],fields:[],dirty:false,text:'',isXfa:false,encrypted:false,fileSize:f.size,pageSizes:[],bookmarks:[]}); }; input.oncancel=()=>resolve(null); input.click(); });
}

export async function openPath(path:string): Promise<DocState|null> {
  if (!isNative) return null;
  const d=await invoke<any>('open_document',{path});
  return {...d,data:null,page:1,zoom:100,fit:'custom',rotation:0,annotations:[],fields:[],dirty:false,text:d.text || ''};
}

export async function saveDocument(doc:DocState, asNew=false): Promise<DocState> {
  if(isNative){
    let working=doc;
    if(!working.path&&!working.data){working={...working,data:bytesToBase64(makeSamplePdf(working))};}
    let path=working.path;
    if(!path||asNew){ path=await nativeSave({defaultPath:working.name,filters:[{name:'PDF document',extensions:['pdf']}]}); if(!path) return doc; }
    const result=await invoke<any>('save_document',{path,request:{id:working.id,sourcePath:working.data?null:working.path,data:working.data,annotations:working.annotations,fields:working.fields,rotation:working.rotation}});
    return {...working,path:result.path,name:result.name,fileSize:result.size,data:result.data,annotations:[],fields:[],dirty:false};
  }
  let bytes:Uint8Array;
  if(doc.data){ bytes=base64ToBytes(doc.data); }
  else { bytes=makeSamplePdf(doc); }
  const blob=new Blob([bytes.buffer as ArrayBuffer],{type:'application/pdf'}); const a=document.createElement('a');a.href=URL.createObjectURL(blob);a.download=doc.name;a.click();setTimeout(()=>URL.revokeObjectURL(a.href),5000);
  return {...doc,dirty:false};
}

export async function chooseSave(defaultPath:string,kind='Excel workbook'):Promise<string|null>{
  if(isNative) return nativeSave({defaultPath,filters:[{name:kind,extensions:['xlsx','csv','txt','pdf','docx','pptx','png']} ]});
  return defaultPath;
}

export async function exportFile(path:string,content:string,format='csv'):Promise<string>{
  if(isNative) return invoke('export_content',{path,content,format});
  const blob=new Blob(format==='base64'?[base64ToBytes(content)]:[content],{type:format==='csv'?'text/csv':format==='base64'?'application/octet-stream':'text/plain'}); const a=document.createElement('a'); a.href=URL.createObjectURL(blob);a.download=path;a.click();setTimeout(()=>URL.revokeObjectURL(a.href),5000);return path;
}

export async function printNative(doc:DocState,options:PrintOptions):Promise<string>{
  if(isNative) return invoke('print_document',{request:saveRequest(doc),options:{...options,currentPage:doc.page}});
  const w=window.open('','_blank'); if(!w) throw new Error('Allow pop-ups to use the browser print preview.');
  w.document.write(`<title>${doc.name}</title><style>body{font:16px Georgia;margin:60px;white-space:pre-wrap}@page{margin:20mm}</style>${(doc.text||samplePageText(doc.page)).replace(/</g,'&lt;')}`);w.document.close();w.print();return 'Sent to the system print dialog';
}

export async function getPrinters():Promise<string[]>{ if(isNative) return invoke('list_printers'); return ['System default printer']; }
export async function renderPage(sourcePath:string|null,data:string|null,page:number,width:number,grayscale=false):Promise<string>{return invoke('render_page',{sourcePath,data,page,width,grayscale});}
export async function pageOperation(doc:DocState,operation:string,extra:Record<string,unknown>={}):Promise<string>{return invoke('page_operation',{request:saveRequest(doc),operation,...extra});}
export async function combineDocuments(paths:string[]):Promise<string>{return invoke('combine_documents',{paths});}
export async function choosePdfs(multiple=true):Promise<string[]>{if(!isNative)return [];const result=await nativeOpen({multiple,filters:[{name:'PDF documents',extensions:['pdf']}]});return !result?[]:Array.isArray(result)?result:[result];}
export function saveRequest(doc:DocState){const data=doc.path||doc.data?doc.data:bytesToBase64(makeSamplePdf(doc));const sourcePath=data?null:doc.path;return{id:doc.id,sourcePath,data,annotations:doc.annotations,fields:doc.fields,rotation:doc.rotation};}
export async function autosave(doc:DocState){if(isNative)return invoke('autosave_document',{request:saveRequest(doc)});}
export async function createBlank():Promise<DocState>{const base=makeSample();const doc={...base,id:uid(),name:'Untitled.pdf',pageCount:1,text:'',annotations:[],fields:[],bookmarks:[],fileSize:0,pageSizes:[{width:612,height:792}]};return{...doc,data:bytesToBase64(makeSamplePdf(doc))};}
export async function pdfCommand<T=any>(command:string,args:any):Promise<T>{ return invoke(command,args); }
export { isNative, bytesToBase64, base64ToBytes };
