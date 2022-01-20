use core::time;
use std::{fmt, env};

use rusb::{Device, DeviceHandle, GlobalContext};

pub mod hid;

const HID_CLASS_CODE: u8 = 3;

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub fn get_hid_descriptor_bytes(device: &Device<rusb::GlobalContext>) -> Result<Vec<u8>, Error> {
    let interface_number = get_hid_interface_number(device)?;
    let mut device_handle = get_device_handle(device)?;

    claim_interface(&mut device_handle, interface_number)?;

    let hid_desciprtor_bytes = read_hid_descriptor(&device_handle, interface_number)?;

    release_interface(&mut device_handle, interface_number)?;

    Ok(hid_desciprtor_bytes)
}

fn get_hid_interface_number(device: &Device<rusb::GlobalContext>) -> Result<u8, Error> {
    // TODO: Handle error
    let config = device.config_descriptor(0).unwrap();

    for interface in config.interfaces() {
        for interface_desc in interface.descriptors() {
            if interface_desc.class_code() == HID_CLASS_CODE {
                return Ok(interface_desc.interface_number());
            }
        }
    }

    Err(Error("HID interface not found".into()))
}

fn get_device_handle(device: &Device<rusb::GlobalContext>) -> Result<DeviceHandle<GlobalContext>, Error> {
    let device_handle_res = device.open();
    match device_handle_res {
        Ok(device_handle) => Ok(device_handle),
        Err(error) => return Err(Error(error.to_string())),
    }
}

fn claim_interface(device_handle: &mut DeviceHandle<rusb::GlobalContext>, interface_number: u8) -> Result<(), Error> {
    match env::consts::OS {
        "linux" => {
            let detach_res = device_handle.set_auto_detach_kernel_driver(true);
            
            match detach_res {
                Ok(_) => {},
                Err(error) => {
                    match error {
                        rusb::Error::NotSupported => { /* No problem, detach not needed if not supported */ },
                        _ => return Err(Error(error.to_string())),
                    }
                },
            }
            
            let claim_res = device_handle.claim_interface(interface_number);
            match claim_res {
                Ok(_) => Ok(()),
                Err(error) => Err(Error(error.to_string())),
            }
        },
        _ => Ok(())
    }
}

fn release_interface(device_handle: &mut DeviceHandle<rusb::GlobalContext>, interface_number: u8) -> Result<(), Error> {
    match env::consts::OS {
        "linux" => {
            match device_handle.release_interface(interface_number) {
                Ok(_) => Ok(()),
                Err(error) => Err(Error(error.to_string())),
            }
        },
        _ => Ok(())
    }
}

fn read_hid_descriptor(
    device_handle: &rusb::DeviceHandle<rusb::GlobalContext>,
    interface_number: u8,
) -> Result<Vec<u8>, Error> {
    let request_type = rusb::request_type(rusb::Direction::In, rusb::RequestType::Standard, rusb::Recipient::Interface);
    let request = rusb::constants::LIBUSB_REQUEST_GET_DESCRIPTOR;
    let request_value = (rusb::constants::LIBUSB_DT_REPORT as u16) << 8;
    let request_index: u16 = interface_number.into();
    let mut output_buffer = [0u8; 4096];
    let timeout = time::Duration::from_secs(1);

    let c = device_handle.read_control(
        request_type,
        request,
        request_value,
        request_index,
        &mut output_buffer,
        timeout,
    );

    match c {
        Ok(result_size) => Ok(output_buffer[0..result_size].to_vec()),
        Err(error) => Err(Error(error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
