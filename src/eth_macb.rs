use crate::macbh::*;
use core::arch::asm;
use log::info;

// clk -> reg
pub const TIMER_CLOCK: u64 = 1000000;

pub const RESET_BASE: u64 = 0;
pub const PRCI_RESETREG_OFFSET: u64 = 0x28;

// PROCMONCFG
pub const PRCI_PROCMONCFG_OFFSET: u64 = 0xF0;
pub const PRCI_PROCMONCFG_CORE_CLOCK_MASK: u64 = 1 << 24;

pub const MACB_IOBASE: u64 = 0x10090000;
pub const PRCI_BASE: u64 = 0x10000000;

/*
 * These buffer sizes must be power of 2 and divisible
 * by RX_BUFFER_MULTIPLE
 */
pub const MACB_RX_BUFFER_SIZE: u64 = 128;
pub const GEM_RX_BUFFER_SIZE: u64 = 2048;
pub const RX_BUFFER_MULTIPLE: u64 = 64;

pub const MACB_RX_RING_SIZE: u64 = 32;
pub const MACB_TX_RING_SIZE: u64 = 16;

pub const MACB_TX_TIMEOUT: u64 = 1000;
pub const MACB_AUTONEG_TIMEOUT: u64 = 5000000;

/// Memory functions that drivers must use
pub trait MemMapping {
    /// Page size (usually 4K)
    const PAGE_SIZE: usize;

    /// Allocate consequent physical memory for DMA;
    /// Return physical address which is page aligned.
    fn dma_alloc_coherent(pages: usize) -> usize;

    /// Deallocate DMA memory
    fn dma_free_coherent(paddr: usize, pages: usize);
}

pub fn open() {
    // device_probe

    // macb_eth_of_to_plat(); check iobase addr

    // ethernet@10090000
    clk_set_defaults(0);

    macb_eth_probe();

    // eth0: ethernet@10090000


    // 准备每次的收发包
    // macb_start, _macb_init
    //macb_start();

    info!("macb loop in open()");
    loop {
        unsafe{ asm!("wfi"); }
    }
}

fn clk_set_defaults(clk_defaults_stage: usize) {
    // ethernet@10090000
    let rate = 125125000;

    sifive_prci_set_rate(rate);
}

fn macb_enable_clk() {
    sifive_prci_enable();

    // sifive_prci_wrpll_recalc_rate, clk_rate: 125125000
}

fn sifive_prci_enable() {
    // sifive_prci_clock_enable
    let value: u32 = 0x80000000;
    let offs = 0x20;
    writev((PRCI_BASE + offs) as *mut u32, value);

    sifive_prci_ethernet_release_reset();
}

fn sifive_prci_set_rate(rate: u64) -> i32 {
    // wrpll_configure_for_rate, Compute the appropriate PLL signal configuration values and store in PLL context @c.
    // parent_rate: 26000000, rate: 125125000

    let value: u32 = 0x206b982;
    let offs = 0x1c;
    writev((PRCI_BASE + offs) as *mut u32, value);

    // microseconds微秒
    let max_pll_lock_us = 70;
    usdelay(max_pll_lock_us);
    0
}

fn macb_eth_probe() {
    let phy_mode = "gmii";

    // is_big_endian ?

    macb_enable_clk();

    macb_eth_initialize();

    // Check that the Ethernet address (MAC) is not 00:00:00:00:00:00,
    // is not a multicast address, and is not FF:FF:FF:FF:FF:FF;
    // Ethernet MAC address: 70:b3:d5:92:fa:99
    let mac: [u8; 6] = [0x70, 0xb3, 0xd5, 0x92, 0xfa, 0x99];
    macb_write_hwaddr(&mac);
}

fn macb_eth_initialize() {
    // rx/tx ring dma
    // rx_buffer_size: 2048

    let id = 0;
    let mut ncfgr: u32 = 0;

    let mut rx_buffer_size = 0;
    if macb_is_gem() {
            info!("macb is GEM");
            rx_buffer_size = GEM_RX_BUFFER_SIZE;
    } else {
            info!("macb is MACB");
            rx_buffer_size = MACB_RX_BUFFER_SIZE;
    }

    // dma alloc

    // todo 为DMA构建环形缓冲区内存
    // dma_alloc_coherent  申请一致性内存，一般为连续物理内存且不cache, 或dcache line对齐

    /*
    let (send_ring_va, send_ring_pa) = P::alloc_dma(P::PAGE_SIZE);
    let (recv_ring_va, recv_ring_pa) = P::alloc_dma(P::PAGE_SIZE);
    let send_ring = unsafe {
        slice::from_raw_parts_mut(
            send_ring_va as *mut DmaDesc,
            P::PAGE_SIZE / size_of::<DmaDesc>(), // 4096/16 = 256 个 dma_desc
        )
    };

    let recv_ring = unsafe {
        slice::from_raw_parts_mut(
            recv_ring_va as *mut DmaDesc,
            P::PAGE_SIZE / size_of::<DmaDesc>(),
        )
    };

    send_ring.fill(DmaDesc {
        desc0: 0,
        desc1: 0,
        desc2: 0,
        desc3: 0,
    });
    recv_ring.fill(DmaDesc {
        desc0: 0,
        desc1: 0,
        desc2: 0,
        desc3: 0,
    });

    let mut send_buffers = Vec::with_capacity(send_ring.len());
    let mut recv_buffers = Vec::with_capacity(recv_ring.len());

    info!("Set a ring desc buffer for TX");
    // Set a ring desc buffer for TX
    for i in 0..send_ring.len() {
        let (buffer_page_va, buffer_page_pa) = P::alloc_dma(P::PAGE_SIZE); // 其实buffer申请2K左右就可以

        // desc1.all |= (1 << 24) Chain mode
        send_ring[i].desc1 |= 1 << 24;

        send_ring[i].desc2 = buffer_page_pa as u32;

        if (i + 1) == send_ring.len() {
            send_ring[i].desc3 = virt_to_phys(&send_ring[0] as *const DmaDesc as usize) as u32;
        } else {
            send_ring[i].desc3 = virt_to_phys(&send_ring[i + 1] as *const DmaDesc as usize) as u32;
        }

        send_buffers.push(buffer_page_va);
    }

    info!("Set a ring desc buffer for RX");
    // Set a ring desc buffer for RX
    for i in 0..recv_ring.len() {
        let (buffer_page_va, buffer_page_pa) = P::alloc_dma(P::PAGE_SIZE);

        recv_ring[i].desc1 |= 1 << 24;
        //recv_ring[i].desc2 = buffer_page_pa as u32;
        if (i + 1) == recv_ring.len() {
            recv_ring[i].desc3 = virt_to_phys(&recv_ring[0] as *const DmaDesc as usize) as u32;
        } else {
            recv_ring[i].desc3 = virt_to_phys(&recv_ring[i + 1] as *const DmaDesc as usize) as u32;
        }

        recv_buffers.push(buffer_page_va);

        // geth_rx_refill, 实际运行refill时却是：priv->rx_clean: 0 ~ 254 ?
        // desc_buf_set(&mut recv_ring[i], buffer_page_pa as u32, MAX_BUF_SZ);
        recv_ring[i].desc1 &= !((1 << 11) - 1);
        recv_ring[i].desc1 |= MAX_BUF_SZ & ((1 << 11) - 1);
        recv_ring[i].desc2 = buffer_page_pa as u32;

        // sync memery, fence指令？

        desc_set_own(&mut recv_ring[i]);
    }

    info!(
        "send_buffers length: {}, recv_buffers length: {}",
        send_buffers.len(),
        recv_buffers.len()
    );
    */

    // Do some basic initialization so that we at least can talk to the PHY
    let pclk_rate = 125125000;
    if macb_is_gem() {
        ncfgr = gem_mdc_clk_div(id, pclk_rate);
        ncfgr |= macb_dbw();
    } else {
        ncfgr = macb_mdc_clk_div(id, pclk_rate);
    }

    info!("macb_eth_initialize to write MACB_NCFGR: {:#x}", ncfgr);
    writev((MACB_IOBASE + MACB_NCFGR) as *mut u32, ncfgr);
}

fn macb_is_gem() -> bool {
    let mid_value: u32 = readv((MACB_IOBASE + MACB_MID) as *const u32);
    let macb_bfext = (mid_value >> MACB_IDNUM_OFFSET) & ((1 << MACB_IDNUM_SIZE) - 1);

    macb_bfext >= 0x2
}

fn macb_write_hwaddr(enetaddr: &[u8; 6]) -> i32 {
      // set hardware address
      let hwaddr_bottom: u32 = (enetaddr[0] as u32) | (enetaddr[1] as u32) << 8 |
                      (enetaddr[2] as u32) << 16 | (enetaddr[3] as u32) << 24;
      writev((MACB_IOBASE + MACB_SA1B) as *mut u32, hwaddr_bottom);

      let hwaddr_top: u16 = (enetaddr[4] as u16) | (enetaddr[5] as u16) << 8;
      writev((MACB_IOBASE + MACB_SA1T) as *mut u16, hwaddr_top);

      info!("macb_write_hwaddr {:#x} {:#x}", hwaddr_top, hwaddr_bottom);

      0
}

pub fn macb_mdio_write(phy_adr: u8, reg: u8, value: u16) {
    let mut netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl |= 1 << MACB_MPE_OFFSET;
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);

    // MACB_BF(name,value) 
    // (((value) & ((1 << MACB_x_SIZE) - 1)) << MACB_x_OFFSET)

    let frame: u64 = ((1 & ((1 << MACB_SOF_SIZE) - 1)) << MACB_SOF_OFFSET)
                    | ((1 & ((1 << MACB_RW_SIZE) - 1)) << MACB_RW_OFFSET)
                    | ((phy_adr as u64 & ((1 << MACB_PHYA_SIZE) - 1)) << MACB_PHYA_OFFSET)
                    | ((reg as u64 & ((1 << MACB_REGA_SIZE) - 1)) << MACB_REGA_OFFSET)
                    | ((2 & ((1 << MACB_CODE_SIZE) - 1)) << MACB_CODE_OFFSET)
                    | ((value as u64 & ((1 << MACB_DATA_SIZE) - 1)) << MACB_DATA_OFFSET);

    writev((MACB_IOBASE + MACB_MAN) as *mut u32, frame as u32);
    while (readv((MACB_IOBASE + MACB_NSR) as *const u32) & (1 << MACB_IDLE_OFFSET)) == 0 {}

    netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl &= !(1 << MACB_MPE_OFFSET);
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);
}

pub fn macb_mdio_read(phy_adr: u8, reg: u8, value: u16) -> u16 {
    let mut netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl |= 1 << MACB_MPE_OFFSET;
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);

    let mut frame: u64 = ((1 & ((1 << MACB_SOF_SIZE) - 1)) << MACB_SOF_OFFSET)
                    | ((2 & ((1 << MACB_RW_SIZE) - 1)) << MACB_RW_OFFSET)
                    | ((phy_adr as u64 & ((1 << MACB_PHYA_SIZE) - 1)) << MACB_PHYA_OFFSET)
                    | ((reg as u64 & ((1 << MACB_REGA_SIZE) - 1)) << MACB_REGA_OFFSET)
                    | ((2 & ((1 << MACB_CODE_SIZE) - 1)) << MACB_CODE_OFFSET);

    writev((MACB_IOBASE + MACB_MAN) as *mut u32, frame as u32);
    while (readv((MACB_IOBASE + MACB_NSR) as *const u32) & (1 << MACB_IDLE_OFFSET)) == 0 {}

    frame = readv((MACB_IOBASE + MACB_MAN) as *const u32) as u64;

    netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl &= !(1 << MACB_MPE_OFFSET);
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);

    ((frame >> MACB_DATA_OFFSET) & ((1 << MACB_DATA_SIZE) - 1)) as u16
}

/*
 * Get the DMA bus width field of the network configuration register that we
 * should program. We find the width from decoding the design configuration
 * register to find the maximum supported data bus width.
 */
fn macb_dbw() -> u32 {
    let dcfg1_value: u32 = readv((MACB_IOBASE + GEM_DCFG1) as *const u32);
    let gem_bfex = (dcfg1_value >> GEM_DBWDEF_OFFSET) & ((1 << GEM_DBWDEF_SIZE) - 1);
    info!("macb_dbw, dcfg1: {:#x}, gem_bfex: {}", dcfg1_value, gem_bfex);
    match gem_bfex {
        4 => ((GEM_DBW128 & ((1 << GEM_DBW_SIZE) - 1)) << GEM_DBW_OFFSET) as u32,
        2 => ((GEM_DBW64 & ((1 << GEM_DBW_SIZE) - 1)) << GEM_DBW_OFFSET) as u32,
        _ => ((GEM_DBW32 & ((1 << GEM_DBW_SIZE) - 1)) << GEM_DBW_OFFSET) as u32,
    }
}

fn macb_mdc_clk_div(id: u32, pclk_rate: u64) -> u32 {
    let mut config: u32 = 0;

    // macb->pclk_rate
    // let macb_hz: u64 = 125125000;
    let macb_hz: u64 = pclk_rate;

    if macb_hz < 20000000 {
        config = ((MACB_CLK_DIV8 & ((1 << MACB_CLK_SIZE) - 1)) << MACB_CLK_OFFSET) as u32;
    } else if macb_hz < 40000000 {
        config = ((MACB_CLK_DIV16 & ((1 << MACB_CLK_SIZE) - 1)) << MACB_CLK_OFFSET) as u32;
    } else if macb_hz < 80000000 {
        config = ((MACB_CLK_DIV32 & ((1 << MACB_CLK_SIZE) - 1)) << MACB_CLK_OFFSET) as u32;
    } else {
        config = ((MACB_CLK_DIV64 & ((1 << MACB_CLK_SIZE) - 1)) << MACB_CLK_OFFSET) as u32;
    }

    config
}

fn gem_mdc_clk_div(id: u32, pclk_rate: u64) -> u32 {
    let mut config: u32 = 0;
    let macb_hz: u64 = pclk_rate;

    if macb_hz < 20000000 {
        config = ((GEM_CLK_DIV8 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 40000000 {
        config = ((GEM_CLK_DIV16 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 80000000 {
        config = ((GEM_CLK_DIV32 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 120000000 {
        config = ((GEM_CLK_DIV48 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 160000000 {
        config = ((GEM_CLK_DIV64 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 240000000 {
        config = ((GEM_CLK_DIV96 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else if macb_hz < 320000000 {
        config = ((GEM_CLK_DIV128 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    } else {
        config = ((GEM_CLK_DIV224 & ((1 << GEM_CLK_SIZE) - 1)) << GEM_CLK_OFFSET) as u32;
    }

    config
}

fn sifive_prci_ethernet_release_reset() {
    /* gemgxl_reset, Release GEMGXL reset */
    // do clk_set_defaults(clock-controller@10000000);
    // sifive reset deassert, write id: 5, regval: 0x2f
    sifive_reset_trigger(5, true);

    /* Procmon => core clock */
    writev(
        (PRCI_BASE + PRCI_PROCMONCFG_OFFSET) as *mut u32,
        PRCI_PROCMONCFG_CORE_CLOCK_MASK as u32,
    );

    /* cltx_reset, Release Chiplink reset */
    // write id: 6, regval: 0x6f
    sifive_reset_trigger(6, true);
}

fn sifive_reset_trigger(id: u32, level: bool) {
    let mut regval: u32 = readv((PRCI_BASE + PRCI_RESETREG_OFFSET) as *const u32);
    if level {
        // Reset deassert
        regval |= 1 << id;
    } else {
        // Reset assert
        regval &= !(1 << id);
    }
    info!("sifive_reset_trigger to write: {:#x}", regval);
    writev((PRCI_BASE + PRCI_RESETREG_OFFSET) as *mut u32, regval);
}

// const MMIO_MTIME: *const u64 = 0x0200_BFF8 as *const u64;
pub fn get_cycle() -> u64 {
    // Load access fault @ 0x200bff8 on fu740
    // unsafe { MMIO_MTIME.read_volatile() }
    let mut cycle: u64 = 0;
    unsafe { asm!("csrr {}, time", out(reg) cycle); }
    cycle
}

// fu740 CPU Timer, Freq = 1000000Hz
// 微秒(us)
pub fn usdelay(us: u64) {
    let mut t1: u64 = get_cycle();
    let t2 = t1 + us * (TIMER_CLOCK / 1000000);

    while t2 >= t1 {
        t1 = get_cycle();
    }
    info!("usdelay, get_cycle: {}", t1);
}

// 毫秒(ms)
#[allow(unused)]
pub fn msdelay(ms: u64) {
    usdelay(ms * 1000);
}

fn readv<T>(src: *const T) -> T {
    unsafe { core::ptr::read_volatile(phys_to_virt(src as usize) as *const T) }
}

fn writev<T>(dst: *mut T, value: T) {
    unsafe {
        core::ptr::write_volatile(phys_to_virt(dst as usize) as *mut T, value);
    }
}

fn phys_to_virt(addr: usize) -> usize {
    addr
}
