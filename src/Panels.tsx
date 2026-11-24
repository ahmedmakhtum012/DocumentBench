import React from 'react';
import {Tab} from './ribbon';
import Icon from './Icon';
export type Panel='extract'|'assistant'|'stamps'|'fields'|'security'|'properties'|'signatures'|'watermark'|'bates'|'ocr'|'compare-settings'|null;
export function PanelHeader({title,subtitle,onClose,icon='sparkle'}:{title:string;subtitle?:string;onClose:()=>void;icon?:any}){return <div className="right-panel-header"><div><span className="panel-icon"><Icon name={icon} size={19}/></span><div><h3>{title}</h3>{subtitle&&<span>{subtitle}</span>}</div></div><button className="icon-button" onClick={onClose} aria-label="Close panel"><Icon name="close"/></button></div>}
export function EmptyPanel({icon,title,children}:{icon:any;title:string;children:React.ReactNode}){return <div className="panel-empty"><Icon name={icon} size={38}/><h4>{title}</h4><p>{children}</p></div>}
