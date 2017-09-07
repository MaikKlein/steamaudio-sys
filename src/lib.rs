#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn it_works() {
        let ctx = IPLContext {
            logCallback: None,
            allocateCallback: None,
            freeCallback: None,
        };
        unsafe {
            let mut handle: *mut _ = ::std::mem::uninitialized();
            iplCreateComputeDevice(
                ctx,
                IPLComputeDeviceType::IPL_COMPUTEDEVICE_ANY,
                2,
                &mut handle,
            );
        }
    }
}
