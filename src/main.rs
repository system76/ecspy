use hwio::{Io, Pio};
use std::io;

fn main() {
    assert!(unsafe { libc::iopl(3) } == 0, "iopl: {}", io::Error::last_os_error());

    let addr_port = || -> Pio<u8> {
        Pio::<u8>::new(0x2E)
    };

    let data_port = || -> Pio<u8> {
        Pio::<u8>::new(0x2F)
    };

    let superio_read = |reg: u8| -> u8 {
        addr_port().write(reg);
        data_port().read()
    };

    let superio_write = |reg: u8, value: u8| {
        addr_port().write(reg);
        data_port().write(value);
    };

    let d2_read = |reg: u8| -> u8 {
        superio_write(0x2E, reg);
        superio_read(0x2F)
    };

    let d2_write = |reg: u8, value: u8| {
        superio_write(0x2E, reg);
        superio_write(0x2F, value);
    };

    let i2ec_read = |addr: u16| -> u8 {
        d2_write(0x11, (addr >> 8) as u8);
        d2_write(0x10, addr as u8);
        d2_read(0x12)
    };

    #[allow(unused_variables)]
    let i2ec_write = |addr: u16, value: u8| {
        d2_write(0x11, (addr >> 8) as u8);
        d2_write(0x10, addr as u8);
        d2_write(0x12, value);
    };

    println!(
        "id {:>02X}{:>02X} rev {}",
        i2ec_read(0x2000),
        i2ec_read(0x2001),
        i2ec_read(0x2002)
    );

    for bank in b'A'..=b'J' {
        let i = (bank - b'A') as u16;
        let data = i2ec_read(0x1601 + i);
        let mirror = i2ec_read(0x1661 + i);
        let pot = i2ec_read(0x1671 + i);
        for pin in 0..8 {
            let ctrl = i2ec_read(0x1610 + i * 8 + pin);
            println!(
                "{}{}: data {} mirror {} pot {} control {:02X}",
                bank as char,
                pin,
                (data >> pin) & 1,
                (mirror >> pin) & 1,
                (pot >> pin) & 1,
                ctrl,
            )
        }
    }

    {
        let bank = b'M';
        let data = i2ec_read(0x160D);
        let mirror = i2ec_read(0x166D);
        for pin in 0..8 {
            let ctrl = i2ec_read(0x16A0 + pin);
            println!(
                "{}{}: data {} mirror {} control {:02X}",
                bank as char,
                pin,
                (data >> pin) & 1,
                (mirror >> pin) & 1,
                ctrl,
            )
        }
    }

    /*
    for sram in 0x0000 .. 0x1000 {
        println!("0x{:04X} = 0x{:02X}", sram, i2ec_read(sram));
    }
    */
}
