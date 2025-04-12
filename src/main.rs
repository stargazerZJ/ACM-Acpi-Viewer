#![no_main]
#![no_std]

use core::slice;

use uefi::{prelude::*, println, table::cfg::ACPI2_GUID, table::system_table_raw};

// Define the RSDP structure (Root System Description Pointer)
#[derive(Debug)]
#[repr(C, packed)]
struct Rsdp {
    signature: [u8; 8], // "RSD PTR "
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8, // For ACPI 2.0+, this is >= 2
    rsdt_address: u32,
    // Fields below are only valid for ACPI 2.0+
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

// Define the common ACPI table header structure
#[repr(C, packed)]
struct AcpiTableHeader {
    signature: [u8; 4],     // Table identifier
    length: u32,            // Length of the table including header
    revision: u8,           // Table revision
    checksum: u8,           // Checksum for the entire table
    oem_id: [u8; 6],        // OEM identifier
    oem_table_id: [u8; 8],  // OEM table identifier
    oem_revision: u32,      // OEM revision
    creator_id: [u8; 4],    // Vendor ID
    creator_revision: u32,  // Vendor revision
}

// Define the XSDT structure (Extended System Description Table)
#[repr(C, packed)]
struct Xsdt {
    header: AcpiTableHeader,
    // Followed by array of 64-bit physical addresses
    // entries: [u64; N] - dynamically sized
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let system_table = system_table_raw().expect("System table should be available");
    let system_table = unsafe { system_table.as_ref() };
    println!("Hello world from uefi-rust!");
    println!("UEFI Version: {}", uefi::system::uefi_revision());
    println!("System Table: {:?}", system_table);

    // Search for ACPI tables
    let config_tables = system_table.configuration_table;
    let config_tables = unsafe {
        slice::from_raw_parts(
            config_tables,
            system_table.number_of_configuration_table_entries,
        )
    };
    println!("Searching for ACPI tables...");
    println!("Number of configuration tables: {}", config_tables.len());

    // Use filter and map to find the ACPI 2.0+ entry
    let acpi2_table = config_tables
        .iter()
        .find(|entry| entry.vendor_guid == ACPI2_GUID)
        .map(|entry| entry.vendor_table);

    let rsdp_ptr = acpi2_table.unwrap();

    println!("Found ACPI 2.0+ table at address: {:p}", rsdp_ptr);

    // Cast the vendor_table pointer to RSDP
    let rsdp = unsafe { &*(rsdp_ptr as *const Rsdp) };

    // Check if the RSDP revision is less than 2
    if rsdp.revision < 2 {
        println!("ERROR: RSDP version less than 2 is not supported.");
        return Status::UNSUPPORTED;
    }

    // Print RSDP info
    println!("RSDP Information:");
    println!("  Physical Address: {:#018x}", rsdp_ptr as u64);
    println!("  Length: {}", rsdp.length as usize);

    // Convert OEM ID bytes to string
    let oem_id = str::from_utf8(&rsdp.oem_id).unwrap_or("Invalid OEM ID");
    println!("  OEM ID: {}", oem_id);
    println!("  Checksum: {:#04x}", rsdp.checksum);
    println!("  Revision: {}", rsdp.revision);
    println!("  XSDT Address: {:#018x}", rsdp.xsdt_address as usize);
    println!();

    // Access the XSDT
    let xsdt = unsafe { &*(rsdp.xsdt_address as *const Xsdt) };

    // Print XSDT info
    println!("XSDT Information:");
    println!("  Physical Address: {:#018x}", rsdp.xsdt_address as usize);
    println!("  Length: {}", xsdt.header.length as usize);

    let xsdt_oem_id = str::from_utf8(&xsdt.header.oem_id).unwrap_or("Invalid OEM ID");
    println!("  OEM ID: {}", xsdt_oem_id);
    println!("  Checksum: {:#04x}", xsdt.header.checksum);

    // Calculate number of entries in XSDT
    let num_entries = (xsdt.header.length as usize - core::mem::size_of::<AcpiTableHeader>()) / 8;
    println!("  Number of Table Entries: {}", num_entries);
    println!();

    // Get pointer to the first entry in XSDT
    let entries_base_addr = rsdp.xsdt_address as usize + core::mem::size_of::<AcpiTableHeader>();

    // Iterate through each table entry
    println!("ACPI Tables:");
    for i in 0..num_entries {
        // Calculate address of the current entry
        let entry_addr = entries_base_addr + (i * 8);

        // Read the entry using unaligned access
        let table_addr = unsafe {
            core::ptr::read_unaligned(entry_addr as *const u64)
        };

        // Access the table header
        let table_header = unsafe { &*(table_addr as *const AcpiTableHeader) };

        // Extract the 4-byte signature as a string
        let sig = str::from_utf8(&table_header.signature).unwrap_or_else(|_| "????");

        // Extract the OEM ID
        let table_oem_id = str::from_utf8(&table_header.oem_id).unwrap_or_else(|_| "Invalid OEM ID");

        println!("Table #{}:", i + 1);
        println!("  Signature: {}", sig);
        println!("  Physical Address: {:#018x}", table_addr);
        println!("  Length: {}", table_header.length as usize);
        println!("  OEM ID: {}", table_oem_id);
        println!("  Checksum: {:#04x}", table_header.checksum);
        println!();
    }

    Status::SUCCESS
}
