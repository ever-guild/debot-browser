use serde_wasm_bindgen::from_value;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use ton_client::crypto::SigningBox;
use ton_client::error::{ClientError, ClientResult};
use ton_client::ClientContext;
use wasm_bindgen::prelude::*;

// Promise
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "object")]
    #[derive(Clone, Debug)]
    pub type Object;

    #[must_use]
    #[wasm_bindgen(extends = Object, typescript_type = "Promise<any>")]
    #[derive(Clone, Debug)]
    pub type Promise;

    #[wasm_bindgen(method, js_name = then)]
    pub fn then2(
        this: &Promise,
        resolve: &Closure<dyn FnMut(JsValue)>,
        reject: &Closure<dyn FnMut(JsValue)>,
    ) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type DAppSigningBox;

    #[wasm_bindgen(method)]
    pub fn get_public_key(this: &DAppSigningBox) -> Promise;

    #[wasm_bindgen(method)]
    pub fn sign(this: &DAppSigningBox, unsigned: &[u8]) -> Promise;

}

struct Inner {
    result: Option<Result<JsValue, JsValue>>,
    task: Option<Waker>,
    callbacks: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
}

pub struct JsFutureSync {
    inner: Arc<Mutex<Inner>>,
}

impl From<Promise> for JsFutureSync {
    fn from(js: Promise) -> JsFutureSync {
        let state = Arc::new(Mutex::new(Inner {
            result: None,
            task: None,
            callbacks: None,
        }));

        fn finish(state: Arc<Mutex<Inner>>, val: Result<JsValue, JsValue>) {
            let task = {
                let mut state = state.lock().unwrap();

                drop(state.callbacks.take());

                state.result = Some(val);
                state.task.take()
            };

            if let Some(task) = task {
                task.wake()
            }
        }

        let resolve = {
            let state = state.clone();
            Closure::once(move |val| finish(state, Ok(val)))
        };

        let reject = {
            let state = state.clone();
            Closure::once(move |val| finish(state, Err(val)))
        };

        let _ = js.then2(&resolve, &reject);

        state.lock().unwrap().callbacks = Some((resolve, reject));

        JsFutureSync { inner: state }
    }
}

impl Future for JsFutureSync {
    type Output = Result<JsValue, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();

        if let Some(val) = inner.result.take() {
            return Poll::Ready(val);
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

unsafe impl Sync for DAppSigningBox {}
unsafe impl Send for DAppSigningBox {}
unsafe impl Send for JsFutureSync {}
unsafe impl Send for Promise {}

fn sdk_mapper<E>(message: &str) -> impl FnOnce(E) -> ClientError {
    let message = message.to_string();
    move |_e| ClientError::with_code_message(0, message)
}

fn convert_jsvalue_to_vector(val: JsValue) -> ClientResult<Vec<u8>> {
    let hex_str: String = from_value(val).map_err(sdk_mapper("failed to decode JsValue"))?;
    hex::decode(hex_str).map_err(sdk_mapper("failed to decode string to Vec<u8>"))
}

#[async_trait::async_trait]
impl SigningBox for DAppSigningBox {
    async fn get_public_key(&self, _context: Arc<ClientContext>) -> ClientResult<Vec<u8>> {
        let val = JsFutureSync::from(self.get_public_key())
            .await
            .map_err(sdk_mapper("failed to get public key"))?;
        convert_jsvalue_to_vector(val)
    }

    async fn sign(&self, _context: Arc<ClientContext>, unsigned: &[u8]) -> ClientResult<Vec<u8>> {
        let val = JsFutureSync::from(self.sign(unsigned))
            .await
            .map_err(sdk_mapper("failed to sign"))?;
        let str_hex: String = from_value(val).map_err(sdk_mapper("failed to decode JsValue"))?;
        hex::decode(str_hex).map_err(|_| ClientError::with_code_message(0, String::new()))
    }
}
