var __FLURX_IIFE__=function(a){"use strict";const e=(a,e=null)=>new Promise(((t,n)=>{const s=i(),o=`_${s}`;window.ipc.postMessage(JSON.stringify((e=>null==e?{type:"Command",message:{id:a,resolve_id:s}}:{type:"Command",message:{id:a,args:JSON.stringify(e),resolve_id:s}})(e))),Object.defineProperty(window.__FLURX__,o,{value:a=>{Reflect.deleteProperty(window.__FLURX__,o),(a=>(!a||"object"==typeof a)&&void 0!==a.Ok)(a)?t(a.Ok):(a=>(!a||"object"==typeof a)&&void 0!==a.Err)(a)?n(a.Err):t(a)},writable:!1,configurable:!0})})),t=(a,e)=>{window.ipc.postMessage(JSON.stringify({type:"Event",message:{event_id:a,payload:JSON.stringify(e)}}))},i=()=>window.crypto.getRandomValues(new Uint32Array(1))[0];var n=Object.freeze({__proto__:null,exit:()=>e("FLURX|app::exit"),getName:()=>e("FLURX|app::get_name"),getVersion:()=>e("FLURX|app::get_version")});var s=Object.freeze({__proto__:null,println:a=>t("FLURX|log::println",{message:a})});var o=Object.freeze({__proto__:null,copyFile:async(a,t,i)=>{await e("FLURX|fs::copy_file",{from:a,to:t,...i})},createDir:async(a,t)=>{await e("FLURX|fs::create_dir",{path:a,...t})},exists:async(a,t)=>await e("FLURX|fs::exists",{path:a,...t}),readBinaryFile:async(a,t)=>await e("FLURX|fs::read_binary_file",{path:a,...t}),readDir:async(a,t)=>await e("FLURX|fs::read_dir",{path:a,...t}),readTextFile:async(a,t)=>await e("FLURX|fs::read_text_file",{path:a,...t}),removeDir:async(a,t)=>{await e("FLURX|fs::remove_dir",{path:a,...t})},removeFile:async(a,t)=>{await e("FLURX|fs::remove_file",{path:a,...t})},renameFile:async(a,t,i)=>{await e("FLURX|fs::rename_file",{oldPath:a,newPath:t,...i})},writeBinaryFile:async(a,t,i)=>{await e("FLURX|fs::write_binary_file",{path:a,contents:t,...i})},writeTextFile:async(a,t,i)=>{await e("FLURX|fs::write_text_file",{path:a,contents:t,...i})}});var r=Object.freeze({__proto__:null,ask:async(a,t)=>await e("FLURX|dialog::ask",{questionMessage:a,...t}),confirm:async(a,t)=>await e("FLURX|dialog::confirm",{questionMessage:a,...t}),message:async(a,t)=>{await e("FLURX|dialog::message",{questionMessage:a,...t})},open:async a=>{const t=await e("FLURX|dialog::open",a);return i=t,i?.Single?t.Single:t.Multiple;var i},save:async a=>await e("FLURX|dialog::save",a)});var _=Object.freeze({__proto__:null,audio:async()=>await e("FLURX|path::audio"),cache:async()=>await e("FLURX|path::cache"),config:async()=>await e("FLURX|path::config"),configLocal:async()=>await e("FLURX|path::config_local"),data:async()=>await e("FLURX|path::data"),dataLocal:async()=>await e("FLURX|path::data_local"),desktop:async()=>await e("FLURX|path::desktop"),document:async()=>await e("FLURX|path::document"),download:async()=>await e("FLURX|path::download"),executable:async()=>await e("FLURX|path::executable"),home:async()=>await e("FLURX|path::home"),picture:async()=>await e("FLURX|path::picture"),publicDir:async()=>await e("FLURX|path::public"),runtime:async()=>await e("FLURX|path::runtime"),temp:async()=>await e("FLURX|path::temp"),template:async()=>await e("FLURX|path::template"),video:async()=>await e("FLURX|path::video")});var c=Object.freeze({__proto__:null,getText:async()=>await e("FLURX|clipboard::get_text"),setText:async a=>{await e("FLURX|clipboard::set_text",a)}});var l=Object.freeze({__proto__:null,send:async(a,t)=>{await e("FLURX|notification::send",{message:a,...t})}});return a.__emitEvent=(a,e)=>{window.__FLURX__[`_event_${a}`]?.(e)},a.__resolveIpc=(a,e)=>{window.__FLURX__[`_${a}`]?.(e)},a.app=n,a.clipboard=c,a.dialog=r,a.emit=t,a.fs=o,a.invoke=e,a.listen=(a,e)=>{const t=`_event_${a}`;return Object.defineProperty(window.__FLURX__,t,{value:e,writable:!1,configurable:!0}),()=>{Reflect.deleteProperty(window.__FLURX__,t)}},a.log=s,a.notification=l,a.path=_,a}({});Object.defineProperty(window,"__FLURX__",{value:__FLURX_IIFE__});
