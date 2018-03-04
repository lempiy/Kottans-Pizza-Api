#[macro_export]
macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                let response = super::ErrorResponse{
                    success: false,
                    error: e.description().to_string()
                };
                if let Ok(res) = serde_json::to_string(&response) {
                    return Ok(Response::with((status::InternalServerError, res)))
                }
                return Ok(Response::with((status::InternalServerError, e.description())))
            }
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                let response = super::ErrorResponse{
                    success: false,
                    error: e.description().to_string()
                };
                if let Ok(res) = serde_json::to_string(&response) {
                    return Ok(Response::with(($error, res)))
                }
                return Ok(Response::with(($error, e.description())))
            }
        }
    };
}

#[macro_export]
macro_rules! try_validate {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                let response = super::ErrorResponseWithValidation{
                    success: false,
                    error: e.description().to_string(),
                    validations:
                        e.inner()
                            .values()
                            .flat_map(|x| {
                                x.iter()
                            })
                            .filter_map(|x| {
                                if let Some(ref msg) = x.message {
                                    Some(msg.to_string())
                                } else {
                                    None
                                }
                            }).collect(),
                };
                if let Ok(res) = serde_json::to_string(&response) {
                    return Ok(Response::with((status::BadRequest, res)))
                }
                return Ok(Response::with((status::BadRequest, "Server error")))
            }
        }
    };
    // rust::validator cannot work with external params inside custom validator
    // this is handler pattern for additional (in ex. DB depended) error validation
    ($e:expr, $more_errors: expr) => {
        match $e {
            Ok(x) => {
                if $more_errors
                    .into_iter()
                    .filter(|x| if let &Err(_) = x {true}else{false})
                        .collect::<Vec<Result<(), ValidationError>>>().len() > 0 {
                    let response = super::ErrorResponseWithValidation{
                        success: false,
                        error: "Validation error".to_string(),
                        validations: $more_errors
                            .into_iter()
                            .filter_map(|x| {
                                if let Err(a) = x {
                                    Some(a)
                                } else {
                                    None
                                }
                            })
                            .filter_map(|x| {
                                if let Some(ref msg) = x.message {
                                    Some(msg.to_string())
                                } else {
                                    None
                                }
                            }).collect()
                    };
                    if let Ok(res) = serde_json::to_string(&response) {
                        return Ok(Response::with((status::BadRequest, res)))
                    };
                    return Ok(Response::with((status::BadRequest, "Server error")))
                } else {
                    x
                }
            },
            Err(e) => {
                let response = super::ErrorResponseWithValidation{
                    success: false,
                    error: e.description().to_string(),
                    validations:
                        e.inner()
                            .into_iter()
                            .flat_map(|x| {
                                let (_, v) = x;
                                v.into_iter()
                            })
                            .chain(
                                $more_errors
                                    .into_iter()
                                    .filter_map(|x| {
                                        if let Err(a) = x {
                                            Some(a)
                                        } else {
                                            None
                                        }
                                    })
                            )
                            .filter_map(|x| {
                                if let Some(ref msg) = x.message {
                                    Some(msg.to_string())
                                } else {
                                    None
                                }
                            }).collect(),
                };
                if let Ok(res) = serde_json::to_string(&response) {
                    return Ok(Response::with((status::BadRequest, res)))
                }
                return Ok(Response::with((status::BadRequest, "Server error")))
            }
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
