use hwio::{Io, Pio};
use std::io;

fn dump_superio() {
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

    let enter_config = || {
        addr_port().write(0x87);
        addr_port().write(0x87);
    };

    let exit_config = || {
        addr_port().write(0xAA);
    };

    enter_config();

    for ldn in 0..=0x16 {
        println!("Logical Device {:02X}", ldn);
        superio_write(0x07, ldn);
        print!("   ");
        for col in 0..=0xF {
            print!("  {:X}", col);
        }
        println!();
        for row in 0..=0xF {
            print!("{:X}0:", row);
            for col in 0..=0xF {
                let reg = (row << 4) | col;
                if reg == 0xAA {
                    print!(" XX");
                } else {
                    print!(" {:02X}", superio_read(reg));
                }
            }
            println!();
        }
    }

    exit_config();
}

fn dump_hm() {
    let addr_port = || -> Pio<u8> { Pio::<u8>::new(0x295) };

    let data_port = || -> Pio<u8> { Pio::<u8>::new(0x296) };

    let hm_read = |reg: u8| -> u8 {
        addr_port().write(reg);
        data_port().read()
    };

    let hm_write = |reg: u8, value: u8| {
        addr_port().write(reg);
        data_port().write(value);
    };

    for bank in 0..=3 {
        println!("HM Bank {:02X}", bank);
        hm_write(0x4E, 0x80 | bank);
        print!("   ");
        for col in 0..=0xF {
            print!("  {:X}", col);
        }
        println!();
        for row in 0..=0xF {
            print!("{:X}0:", row);
            for col in 0..=0xF {
                let reg = (row << 4) | col;
                print!(" {:02X}", hm_read(reg));
            }
            println!();
        }
    }
}

fn main() {
    assert!(
        unsafe { libc::iopl(3) } == 0,
        "iopl: {}",
        io::Error::last_os_error()
    );

    dump_superio();
    dump_hm();
}
