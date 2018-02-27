#[macro_export]
macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    };
}

#[macro_export]
macro_rules! get_http_param {
    ($r: expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest))
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    }
}
