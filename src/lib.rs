use std::convert::Infallible;
use std::ffi::CStr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use libc::{strdup, c_char};


#[tokio::main]
pub async fn listener(ip_addr: [u8;4], port: u16, routes: Vec<String>, handlers: Vec<extern "C" fn(*const C_Request, *mut C_Response)>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let make_svc = make_service_fn(move |_conn| {
        let routes = routes.clone();
        let handlers = handlers.clone();
        async move { 
            Ok::<_, Infallible>(
                service_fn(move |req: Request<Body>| {
                    let routes = routes.clone();
                    let handlers = handlers.clone();
                    let mut menssage = String::from("nothing to see here");
                    let c_req = C_Request {parameters: std::ptr::null()};
                    let mut c_res = C_Response {content_type: 12, body: std::ptr::null_mut()};
                    for i in 0..routes.len() {
                        if req.uri().path() == routes[i] {
                            handlers[i](&c_req, &mut c_res);
                            let body_ptr = unsafe { CStr::from_ptr(c_res.body) };
                            let body_ptr = body_ptr.to_owned();
                            let body_str = match body_ptr.to_str() {
                                Ok(v) => v,
                                Err(_e) => "none",
                            };
                            menssage = String::from(body_str);
                            break;
                        }
                    }
                    async move {
                        Ok::<Response<Body>, Infallible>(Response::new(Body::from(format!("{} in route {}", "Hello World!", menssage))))
                    }
            }))
        }
    });

    let addr = (ip_addr, port).into();
    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}

#[repr(C)]
pub struct C_Request {
    parameters:  * const * const c_char,
}

#[repr(C)]
pub struct C_Response {
    content_type: usize,
    body:  * mut c_char,
}

#[repr(C)]
pub struct C_Server {
    number_routes:  usize,
    routes_arr: [* mut c_char; 100],
    routes_handler: [extern "C" fn(req: * const C_Request, res: * mut C_Response); 100]
}

#[no_mangle]
pub unsafe extern "C" fn add_route(c_server: * mut C_Server, route: * const c_char, route_handler: extern "C" fn(req: * const C_Request, res: * mut C_Response)) {
    let arr_size: usize = (*c_server).number_routes;
    (*c_server).routes_arr[arr_size] = strdup(route);
    (*c_server).routes_handler[arr_size] = route_handler;
    (*c_server).number_routes += 1;
}

#[no_mangle]
pub unsafe extern "C" fn listen_at(c_server: * mut C_Server, ip_addr_ptr: * const u8, port: u16) {
    let mut routes = Vec::new();
    let mut handlers = Vec::new();
    let size: usize = (*c_server).number_routes;
    for i in 0..size {
        let route_ptr = CStr::from_ptr((*c_server).routes_arr[i]);
        let route_ptr = route_ptr.to_owned();
        let route_str = match route_ptr.to_str() {
            Ok(v) => v,
            Err(_e) => "none",
        };
        let handler = (*c_server).routes_handler[i];
        routes.push(String::from(route_str.clone()));
        handlers.push(handler.clone());
    }
    let ip_addr = std::slice::from_raw_parts(ip_addr_ptr, 4);
    listener([ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]], port, routes, handlers);
}
