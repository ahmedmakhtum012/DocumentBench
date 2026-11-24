import React from 'react';
import Icon,{IconName} from './Icon';
export default function Modal({title,children,onClose,footer,wide=false,icon='document'}:{title:string;children:React.ReactNode;onClose:()=>void;footer?:React.ReactNode;wide?:boolean;icon?:IconName}){
 return <div className="modal-scrim" role="presentation" onMouseDown={e=>{if(e.target===e.currentTarget)onClose()}}><section className={`modal ${wide?'modal-wide':''}`} role="dialog" aria-modal="true" aria-label={title}>
  <header className="modal-header"><span className="modal-title"><Icon name={icon} size={19}/>{title}</span><button className="icon-button" onClick={onClose} aria-label="Close dialog"><Icon name="close"/></button></header>
  <div className="modal-body">{children}</div>{footer&&<footer className="modal-footer">{footer}</footer>}
 </section></div>
}
export function Field({label,children,hint}:{label:string;children:React.ReactNode;hint?:string}){return <label className="field"><span className="field-label">{label}</span>{children}{hint&&<span className="field-hint">{hint}</span>}</label>}
export function Notice({children,tone='info'}:{children:React.ReactNode;tone?:'info'|'warning'|'success'}){return <div className={`notice notice-${tone}`}><Icon name={tone==='warning'?'warning':tone==='success'?'check':'info'} size={17}/><span>{children}</span></div>}
