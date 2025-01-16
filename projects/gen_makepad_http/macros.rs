/// A macro to simplify the creation of a GET request.
#[macro_export]
macro_rules! http_get {
    ($Id: tt, $cx: ident) => {
        http_get!(
            $Id,
            "",
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $cx: ident) => {
        http_get!(
            $Id,
            $Url,
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $Patch: expr, $cx: ident) => {
        gen_macro::inject_ref!(http1)
            .get($Url, $Patch, live_id!($Id))
            .and_then(|request| {
                $cx.http_request(live_id!($Id), request);
                Ok(())
            })
    };
}

/// A macro to simplify the creation of a POST request.
#[macro_export]
macro_rules! http_post {
    ($Id: tt, $cx: ident) => {
        http_post!(
            $Id,
            "",
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $cx: ident) => {
        http_post!(
            $Id,
            $Url,
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $Patch: expr, $cx: ident) => {
        gen_macro::inject_ref!(http1)
            .post($Url, $Patch, live_id!($Id))
            .and_then(|request| {
                $cx.http_request(live_id!($Id), request);
                Ok(())
            })
    };
}

#[macro_export]
macro_rules! http_put {
    ($Id: tt, $cx: ident) => {
        http_put!(
            $Id,
            "",
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $cx: ident) => {
        http_put!(
            $Id,
            $Url,
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $Patch: expr, $cx: ident) => {
        gen_macro::inject_ref!(http1)
            .put($Url, $Patch, live_id!($Id))
            .and_then(|request| {
                $cx.http_request(live_id!($Id), request);
                Ok(())
            })
    };
}

#[macro_export]
macro_rules! http_delete {
    ($Id: tt, $cx: ident) => {
        http_delete!(
            $Id,
            "",
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $cx: ident) => {
        http_delete!(
            $Id,
            $Url,
            crate::gen_makepad_http::PatchRequest::default(),
            $cx
        )
    };
    ($Id: tt, $Url: expr, $Patch: expr, $cx: ident) => {
        gen_macro::inject_ref!(http1)
            .delete($Url, $Patch, live_id!($Id))
            .and_then(|request| {
                $cx.http_request(live_id!($Id), request);
                Ok(())
            })
    };
}

#[macro_export]
macro_rules! http_request {
    ($(
        $F: ident : $M: path
    )*) => {
        $(
            pub fn $F<P>(&self, url: &str, patch: P, live_id: LiveId) -> Result<HttpRequest, Box<dyn Error>>
            where
                P: Into<PatchRequest>,
            {
                self.send_request(url, patch, $M, live_id)
            }
        )*
    };
}
