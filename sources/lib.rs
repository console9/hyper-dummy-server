

use ::hyper_simple_server as hss;


pub use hss::{
		MainResult,
	};




pub fn main () -> MainResult {
	
	
	const DATE_VALUE : hss::HeaderValue = hss::HeaderValue::from_static ("");
	
	fn _response_builder () -> hss::Response<hss::Body> {
			let _body = hss::Body::from ("hello world!\n");
			let mut _response = hss::Response::new (_body);
			_response.headers_mut () .insert (hss::consts::DATE, DATE_VALUE);
			_response
		}
	
	
	if true {
		
		let _handler = |_request : hss::Request<hss::Body>| {
				let _response = _response_builder ();
				Ok (_response)
			};
		
		let _handler = hss::HandlerFnSync::from (_handler);
		let _handler = ::std::sync::Arc::new (_handler);
		
		return hss::main_with_handler (_handler, None, None);
		
	} else {
		
		let _service = |_request : hss::Request<hss::Body>| {
				let _response = _response_builder ();
				async {
					Ok (_response)
				}
			};
		
		return hss::main_with_service_fn (_service, None, None);
	}
}

