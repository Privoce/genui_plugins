/// A macro to simplify the creation of a GET request.
#[macro_export]
macro_rules! http_get {
    ($Url: expr, $cx: ident) => {
        http_get!($Url, crate::http::PatchRequest::default(), $cx)
    };
    ($Url: expr, $Patch: expr, $cx: ident) => {
        gen_macro::inject_ref!(http1).get($Url, $Patch).and_then(|request|{
            $cx.http_request(live_id!(A), request);
            Ok(())
        })
    };
}