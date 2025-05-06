#![no_main]
#![no_std]
#![feature(naked_functions)]

use core::arch::naked_asm;
use uefi::runtime::{VariableVendor, set_variable, VariableAttributes};
use uefi::{Guid, guid, prelude::*, println};

const SERVICE_GUID: Guid = guid!("9f01e43e-b2a1-4774-9ea8-a51ffd6d30fc");
/// test_fn takes an integer i and returns i * i
#[naked]
pub unsafe extern "efiapi" fn test_fn() {
    naked_asm!(
        "mov rax, rdi",     // rdi contains the u64 input argument
        "mul rax",          // multiply rax by rax, result stored in rdx:rax (high:low)
        "ret",              // return value in rax
    );
    // (input * input) as usize
    // if output.is_null() {
    //     return Status::INVALID_PARAMETER;
    // }
    // unsafe {
    //     *output = input * input;
    // }
    // Status::SUCCESS
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    println!("Hello from UEFI!");

    let service_name = cstr16!("MyRuntimeService");

    let test_fn_bytes = (test_fn as usize).to_ne_bytes();

    println!("Address of test_fn: {:#x}", test_fn as usize);

    let result = set_variable(
        service_name,
        &VariableVendor(SERVICE_GUID),
        VariableAttributes::BOOTSERVICE_ACCESS | VariableAttributes::RUNTIME_ACCESS,
        &test_fn_bytes,
    );

    match result {
        Ok(_) => println!("Successfully set variable"),
        Err(e) => println!("Failed to set variable: {:?}", e),
    }

    Status::SUCCESS
}
