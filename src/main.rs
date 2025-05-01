#![no_main]
#![no_std]

use uefi::{guid, prelude::*, println, Error, Guid};
use uefi::boot::install_protocol_interface;
use uefi::proto::unsafe_protocol;

const PROTO_GUID: Guid = guid!("9f01e43e-b2a1-4774-9ea8-a51ffd6d30fc");

#[derive(Debug)]
#[repr(C)]
pub struct MyProtocol {
    pub test_fn: unsafe extern "efiapi" fn(this: *mut Self, input: i32, output: *mut i32) -> Status
}

impl MyProtocol {
    /// test_fn takes an integer i and returns i * i
    pub unsafe extern "efiapi" fn test_fn(_this: *mut Self, input: i32, output: *mut i32) -> Status {
        if output.is_null() {
            return Status::INVALID_PARAMETER;
        }
        unsafe {
            *output = input * input;
        }
        Status::SUCCESS
    }
}

#[unsafe_protocol(PROTO_GUID)]
pub struct MyProtocolInterface(MyProtocol);

impl MyProtocolInterface {
    pub fn test_fn(&mut self, input: i32) -> Result<i32, Error> {
        let mut output: i32 = 0;
        let status = unsafe { (self.0.test_fn)(&mut self.0, input, &mut output) };
        status.to_result_with_val(|| output)
    }
    
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    println!("Hello from UEFI!");

    let my_protocol = MyProtocol {
        test_fn: MyProtocol::test_fn,
    };

    let result = unsafe {
        install_protocol_interface(
            None,
            &PROTO_GUID,
            &my_protocol as *const _ as *const _
        )
    };

    let handle = result.unwrap();

    println!("Protocol installed with handle: {:?}", handle);   // seems to always be Handle(0x7e1b1f98)

    let mut opened_protocol = boot::open_protocol_exclusive::<MyProtocolInterface>(handle).unwrap();

    let input = 5;
    let mut output: i32 = 0;
    let status = unsafe { (opened_protocol.0.test_fn)(&mut opened_protocol.0, input, &mut output) };

    match status {
        Status::SUCCESS => println!("test_fn({}): {}", input, output),
        _ => println!("Failed to call test_fn: {:?}", status),
    }

    Status::SUCCESS
}