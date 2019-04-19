use core::sync::atomic::{Ordering, compiler_fence};

use rs64_periph::mips::dcache_hit_writeback_invalidate;
use rs64_periph::si;

use volatile::Volatile;

#[repr(align(64))]
struct ScanPacket {
    values: [u64; 8],
}

const INIT_PACKET: ScanPacket = ScanPacket {
    values: [
        0xff_01_04_01_00000000,
        0,
        0,
        0,
        0xfe_00_00_00_00000000,
        0,
        0,
        1,
    ]
};

impl ScanPacket {
    fn new() -> ScanPacket {
        ScanPacket {
            values: [0u64; 8],
        }
    }
}

pub fn init() {
    unsafe {
        let init_packet_addr = (&INIT_PACKET as *const ScanPacket) as usize;

        si::dma_wait();

        si::write_pif(init_packet_addr);

        si::dma_wait();
    }
}

static mut scan_packet: ScanPacket = ScanPacket {
    values: [0; 8],
};

pub fn scan() -> Option<u32> {
    Some(unsafe {
        let scan_packet_addr = (&mut scan_packet as *mut ScanPacket) as usize;

        // println!("{:08x}", si::mmio_ref().status.read());
        // println!("{:08x}", scan_packet_addr);

        si::read_pif(scan_packet_addr);
        // println!("{:08x}", si::mmio_ref().status.read());

        compiler_fence(Ordering::AcqRel);

        // println!("+{:016x}", scan_packet.values[0]);

        let ptr = 0xbfc0_07c0 as *const [Volatile<u32>;8];
        // println!("-{:08x}{:08x}", (*ptr)[0].read(), (*ptr)[1].read());

        (*ptr)[1].read()
    })
}
