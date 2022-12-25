use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum DeviceError {
    #[error("could not initialize device")]
    InitializationFailed,
}

pub struct Device {
    raw: metal::Device,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let raw = metal::Device::system_default().ok_or(DeviceError::InitializationFailed)?;
        Ok(Self { raw })
    }

    pub fn raw(&self) -> &metal::Device {
        &self.raw
    }
}
