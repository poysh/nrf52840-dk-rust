use nrf52840_hal::twim::{Twim, Error, Instance};

pub const DEFAULT_ADDRESS: u8 = 0x61;

pub struct SDC30<T: Instance>(Twim<T>);

impl<T> SDC30<T> where T: Instance {
    
    pub fn init(i2c2: Twim<T>) -> Self {
        SDC30(i2c2)
    }

    pub fn get_firmware_version(&mut self) -> Result<[u8; 2], Error> {
        let command:[u8; 2] = [0xd1, 0x00];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let major = u8::from_be(rd_buffer[0]);
        let minor = u8::from_be(rd_buffer[1]);

        Ok([major, minor])
    }
}