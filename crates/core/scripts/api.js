var __FLURX_IIFE__=function(e){"use strict";const t=(e,t=null)=>new Promise(((i,r)=>{const _=a(),n=`_${_}`;window.ipc.postMessage(JSON.stringify((t=>null==t?{type:"Command",message:{id:e,resolve_id:_}}:{type:"Command",message:{id:e,args:JSON.stringify(t),resolve_id:_}})(t))),Object.defineProperty(window.__FLURX__,n,{value:e=>{Reflect.deleteProperty(window.__FLURX__,n),(e=>(!e||"object"==typeof e)&&void 0!==e.Ok)(e)?i(e.Ok):(e=>(!e||"object"==typeof e)&&void 0!==e.Err)(e)?r(e.Err):i(e)},writable:!1,configurable:!0})})),i=(e,t)=>{window.ipc.postMessage(JSON.stringify({type:"Event",message:{event_id:e,payload:JSON.stringify(t)}}))},a=()=>window.crypto.getRandomValues(new Uint32Array(1))[0];var r=Object.freeze({__proto__:null,exit:()=>t("FLURX|app::exit"),getName:()=>t("FLURX|app::get_name"),getVersion:()=>t("FLURX|app::get_version")});var _=Object.freeze({__proto__:null,println:e=>i("FLURX|log::println",{message:e})});var n=Object.freeze({__proto__:null,copyFile:async(e,i,a)=>{await t("FLURX|fs::copy_file",{from:e,to:i,...a})},createDir:async(e,i)=>{await t("FLURX|fs::create_dir",{path:e,...i})},exists:async(e,i)=>await t("FLURX|fs::exists",{path:e,...i}),readBinaryFile:async e=>await t("FLURX|fs::read_binary_file",e),readDir:async e=>{throw new Error("not impl")},readTextFile:async(e,i)=>await t("FLURX|fs::read_text_file",{path:e,...i}),removeDir:async(e,i)=>{await t("FLURX|fs::remove_dir",{path:e,...i})},removeFile:async(e,i)=>{await t("FLURX|fs::remove_file",{path:e,...i})},renameFile:async(e,i)=>{await t("FLURX|fs::rename_file",[e,i])},writeBinaryFile:async(e,i,a)=>{await t("FLURX|fs::write_binary_file",{path:e,contents:i,...a})},writeTextFile:async(e,i,a)=>{await t("FLURX|fs::write_text_file",{path:e,contents:i,...a})}});return e.__emitEvent=(e,t)=>{window.__FLURX__[`_event_${e}`]?.(t)},e.__resolveIpc=(e,t)=>{window.__FLURX__[`_${e}`]?.(t)},e.app=r,e.emit=i,e.fs=n,e.invoke=t,e.listen=(e,t)=>{const i=`_event_${e}`;return Object.defineProperty(window.__FLURX__,i,{value:t,writable:!1,configurable:!0}),()=>{Reflect.deleteProperty(window.__FLURX__,i)}},e.log=_,e}({});Object.defineProperty(window,"__FLURX__",{value:__FLURX_IIFE__});
