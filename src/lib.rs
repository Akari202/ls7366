#![cfg(no_std)]

use self::embedded_hal::{digital::v2::OutputPin, blocking};
extern crate embedded_hal;
use core::panic;

pub const NQUAD: u8 = 0x00;
pub const QUADRX1: u8 = 0x01;
pub const QUADRX2: u8 = 0x02;
pub const QUADRX4: u8 = 0x03;

pub const FREE_RUN: u8 = 0x00;
pub const SINGLE_CYCLE: u8 = 0x04;
pub const RANGE_LIMIT: u8 = 0x08;
pub const MODULO_N: u8 = 0x0C;

pub const DISABLE_INDX: u8 = 0x00;
pub const INDX_LOADC: u8 = 0x10;
pub const INDX_RESETC: u8 = 0x20;
pub const INDX_LOADO: u8 = 0x30;
pub const ASYNCH_INDX: u8 = 0x00;
pub const SYNCH_INDX: u8 = 0x40;

pub const FILTER_1: u8 = 0x00;
pub const FILTER_2: u8 = 0x80;

pub const NO_FLAGS: u8 = 0x00;
pub const IDX_FLAG: u8 = 0x10;
pub const CMP_FLAG: u8 = 0x20;
pub const BW_FLAG: u8 = 0x40;
pub const CY_FLAG: u8 = 0x80;

pub const EN_CNTR: u8 = 0x00;
pub const DIS_CNTR: u8 = 0x04;

pub const BYTE_4: u8 = 0x00;
pub const BYTE_3: u8 = 0x01;
pub const BYTE_2: u8 = 0x02;
pub const BYTE_1: u8 = 0x03;

pub struct LS7366<SPI, CSPIN> {
    spi_bus: SPI,
    chip_select: CSPIN,
    datawidth: u8
}

impl<SPI, CSPIN> LS7366<SPI, CSPIN>
where
    SPI: blocking::spi::Transfer<u8>, CSPIN: OutputPin
{
    pub fn new(spi_bus: SPI, chip_select: CSPIN) -> Self {
        let datawidth: u8 = 4;
        Self { spi_bus, chip_select, datawidth }
    }

    pub fn clear_mode_register_0(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x08]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn clear_mode_register_1(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x10]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn clear_counter(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x20]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn clear_status_register(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x30]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn read_mode_register_0(&mut self) -> u8 {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x48]).unwrap_or_else(|_| panic!());
        let data: u8 = self.spi_bus.transfer(&mut [0x00]).unwrap_or_else(|_| panic!())[0];
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        data
    }

    pub fn read_mode_register_1(&mut self) -> u8 {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x50]).unwrap_or_else(|_| panic!());
        let data: u8 = self.spi_bus.transfer(&mut [0x00]).unwrap_or_else(|_| panic!())[0];
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        data
    }

    pub fn read_counter(&mut self) -> u32 {
        let mut count: u32 = 0;
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x60]).unwrap_or_else(|_| panic!()); 
        for _ in 0..self.datawidth {
            let data: u32 = self.spi_bus.transfer(&mut [0x00]).unwrap_or_else(|_| panic!())[0] as u32; 
            count = (count << 8) | data;
        }
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        count
    }

    pub fn read_otr(&mut self) -> u32 {
        let mut count: u32 = 0;
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x68]).unwrap_or_else(|_| panic!()); 
        for _ in 0..self.datawidth {
            let data: u32 = self.spi_bus.transfer(&mut [0x00]).unwrap_or_else(|_| panic!())[0] as u32; 
            count = (count << 8) | data;
        } 
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        count
    }
    
    pub fn read_status_register(&mut self) -> u8 {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x70]).unwrap_or_else(|_| panic!()); 
        let data: u8 = self.spi_bus.transfer(&mut [0x00]).unwrap_or_else(|_| panic!())[0]; 
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        data
    }

    pub fn write_mode_register_0(&mut self, value: u8) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x88]).unwrap_or_else(|_| panic!()); 
        self.spi_bus.transfer(&mut [value]).unwrap_or_else(|_| panic!()); 
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn write_mode_register_1(&mut self, value: u8) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x90]).unwrap_or_else(|_| panic!()); 
        self.spi_bus.transfer(&mut [value]).unwrap_or_else(|_| panic!()); 
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
        self.datawidth = 0x04 - (0x03 & value);
    }

    pub fn write_data_register(&mut self, value: u32) {
        let mut value_to_write: u32;
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0x98]).unwrap_or_else(|_| panic!()); 
        for i in 0..self.datawidth {
            value_to_write = value >> (8 * (self.datawidth - 1 - i));
            self.spi_bus.transfer(&mut [value_to_write as u8]).unwrap_or_else(|_| panic!());
        }
        self.chip_select.set_high().unwrap_or_else(|_| panic!()); 
    }

    pub fn load_counter(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0xE0]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn load_otr(&mut self) {
        self.chip_select.set_low().unwrap_or_else(|_| panic!());
        self.spi_bus.transfer(&mut [0xE8]).unwrap_or_else(|_| panic!());
        self.chip_select.set_high().unwrap_or_else(|_| panic!());
    }

    pub fn left_extend_msb(&mut self, value: u32) -> u32 {
        if self.datawidth == 4 {
            value
        } else {
            let msb: u32 = (value >> (self.datawidth * 8 - 1)) & 0x0001;
            if msb == 0 {
                value
            } else {
                match self.datawidth {
                    1 => return 0xFFFFFF00 | value,
                    2 => return 0xFFFF0000 | value,
                    3 => return 0xFF000000 | value,
                    _ => panic!()
                }
            }
        }
    }
}