use hwio::{Io, Pio};
use std::io;

fn main() {
    assert!(
        unsafe { libc::iopl(3) } == 0,
        "iopl: {}",
        io::Error::last_os_error()
    );

    let addr_port = || -> Pio<u8> { Pio::<u8>::new(0x2E) };

    let data_port = || -> Pio<u8> { Pio::<u8>::new(0x2F) };

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

    {
        for (address, name) in &[
            (0x1600, "GCR"),
            (0x16F0, "GCR1"),
            (0x16F1, "GCR2"),
            (0x16F2, "GCR3"),
            (0x16F3, "GCR4"),
            (0x16F4, "GCR5"),
            (0x16F5, "GCR6"),
            (0x16F6, "GCR7"),
            (0x16F7, "GCR8"),
            (0x16F8, "GCR9"),
            (0x16F9, "GCR10"),
            (0x16FA, "GCR11"),
            (0x16FB, "GCR12"),
            (0x16FC, "GCR13"),
            (0x16FD, "GCR14"),
            (0x16FE, "GCR15"),
            (0x16E0, "GCR16"),
            (0x16E1, "GCR17"),
            (0x16E2, "GCR18"),
            //TODO: only do these on IT5570 and IT5571
            (0x16E4, "GCR19"),
            (0x16E5, "GCR20"),
            (0x16E6, "GCR21"),
            (0x16E7, "GCR22"),
            (0x16E8, "GCR23"),
        ] {
            println!("{}: 0x{:02X}", name, i2ec_read(*address));
        }
    }

    /*
    for sram in 0x0000 .. 0x1000 {
        println!("0x{:04X} = 0x{:02X}", sram, i2ec_read(sram));
    }
    */
}
