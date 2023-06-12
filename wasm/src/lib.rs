mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use web_sys::{console, Request, RequestInit, RequestMode, Response, Blob, window};
use js_sys::{Function, Array};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &JsValue) {
    console::log_1(&msg);
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub async fn lfetch(loc: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let host = &window().unwrap().location().hostname().unwrap();
    let url = format!("http://{}:8080/{}", host, loc);
    console::log_1(&JsValue::from_str(&url));
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Accept", "text/text")?;

    let window = web_sys::window().unwrap();
    let resp_val = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_val.is_instance_of::<Response>());
    let resp: Response = resp_val.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    //let json = JsFuture::from(resp.json()?).await?;
    //let blob = JsFuture::from(resp.blob()?).await?;

    Ok(text)
}

#[wasm_bindgen]
pub async fn send_rpc(cmd: String, args: Array, cb: Function) {
    let jcmd = JsValue::from_str(&cmd);
    console::log_1(&jcmd);
    /*if args < 7 {

    }*/
    console::log_1(&args);
    cb.call2(&JsValue::NULL, &jcmd, &args);
    console::log_1(&JsValue::from_str(&cmd));
}

//pub async fn rsfetch(cmd: String, args: )
