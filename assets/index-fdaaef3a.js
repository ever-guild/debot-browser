var Gt=(B,v)=>()=>(v||B((v={exports:{}}).exports,v),v.exports);var Kt=Gt((Qt,W)=>{(async()=>{(function(){const n=document.createElement("link").relList;if(n&&n.supports&&n.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))t(r);new MutationObserver(r=>{for(const c of r)if(c.type==="childList")for(const i of c.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&t(i)}).observe(document,{childList:!0,subtree:!0});function e(r){const c={};return r.integrity&&(c.integrity=r.integrity),r.referrerPolicy&&(c.referrerPolicy=r.referrerPolicy),r.crossOrigin==="use-credentials"?c.credentials="include":r.crossOrigin==="anonymous"?c.credentials="omit":c.credentials="same-origin",c}function t(r){if(r.ep)return;r.ep=!0;const c=e(r);fetch(r.href,c)}})();const B="/v5/debot.svg",v="/v5/assets/debot_browser_bg-de24d5a6.wasm",P=async(n={},e)=>{let t;if(e.startsWith("data:")){const r=e.replace(/^data:.*?base64,/,"");let c;if(typeof Buffer=="function"&&typeof Buffer.from=="function")c=Buffer.from(r,"base64");else if(typeof atob=="function"){const i=atob(r);c=new Uint8Array(i.length);for(let b=0;b<i.length;b++)c[b]=i.charCodeAt(b)}else throw new Error("Cannot decode base64-encoded data URL");t=await WebAssembly.instantiate(c,n)}else{const r=await fetch(e),c=r.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&c.startsWith("application/wasm"))t=await WebAssembly.instantiateStreaming(r,n);else{const i=await r.arrayBuffer();t=await WebAssembly.instantiate(i,n)}}return t.instance.exports};let u;function V(n){u=n}const h=new Array(128).fill(void 0);h.push(void 0,null,!0,!1);function _(n){return h[n]}let A=h.length;function J(n){n<132||(h[n]=A,A=n)}function p(n){const e=_(n);return J(n),e}function l(n){return n==null}let C=null;function G(){return(C===null||C.byteLength===0)&&(C=new Float64Array(u.memory.buffer)),C}let S=null;function d(){return(S===null||S.byteLength===0)&&(S=new Int32Array(u.memory.buffer)),S}function o(n){A===h.length&&h.push(h.length+1);const e=A;return A=h[e],h[e]=n,e}let w=0,T=null;function O(){return(T===null||T.byteLength===0)&&(T=new Uint8Array(u.memory.buffer)),T}const K=typeof TextEncoder>"u"?(0,W.require)("util").TextEncoder:TextEncoder;let F=new K("utf-8");const H=typeof F.encodeInto=="function"?function(n,e){return F.encodeInto(n,e)}:function(n,e){const t=F.encode(n);return e.set(t),{read:n.length,written:t.length}};function y(n,e,t){if(t===void 0){const f=F.encode(n),m=e(f.length)>>>0;return O().subarray(m,m+f.length).set(f),w=f.length,m}let r=n.length,c=e(r)>>>0;const i=O();let b=0;for(;b<r;b++){const f=n.charCodeAt(b);if(f>127)break;i[c+b]=f}if(b!==r){b!==0&&(n=n.slice(b)),c=t(c,r,r=b+n.length*3)>>>0;const f=O().subarray(c+b,c+r),m=H(n,f);b+=m.written}return w=b,c}const Q=typeof TextDecoder>"u"?(0,W.require)("util").TextDecoder:TextDecoder;let E=new Q("utf-8",{ignoreBOM:!0,fatal:!0});E.decode();function g(n,e){return n=n>>>0,E.decode(O().subarray(n,n+e))}let j=null;function X(){return(j===null||j.byteLength===0)&&(j=new BigInt64Array(u.memory.buffer)),j}function I(n){const e=typeof n;if(e=="number"||e=="boolean"||n==null)return`${n}`;if(e=="string")return`"${n}"`;if(e=="symbol"){const c=n.description;return c==null?"Symbol":`Symbol(${c})`}if(e=="function"){const c=n.name;return typeof c=="string"&&c.length>0?`Function(${c})`:"Function"}if(Array.isArray(n)){const c=n.length;let i="[";c>0&&(i+=I(n[0]));for(let b=1;b<c;b++)i+=", "+I(n[b]);return i+="]",i}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(t.length>1)r=t[1];else return toString.call(n);if(r=="Object")try{return"Object("+JSON.stringify(n)+")"}catch{return"Object"}return n instanceof Error?`${n.name}: ${n.message}
${n.stack}`:r}function x(n,e,t,r){const c={a:n,b:e,cnt:1,dtor:t},i=(...b)=>{c.cnt++;const f=c.a;c.a=0;try{return r(f,c.b,...b)}finally{--c.cnt===0?u.__wbindgen_export_2.get(c.dtor)(f,c.b):c.a=f}};return i.original=c,i}function Y(n,e){u._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b3fe03cdd13362e(n,e)}function Z(n,e,t){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h96c04d5c73b8c0b1(n,e,o(t))}function nn(n,e,t){try{const i=u.__wbindgen_add_to_stack_pointer(-16);u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3c2320ea371e126(i,n,e,o(t));var r=d()[i/4+0],c=d()[i/4+1];if(c)throw p(r)}finally{u.__wbindgen_add_to_stack_pointer(16)}}function en(n,e,t){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7a02cd1b17724dc8(n,e,o(t))}function _n(n,e,t){u._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h90acb1d08e32c35f(n,e,o(t))}function L(n,e,t,r){const c={a:n,b:e,cnt:1,dtor:t},i=(...b)=>{c.cnt++;try{return r(c.a,c.b,...b)}finally{--c.cnt===0&&(u.__wbindgen_export_2.get(c.dtor)(c.a,c.b),c.a=0)}};return i.original=c,i}function tn(n,e,t){u._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8757be5717e4e827(n,e,o(t))}function rn(n,e){u._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1e2e27decc2c3b9c(n,e)}function M(n,e,t,r){const c=y(n,u.__wbindgen_malloc,u.__wbindgen_realloc),i=w,b=y(e,u.__wbindgen_malloc,u.__wbindgen_realloc),f=w;var m=l(t)?0:y(t,u.__wbindgen_malloc,u.__wbindgen_realloc),Dt=w,Pt=l(r)?0:y(r,u.__wbindgen_malloc,u.__wbindgen_realloc),Vt=w;const Jt=u.create_browser(c,i,b,f,m,Dt,Pt,Vt);return p(Jt)}function on(n){const e=u.destroy_browser(n);return p(e)}function cn(n,e){const t=u.run_browser(n,o(e));return p(t)}function U(n,e){return n=n>>>0,O().subarray(n/1,n/1+e)}function a(n,e){try{return n.apply(this,e)}catch(t){u.__wbindgen_exn_store(o(t))}}function sn(n,e,t,r){u.wasm_bindgen__convert__closures__invoke2_mut__h5c84ba232f440f37(n,e,o(t),o(r))}function un(n){p(n)}function an(n){return _(n)===void 0}function bn(n,e){return _(n)in _(e)}function fn(n){const e=_(n);return typeof e=="boolean"?e?1:0:2}function dn(n){return typeof _(n)=="bigint"}function gn(n,e){const t=_(e),r=typeof t=="number"?t:void 0;G()[n/8+1]=l(r)?0:r,d()[n/4+0]=!l(r)}function ln(n){return o(n)}function wn(n,e){return _(n)===_(e)}function mn(n,e){const t=_(e),r=typeof t=="string"?t:void 0;var c=l(r)?0:y(r,u.__wbindgen_malloc,u.__wbindgen_realloc),i=w;d()[n/4+1]=i,d()[n/4+0]=c}function pn(n){const e=_(n);return typeof e=="object"&&e!==null}function yn(n){const e=BigInt.asUintN(64,n);return o(e)}function hn(n){return typeof _(n)=="string"}function vn(n,e){const t=g(n,e);return o(t)}function kn(n,e){const t=new Error(g(n,e));return o(t)}function An(n,e,t){const r=_(n).then(_(e),_(t));return o(r)}function On(n){const e=_(n).get_public_key();return o(e)}function xn(n,e,t){const r=_(n).sign(U(e,t));return o(r)}function Rn(n){const e=p(n).original;return e.cnt--==1?(e.a=0,!0):!1}function Wn(n){const e=_(n);return o(e)}function Cn(n){const e=_(n).Window;return o(e)}function Sn(n){const e=_(n).WorkerGlobalScope;return o(e)}function Tn(n){return o(n)}function Fn(){return a(function(){const n=self.self;return o(n)},arguments)}function jn(n){const e=_(n).crypto;return o(e)}function Mn(n){const e=_(n).msCrypto;return o(e)}function qn(n,e,t){const r=_(n).require(g(e,t));return o(r)}function $n(n){const e=_(n).getRandomValues;return o(e)}function Bn(n,e){_(n).getRandomValues(_(e))}function In(n,e,t){_(n).randomFillSync(U(e,t))}function zn(){return o(W)}function En(){return a(function(n,e){_(n).randomFillSync(p(e))},arguments)}function Ln(){return a(function(n,e){_(n).getRandomValues(_(e))},arguments)}function Un(n){const e=_(n).crypto;return o(e)}function Nn(n){const e=_(n).process;return o(e)}function Dn(n){const e=_(n).versions;return o(e)}function Pn(n){const e=_(n).node;return o(e)}function Vn(){return a(function(){const n=W.require;return o(n)},arguments)}function Jn(n){const e=_(n).msCrypto;return o(e)}function Gn(n,e){return _(n)==_(e)}function Kn(n,e){const t=_(n)[_(e)];return o(t)}function Hn(n,e,t){_(n)[p(e)]=p(t)}function Qn(n,e){const t=String(_(e)),r=y(t,u.__wbindgen_malloc,u.__wbindgen_realloc),c=w;d()[n/4+1]=c,d()[n/4+0]=r}function Xn(n){let e;try{e=_(n)instanceof Window}catch{e=!1}return e}function Yn(){return a(function(n){const e=_(n).indexedDB;return l(e)?0:o(e)},arguments)}function Zn(n,e){_(n).clearTimeout(e)}function ne(n,e){const t=_(n).fetch(_(e));return o(t)}function ee(){return a(function(n,e,t){return _(n).setTimeout(_(e),t)},arguments)}function _e(n){console.log(_(n))}function te(n,e,t){const r=_(e).item(t>>>0);var c=l(r)?0:y(r,u.__wbindgen_malloc,u.__wbindgen_realloc),i=w;d()[n/4+1]=i,d()[n/4+0]=c}function re(n,e){_(n).onblocked=_(e)}function oe(n,e){_(n).onupgradeneeded=_(e)}function ce(){return a(function(n){const e=_(n).result;return o(e)},arguments)}function se(){return a(function(n){const e=_(n).error;return l(e)?0:o(e)},arguments)}function ue(n){const e=_(n).readyState;return o(e)}function ie(n,e){_(n).onsuccess=_(e)}function ae(n,e){_(n).onerror=_(e)}function be(n){const e=_(n).data;return o(e)}function fe(n){const e=_(n).headers;return o(e)}function de(){return a(function(n,e,t){const r=new Request(g(n,e),_(t));return o(r)},arguments)}function ge(n){const e=_(n).target;return l(e)?0:o(e)}function le(){return a(function(n,e,t){const r=_(n).open(g(e,t));return o(r)},arguments)}function we(){return a(function(n){const e=_(n).indexedDB;return l(e)?0:o(e)},arguments)}function me(){return a(function(n,e){const t=_(n).delete(_(e));return o(t)},arguments)}function pe(){return a(function(n,e){const t=_(n).get(_(e));return o(t)},arguments)}function ye(){return a(function(n,e,t){const r=_(n).put(_(e),_(t));return o(r)},arguments)}function he(n,e){_(n).onabort=_(e)}function ve(n,e){_(n).oncomplete=_(e)}function ke(n,e){_(n).onerror=_(e)}function Ae(){return a(function(n,e,t){const r=_(n).objectStore(g(e,t));return o(r)},arguments)}function Oe(n,e){_(n).onopen=_(e)}function xe(n,e){_(n).onerror=_(e)}function Re(n,e){_(n).onmessage=_(e)}function We(){return a(function(n,e){const t=new WebSocket(g(n,e));return o(t)},arguments)}function Ce(){return a(function(n,e,t,r){const c=new WebSocket(g(n,e),g(t,r));return o(c)},arguments)}function Se(){return a(function(n){_(n).close()},arguments)}function Te(){return a(function(n,e,t){_(n).send(g(e,t))},arguments)}function Fe(n){const e=_(n).objectStoreNames;return o(e)}function je(n,e){_(n).onversionchange=_(e)}function Me(){return a(function(n,e,t){const r=_(n).createObjectStore(g(e,t));return o(r)},arguments)}function qe(){return a(function(n,e,t,r){const c=_(n).transaction(g(e,t),p(r));return o(c)},arguments)}function $e(n,e){const t=_(e).message,r=y(t,u.__wbindgen_malloc,u.__wbindgen_realloc),c=w;d()[n/4+1]=c,d()[n/4+0]=r}function Be(){return a(function(n,e,t,r,c){_(n).set(g(e,t),g(r,c))},arguments)}function Ie(n){let e;try{e=_(n)instanceof Response}catch{e=!1}return e}function ze(n,e){const t=_(e).url,r=y(t,u.__wbindgen_malloc,u.__wbindgen_realloc),c=w;d()[n/4+1]=c,d()[n/4+0]=r}function Ee(n){return _(n).status}function Le(){return a(function(n){const e=_(n).text();return o(e)},arguments)}function Ue(n,e){const t=_(n)[e>>>0];return o(t)}function Ne(n){return _(n).length}function De(){const n=new Array;return o(n)}function Pe(n){return typeof _(n)=="function"}function Ve(n,e){const t=new Function(g(n,e));return o(t)}function Je(){return o(new Map)}function Ge(n){const e=_(n).next;return o(e)}function Ke(){return a(function(n){const e=_(n).next();return o(e)},arguments)}function He(n){return _(n).done}function Qe(n){const e=_(n).value;return o(e)}function Xe(){return o(Symbol.iterator)}function Ye(){return a(function(n,e){const t=Reflect.get(_(n),_(e));return o(t)},arguments)}function Ze(){return a(function(n,e){const t=_(n).call(_(e));return o(t)},arguments)}function n_(){const n=new Object;return o(n)}function e_(){return a(function(){const n=self.self;return o(n)},arguments)}function __(){return a(function(){const n=window.window;return o(n)},arguments)}function t_(){return a(function(){const n=globalThis.globalThis;return o(n)},arguments)}function r_(){return a(function(){const n=global.global;return o(n)},arguments)}function o_(n,e,t){_(n)[e>>>0]=p(t)}function c_(n){return Array.isArray(_(n))}function s_(n){let e;try{e=_(n)instanceof ArrayBuffer}catch{e=!1}return e}function u_(n){let e;try{e=_(n)instanceof Error}catch{e=!1}return e}function i_(n){const e=_(n).message;return o(e)}function a_(){return a(function(n,e,t){const r=_(n).call(_(e),_(t));return o(r)},arguments)}function b_(n,e,t){const r=_(n).set(_(e),_(t));return o(r)}function f_(n){return Number.isSafeInteger(_(n))}function d_(n){return _(n).getTime()}function g_(n){return _(n).getTimezoneOffset()}function l_(){return o(new Date)}function w_(n){const e=Object.entries(_(n));return o(e)}function m_(n,e){try{var t={a:n,b:e},r=(i,b)=>{const f=t.a;t.a=0;try{return sn(f,t.b,i,b)}finally{t.a=f}};const c=new Promise(r);return o(c)}finally{t.a=t.b=0}}function p_(n){const e=Promise.resolve(_(n));return o(e)}function y_(n,e){const t=_(n).then(_(e));return o(t)}function h_(n,e,t){const r=_(n).then(_(e),_(t));return o(r)}function v_(n){const e=_(n).buffer;return o(e)}function k_(n,e,t){const r=new Uint8Array(_(n),e>>>0,t>>>0);return o(r)}function A_(n){const e=new Uint8Array(_(n));return o(e)}function O_(n,e,t){_(n).set(_(e),t>>>0)}function x_(n){return _(n).length}function R_(n){let e;try{e=_(n)instanceof Uint8Array}catch{e=!1}return e}function W_(n){const e=new Uint8Array(n>>>0);return o(e)}function C_(n,e,t){const r=_(n).subarray(e>>>0,t>>>0);return o(r)}function S_(){return a(function(n,e,t){return Reflect.set(_(n),_(e),_(t))},arguments)}function T_(){return a(function(n){const e=JSON.stringify(_(n));return o(e)},arguments)}function F_(n,e){const t=_(e),r=typeof t=="bigint"?t:void 0;X()[n/8+1]=l(r)?BigInt(0):r,d()[n/4+0]=!l(r)}function j_(n,e){const t=I(_(e)),r=y(t,u.__wbindgen_malloc,u.__wbindgen_realloc),c=w;d()[n/4+1]=c,d()[n/4+0]=r}function M_(n,e){throw new Error(g(n,e))}function q_(){const n=u.memory;return o(n)}function $_(n,e,t){const r=x(n,e,1720,Y);return o(r)}function B_(n,e,t){const r=x(n,e,1720,Z);return o(r)}function I_(n,e,t){const r=x(n,e,1720,nn);return o(r)}function z_(n,e,t){const r=x(n,e,1720,en);return o(r)}function E_(n,e,t){const r=x(n,e,3523,_n);return o(r)}function L_(n,e,t){const r=L(n,e,3541,tn);return o(r)}function U_(n,e,t){const r=L(n,e,3541,rn);return o(r)}URL=globalThis.URL;const s=await P({"./debot_browser_bg.js":{__wbindgen_object_drop_ref:un,__wbindgen_is_undefined:an,__wbindgen_in:bn,__wbindgen_boolean_get:fn,__wbindgen_is_bigint:dn,__wbindgen_number_get:gn,__wbindgen_bigint_from_i64:ln,__wbindgen_jsval_eq:wn,__wbindgen_string_get:mn,__wbindgen_is_object:pn,__wbindgen_bigint_from_u64:yn,__wbindgen_is_string:hn,__wbindgen_string_new:vn,__wbindgen_error_new:kn,__wbg_then_c6e589d0bbc6d0be:An,__wbg_getpublickey_4587582aa2216f14:On,__wbg_sign_b901f5515c041fbd:xn,__wbindgen_cb_drop:Rn,__wbindgen_object_clone_ref:Wn,__wbg_Window_5684341ff6dfe3ad:Cn,__wbg_WorkerGlobalScope_e0447ffcae8bb272:Sn,__wbindgen_number_new:Tn,__wbg_self_7eede1f4488bf346:Fn,__wbg_crypto_c909fb428dcbddb6:jn,__wbg_msCrypto_511eefefbfc70ae4:Mn,__wbg_require_900d5c3984fe7703:qn,__wbg_getRandomValues_307049345d0bd88c:$n,__wbg_getRandomValues_cd175915511f705e:Bn,__wbg_randomFillSync_85b3f4c52c56c313:In,__wbg_static_accessor_MODULE_ef3aa2eb251158a5:zn,__wbg_randomFillSync_e950366c42764a07:En,__wbg_getRandomValues_3774744e221a22ad:Ln,__wbg_crypto_70a96de3b6b73dac:Un,__wbg_process_dd1577445152112e:Nn,__wbg_versions_58036bec3add9e6f:Dn,__wbg_node_6a9d28205ed5b0d8:Pn,__wbg_require_f05d779769764e82:Vn,__wbg_msCrypto_adbc770ec9eca9c7:Jn,__wbindgen_jsval_loose_eq:Gn,__wbg_getwithrefkey_5e6d9547403deab8:Kn,__wbg_set_841ac57cff3d672b:Hn,__wbg_String_88810dfeb4021902:Qn,__wbg_instanceof_Window_f2bf9e8e91f1be0d:Xn,__wbg_indexedDB_969f5ab05ee7ae14:Yn,__wbg_clearTimeout_c858b1cf4a2dab60:Zn,__wbg_fetch_33e667e877bf8066:ne,__wbg_setTimeout_250d9729242b4d13:ee,__wbg_log_003c998d6df63565:_e,__wbg_item_d6b36f33bfd983dc:te,__wbg_setonblocked_397eb895dc26e2b6:re,__wbg_setonupgradeneeded_41b9b274103a9aa5:oe,__wbg_result_c7489dcce5ede0f6:ce,__wbg_error_83e5a3ca69be6473:se,__wbg_readyState_a94c74b581a57107:ue,__wbg_setonsuccess_7df16107c9c8cb86:ie,__wbg_setonerror_93b8915fbe25d5a2:ae,__wbg_data_ef47af9c565d228b:be,__wbg_headers_142abdd2a9b86d0f:fe,__wbg_newwithstrandinit_8e1c089763754d1e:de,__wbg_target_303861d1c3271001:ge,__wbg_open_96c856e7c4adc9bb:le,__wbg_indexedDB_730f62561311ce85:we,__wbg_delete_7ca239dc61252742:me,__wbg_get_dc9b064547e71f80:pe,__wbg_put_401e37fd76ae5de3:ye,__wbg_setonabort_cdc15c722789920d:he,__wbg_setoncomplete_12d54ddbb2ab877f:ve,__wbg_setonerror_ee89c02ada7a3a9b:ke,__wbg_objectStore_3976d990dc1c78d8:Ae,__wbg_setonopen_6fd8b28538150568:Oe,__wbg_setonerror_9f7532626d7a9ce2:xe,__wbg_setonmessage_493b82147081ec7e:Re,__wbg_new_39e958ac9d5cae7d:We,__wbg_newwithstr_d45cb0be79c8b045:Ce,__wbg_close_18f6acc05e28b66d:Se,__wbg_send_45db219b9f40cc7e:Te,__wbg_objectStoreNames_69f2340f53fb79e2:Fe,__wbg_setonversionchange_69389ce6cc312fec:je,__wbg_createObjectStore_12cc3cfaa4935622:Me,__wbg_transaction_1d9975e42cba4d6f:qe,__wbg_message_3c3b922f470f01bb:$e,__wbg_set_c146eed0996fb31d:Be,__wbg_instanceof_Response_b1d8fb5649a38770:Ie,__wbg_url_8e528fd65523cbe8:ze,__wbg_status_27590aae3bea771c:Ee,__wbg_text_01d2781c04763803:Le,__wbg_get_e52aaca45f37b337:Ue,__wbg_length_070e3265c186df02:Ne,__wbg_new_18bc2084e9a3e1ff:De,__wbindgen_is_function:Pe,__wbg_newnoargs_e643855c6572a4a8:Ve,__wbg_new_b6fd0149e79ffce8:Je,__wbg_next_3975dcca26737a22:Ge,__wbg_next_5a9700550e162aa3:Ke,__wbg_done_a184612220756243:He,__wbg_value_6cc144c1d9645dd5:Qe,__wbg_iterator_c1677479667ea090:Xe,__wbg_get_363c3b466fe4896b:Ye,__wbg_call_f96b398515635514:Ze,__wbg_new_7befa02319b36069:n_,__wbg_self_b9aad7f1c618bfaf:e_,__wbg_window_55e469842c98b086:__,__wbg_globalThis_d0957e302752547e:t_,__wbg_global_ae2f87312b8987fb:r_,__wbg_set_aee8682c7ee9ac44:o_,__wbg_isArray_07d89ced8fb14171:c_,__wbg_instanceof_ArrayBuffer_de688b806c28ff28:s_,__wbg_instanceof_Error_138ea316d387a37b:u_,__wbg_message_eb40690f9108eb28:i_,__wbg_call_35782e9a1aa5e091:a_,__wbg_set_6c1b2b7b73337778:b_,__wbg_isSafeInteger_fcdf4c4f25c86778:f_,__wbg_getTime_1f655755b697302c:d_,__wbg_getTimezoneOffset_652d68042260ca29:g_,__wbg_new0_d2a7d711adb0fe0f:l_,__wbg_entries_c3e06bf0354f5d20:w_,__wbg_new_113855d7ab252420:m_,__wbg_resolve_f3a7b38cd2af0fa4:p_,__wbg_then_65c9631eb0022205:y_,__wbg_then_cde1713a812adbda:h_,__wbg_buffer_fcbfb6d88b2732e9:v_,__wbg_newwithbyteoffsetandlength_92c251989c485785:k_,__wbg_new_bc5d9aad3f9ac80e:A_,__wbg_set_4b3aa8445ac1e91c:O_,__wbg_length_d9c4ded7e708c6a1:x_,__wbg_instanceof_Uint8Array_4733577ba827276b:R_,__wbg_newwithlength_89eca18f2603a999:W_,__wbg_subarray_7649d027b2b141b3:C_,__wbg_set_bc33b7c3be9319b5:S_,__wbg_stringify_9003c389758d16d4:T_,__wbindgen_bigint_get_as_i64:F_,__wbindgen_debug_string:j_,__wbindgen_throw:M_,__wbindgen_memory:q_,__wbindgen_closure_wrapper2115:$_,__wbindgen_closure_wrapper2116:B_,__wbindgen_closure_wrapper2119:I_,__wbindgen_closure_wrapper2121:z_,__wbindgen_closure_wrapper14562:E_,__wbindgen_closure_wrapper14625:L_,__wbindgen_closure_wrapper14627:U_}},v),N_=s.memory,D_=s.init_log,P_=s.run_debot_browser,V_=s.create_browser,J_=s.destroy_browser,G_=s.run_browser,K_=s.update_user_settings,H_=s.generate_keypair,Q_=s.sign,X_=s.register_signing_box,Y_=s.close_signing_box,Z_=s.signing_box_public_key,nt=s.sha256,et=s.chacha20,_t=s.scrypt,tt=s.generate_random_bytes,rt=s.tc_create_context,ot=s.tc_request,ct=s.tc_request_ptr,st=s.tc_request_sync,ut=s.tc_destroy_string,it=s.tc_read_string,at=s.tc_destroy_context,bt=s.rust_zstd_wasm_shim_malloc,ft=s.rust_zstd_wasm_shim_calloc,dt=s.rust_zstd_wasm_shim_free,gt=s.rust_zstd_wasm_shim_memcpy,lt=s.rust_zstd_wasm_shim_memmove,wt=s.rust_zstd_wasm_shim_memset,mt=s.__wbindgen_malloc,pt=s.__wbindgen_realloc,yt=s.__wbindgen_export_2,ht=s._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b3fe03cdd13362e,vt=s._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h96c04d5c73b8c0b1,kt=s.__wbindgen_add_to_stack_pointer,At=s._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3c2320ea371e126,Ot=s._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7a02cd1b17724dc8,xt=s._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h90acb1d08e32c35f,Rt=s._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8757be5717e4e827,Wt=s._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1e2e27decc2c3b9c,Ct=s.__wbindgen_free,St=s.__wbindgen_exn_store,Tt=s.wasm_bindgen__convert__closures__invoke2_mut__h5c84ba232f440f37,Ft=Object.freeze(Object.defineProperty({__proto__:null,__wbindgen_add_to_stack_pointer:kt,__wbindgen_exn_store:St,__wbindgen_export_2:yt,__wbindgen_free:Ct,__wbindgen_malloc:mt,__wbindgen_realloc:pt,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7a02cd1b17724dc8:Ot,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h90acb1d08e32c35f:xt,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h96c04d5c73b8c0b1:vt,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3c2320ea371e126:At,_dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b3fe03cdd13362e:ht,_dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8757be5717e4e827:Rt,_dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1e2e27decc2c3b9c:Wt,chacha20:et,close_signing_box:Y_,create_browser:V_,destroy_browser:J_,generate_keypair:H_,generate_random_bytes:tt,init_log:D_,memory:N_,register_signing_box:X_,run_browser:G_,run_debot_browser:P_,rust_zstd_wasm_shim_calloc:ft,rust_zstd_wasm_shim_free:dt,rust_zstd_wasm_shim_malloc:bt,rust_zstd_wasm_shim_memcpy:gt,rust_zstd_wasm_shim_memmove:lt,rust_zstd_wasm_shim_memset:wt,scrypt:_t,sha256:nt,sign:Q_,signing_box_public_key:Z_,tc_create_context:rt,tc_destroy_context:at,tc_destroy_string:ut,tc_read_string:it,tc_request:ot,tc_request_ptr:ct,tc_request_sync:st,update_user_settings:K_,wasm_bindgen__convert__closures__invoke2_mut__h5c84ba232f440f37:Tt},Symbol.toStringTag,{value:"Module"}));V(Ft);const jt=0,Mt="",qt="invokeTest",$t={arg1:"1500000000",arg2:"68656c6c6f20776f726c6421",arg3:!0,arg4:3,arg5:"0:e859a5858fc99c8f6044aa179af68140c2fb9b07b3f52b70bef51e0c799fd2df",arg6:"",arg7:{1:{data:"10"},2:{data:"2020"}}},Bt={"ABI version":2,header:[],functions:[{name:"OnInvokeCompleted",inputs:[{name:"status",type:"uint8"},{components:[{name:"data",type:"bytes"}],name:"ret1",type:"map(uint32,tuple)"}],outputs:[]}],data:[],events:[]},It=!0,zt=[{type:"Input",interface:"a1d347099e29c1624c8890619daf207bde18e92df5220a54bcc6d858309ece84",method:"get",params:{value:"1500000000"}},{type:"Input",interface:"8796536366ee21852db56dccb60bc564598b618c865fc50c8b1ab740bba128e3",method:"input",params:{value:"68656c6c6f20776f726c6421"}},{type:"Input",interface:"16653eaf34c921467120f2685d425ff963db5cbb5aa676a62a2e33bfc3f6828a",method:"get",params:{value:!0}},{type:"Input",interface:"ac1a4d3ecea232e49783df4a23a81823cdca3205dc58cd20c4db259c25605b48",method:"select",params:{index:3}},{type:"Input",interface:"d7ed1bd8e6230871116f4522e58df0a93c5520c56f4ade23ef3d8919a984653b",method:"get",params:{value:"0:e859a5858fc99c8f6044aa179af68140c2fb9b07b3f52b70bef51e0c799fd2df"}}],z={version:jt,debotAddress:Mt,initMethod:qt,initArgs:$t,abi:Bt,quiet:It,chain:zt};let Et={public:"9f7fd3df9d72b133fe155c087928c4f9da423076cc20c9f5386614b462e49811",secret:"607a90aedb5df02a0f712572f0b5aa5d9342e5f3f2c0794df43f4a2a9688aef3"};const Lt="bf520b125fe24b96b4545f4358d2edba",N="devnet",q=`https://${N}.evercloud.dev/${Lt}/graphql`,k="0:2d2696edfe3d7c0d74e8900b2a43ac362de5a45db6fe6147177e2fcd2abfd3e2",$="0:2f9f742cd3ed63c39a31c077d5faada4e52ea365a4b4a9e1d6709e6cb0e9d927",R=`0x${Et.public}`,D=3;async function Ut(n){const e=m=>{n.textContent+=`${m}
`};z.debotAddress=k,z.initArgs.arg6=R,e(`NETWORK=${N}`),e(`DEBOT=${k}`);let t;e(`Test 2. Create, run, destroy browser (${D} calls)`);const r=await M(q,k,$,R);console.time("Test 2");for(let m=0;m<D;m++)t=await cn(r,z);console.timeEnd("Test 2"),console.log("Result:",t),e(JSON.stringify(t)),await on(r),e("Test 2. Completed"),e("Test 3. Create 3 browsers in parallel"),console.time("Test 3");const c=M(q,k,$,R),i=M(q,k,$,R),b=M(q,k,$,R),f=await Promise.all([c,i,b]);console.log(`handle1 = ${f[0]} handle2 = ${f[1]} handle3 = ${f[2]}`),console.timeEnd("Test 3"),e("Test 3. Completed.")}async function Nt(){document.querySelector("#app").innerHTML=`
  <div>
    <a href="https://www.npmjs.com/package/@ever-guild/debot-browser" target="_blank">
      <img src="${B}" class="logo" alt="debot logo" />
    </a>
    <h1>Bebot browser</h1>
    <div class="card">
      <pre>
        <code id="log"></code>
      </pre>
    </div>
  </div>
`,await Ut(document.querySelector("#log"))}Nt().catch(console.error)})()});export default Kt();