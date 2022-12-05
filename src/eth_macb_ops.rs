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

pub const HW_DMA_CAP_32B: u64 = 0;
pub const HW_DMA_CAP_64B: u64 = 1;
pub const DMA_DESC_SIZE: u64 = 16;

pub const RXBUF_FRMLEN_MASK: u64 = 0x00000fff;
pub const TXBUF_FRMLEN_MASK: u64 = 0x000007ff;

const MII: usize = 2;
const GMII: usize = 3;
const RMII: usize = 7;
const RGMII: usize = 8;

const hw_dma_cap: usize = 0;
static mut phy_addr: u16 = 0;

#[derive(Debug)]
#[repr(C)]
pub struct DmaDesc {
    // size: 64 bit ?
    addr: u32,
    ctrl: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct DmaDesc64 {
    addrh: u32,
    unused: u32,
}

pub enum PhyInterfaceMode {
    MII = 0,
    GMII = 1,
    SGMII = 2,
    SGMII_2500 = 3,
    QSGMII = 4,
    TBI = 5,
    RMII = 6,
    RGMII = 7,
    RGMII_ID = 8,
    RGMII_RXID = 9,
    RGMII_TXID = 10,
    RTBI = 11,
    1000BASEX = 12,
    2500BASEX = 13,
    XGMII = 14,
    XAUI = 15,
    RXAUI = 16,
    SFI = 17,
    INTERNAL = 18,
    25G_AUI = 19,
    XLAUI = 20,
    CAUI2 = 21,
    CAUI4 = 22,
    NCSI = 23,
    10GBASER = 24,
    USXGMII = 25,
    NONE = 26, /* Must be last */

    COUNT = 27,
}

struct macb_config {
    dma_burst_length: u32,
    hw_dma_cap: u32,
    caps: u32,
    clk_init: usize, // fn clk_init

    usrio_mii: u32,
    usrio_rmii: u32,
    usrio_rgmii: u32,
    usrio_clken: u32,
}

fn macb_start() {
    let rx_buffer_size = GEM_RX_BUFFER_SIZE;

    let config = macb_config {
        dma_burst_length: 0,
        hw_dma_cap: 0,
        caps: 0,
        clk_init: 0,

        usrio_mii: 0,
        usrio_rmii: 0,
        usrio_rgmii: 0,
        usrio_clken: 0,
    };

    // todo 为DMA构建环形缓冲区内存
    // dma_alloc_coherent  申请一致性内存，一般为连续物理内存且不cache, 或dcache line对齐

    let alloc_tx_ring_pages =
        ((MACB_TX_RING_SIZE * DMA_DESC_SIZE) + (MEM::PAGE_SIZE - 1)) / MEM::PAGE_SIZE;
    let alloc_rx_ring_pages =
        ((MACB_RX_RING_SIZE * DMA_DESC_SIZE) + (MEM::PAGE_SIZE - 1)) / MEM::PAGE_SIZE;
    let tx_ring_dma = MEM::dma_alloc_coherent(alloc_tx_ring_pages);
    let rx_ring_dma = MEM::dma_alloc_coherent(alloc_rx_ring_pages);

    let tx_ring = unsafe {
        slice::from_raw_parts_mut(
            tx_ring_dma as *mut DmaDesc,
            MACB_TX_RING_SIZE * DMA_DESC_SIZE / size_of::<DmaDesc>(), // 4096/16 = 256 个 dma_desc ?
        )
    };

    let rx_ring = unsafe {
        slice::from_raw_parts_mut(
            rx_ring_dma as *mut DmaDesc,
            MACB_RX_RING_SIZE * DMA_DESC_SIZE / size_of::<DmaDesc>(),
        )
    };

    let mut send_buffers = Vec::with_capacity(tx_ring.len());
    let mut recv_buffers = Vec::with_capacity(rx_ring.len());

    // 一起申请所有RX内存
    let alloc_rx_buffer_pages =
        ((MACB_RX_RING_SIZE * rx_buffer_size) + (MEM::PAGE_SIZE - 1)) / MEM::PAGE_SIZE;
    let rx_buffer_dma = MEM::dma_alloc_coherent(alloc_rx_buffer_pages);

    info!("Set ring desc buffer for RX");
    let count = 0;
    let paddr: u64 = rx_buffer_dma;
    for i in 0..MACB_RX_RING_SIZE {
        if i == MACB_RX_RING_SIZE - 1 {
            paddr |= 1 << MACB_RX_WRAP_OFFSET;
        }

        if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
            count = i * 2;
            rx_ring[count + 1].addr = upper_32_bits(paddr); // Fill DmaDesc64.addrh
        } else {
            count = i;
        }
        rx_ring[count].ctrl = 0;
        rx_ring[count].addr = lower_32_bits(paddr);

        recv_buffers.push(phys_to_virt(paddr));
        paddr += rx_buffer_size;

        // sync memery, fence指令？
    }
    flush_dcache_range(); // RX dma ring and buffer

    info!("Set ring desc buffer for TX");
    paddr = 0;
    for i in 0..MACB_TX_RING_SIZE {
        if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
            count = i * 2;
            tx_ring[count + 1].addr = upper_32_bits(paddr); // Fill DmaDesc64.addrh
        } else {
            count = i;
        }
        tx_ring[count].addr = lower_32_bits(paddr);

        if i == MACB_TX_RING_SIZE - 1 {
            tx_ring[count].ctrl = (1 << MACB_TX_USED_OFFSET) | (1 << MACB_TX_WRAP_OFFSET);
        } else {
            tx_ring[count].ctrl = (1 << MACB_TX_USED_OFFSET);
        }

        send_buffers.push(phys_to_virt(paddr));
    }
    flush_dcache_range(); // TX dma ring

    info!(
        "send_buffers length: {}, recv_buffers length: {}",
        send_buffers.len(),
        recv_buffers.len()
    );

    /*
     macb->rx_tail = 0;
     macb->tx_head = 0;
     macb->tx_tail = 0;
     macb->next_rx_tail = 0;
    */

    writev(
        (MACB_IOBASE + MACB_RBQP) as *mut u32,
        lower_32_bits(rx_ring_dma),
    );
    writev(
        (MACB_IOBASE + MACB_TBQP) as *mut u32,
        lower_32_bits(tx_ring_dma),
    );
    if config.hw_dma_cap & HW_DMA_CAP_64B != 0 {
        writev(
            (MACB_IOBASE + MACB_RBQPH) as *mut u32,
            upper_32_bits(rx_ring_dma),
        );
        writev(
            (MACB_IOBASE + MACB_TBQPH) as *mut u32,
            upper_32_bits(tx_ring_dma),
        );
    }

    let val: u32 = 0;
    if macb_is_gem() {
        // Initialize DMA properties
        gmac_configure_dma(config);
        // Check the multi queue and initialize the queue for tx
        gmac_init_multi_queues(config);

        if (phy_interface == RGMII)
            || (phy_interface == RGMII_ID)
            || (phy_interface == RGMII_RXID)
            || (phy_interface == RGMII_TXID)
        {
            val = config.usrio_rgmii;
        } else if phy_interface == RMII {
            val = config.usrio_rmii;
        } else if phy_interface == MII {
            val = config.usrio_mii;
        }

        if (config.caps & MACB_CAPS_USRIO_HAS_CLKEN) != 0 {
            val |= config.usrio_clken;
        }

        writev((MACB_IOBASE + GEM_USRIO) as *mut u32, val);

        if phy_interface == SGMII {
            let ncfgr: u32 = readv((MACB_IOBASE + MACB_NCFGR) as *const u32);
            ncfgr |= (1 << GEM_SGMIIEN_OFFSET) | (1 << GEM_PCSSEL_OFFSET);
            info!("Write MACB_NCFGR: %#x when SGMII", ncfgr);
            writev((MACB_IOBASE + MACB_NCFGR) as *mut u32, ncfgr);
        }
    } else {
        if phy_interface == RMII {
            writev((MACB_IOBASE + MACB_USRIO) as *mut u32, 0);
        } else {
            writev((MACB_IOBASE + MACB_USRIO) as *mut u32, config.usrio_mii);
        }
    }

    macb_phy_init();

    // Enable TX and RX
}

fn macb_phy_init() -> i32 {

    arch_get_mdio_control();

    // phy config

    // Auto-detect phy_addr
    macb_phy_find();

    // Check if the PHY is up to snuff...
    let phy_id: u16 = macb_mdio_read(phy_addr, MII_PHYSID1);
    if phy_id == 0xffff {
        error!("No PHY present");
        return -1;
    }

    phy_connect_dev();


}

fn macb_phy_find() -> i32 {
    //let mut phy_addr: u16 = 0;

    let mut phy_id: u16 = phy_id = macb_mdio_read(phy_addr, MII_PHYSID1);
    if phy_id != 0xffff {
        info("PHY present at {}", phy_addr);
        return 0;
    }
    // Search for PHY...
    for i in 0..32 {
        phy_addr = i;
        phy_id = macb_mdio_read(phy_addr, MII_PHYSID1);
        if phy_id != 0xffff {
            info("PHY present at {}", i);
            return 0;
        }
    }

    // PHY isn't up to snuff
    error!("PHY not found");
    return -1;
}

fn phy_connect_dev(addr: u32, interface: PhyInterfaceMode) {
    let phydev_addr = 0; let phydev_flags = 0; // ?

    let phydev = 0xff;
    let mask: u32 = if (addr >= 0) { 1 << addr } else { 0xffffffff };
    /*
    if phydev == 0 {
        phydev = phy_find_by_mask(bus, mask, interface);
    }
    */
    if phydev != 0 {
    /* Soft Reset the PHY */
    phy_reset(phydev_addr, phydev_flags);
    info!("Ethernet connected to PHY");

    } else {
        error!("Could not get PHY for %s: addr %d\n", addr);
    }
}

fn phy_reset(phydev_addr: u32, phydev_flags: u32) -> i32 {
    let mut timeout = 500;
    let devad = MDIO_DEVAD_NONE;

    if phydev_flags & PHY_FLAG_BROKEN_RESET {
        info!("PHY soft reset not supported");
        return 0;
    }

    macb_mdio_write(phydev_addr, MII_BMCR, BMCR_RESET);
    /*
     * Poll the control register for the reset bit to go to 0 (it is
     * auto-clearing).  This should happen within 0.5 seconds per the
     * IEEE spec.
     */
    let mut reg: u16 = macb_mdio_read(phydev_addr, MII_BMCR);
    while ((reg & BMCR_RESET) != 0) && (timeout != 0) {
        timeout -= 1;
        reg = macb_mdio_read(phydev_addr, MII_BMCR);
        if reg < 0 {
            error!("PHY status read failed");
            return -1;
        }
        usdelay(1000);
    }
    if (reg & BMCR_RESET) != 0 {
        error!("PHY reset timed out");
        return -1;
    }
    0
}

fn phy_config() {
    // Microsemi VSC8541 PHY driver config fn: vsc8541_config()
    


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

pub fn macb_mdio_read(phy_adr: u8, reg: u8) -> u16 {
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

fn macb_sifive_clk_init(rate: u64) {
    /*
     * SiFive GEMGXL TX clock operation mode:
     *
     * 0 = GMII mode. Use 125 MHz gemgxlclk from PRCI in TX logic
     *     and output clock on GMII output signal GTX_CLK
     * 
     * 1 = MII mode. Use MII input signal TX_CLK in TX logic
     */
     let mode: u32 = if rate == 125000000 {0} else {1};
     writev(GEMGXL_BASE as *mut u32, mode);
}

fn macb_linkspd_cb(speed: u32) -> i32 {
    let mut rate: u64 = 0;
    match speed {
        10 => rate = 2500000,
        100 => rate = 25000000,
        1000 => rate = 125000000,
        _ => return 0, // does not change anything
    }

    //clk_init
    macb_sifive_clk_init(rate);
    0
}
fn mii_nway_result(negotiated: u32) -> u32 {
    let ret: u32 = 0;
    if (negotiated & LPA_100FULL) != 0 {
            ret = LPA_100FULL;
    } else if (negotiated & LPA_100BASE4) != 0 {
            ret = LPA_100BASE4;
    } else if (negotiated & LPA_100HALF) != 0 {
            ret = LPA_100HALF;
    } else if (negotiated & LPA_10FULL) != 0 {
            ret = LPA_10FULL;
    } else {
            ret = LPA_10HALF;
    }

    ret
}

fn gmac_configure_dma(config: macb_config) -> i32 {
    let GEM_BF = |gem_offset, gem_size, value| (value & ((1 << gem_size) - 1)) << gem_offset;
    let GEM_BFINS = |gem_offset, gem_size, value, old| {
        ((old & !(((1 << gem_size) - 1) << gem_offset)) | GEM_BF(gem_offset, gem_size, value))
    };
    let GEM_BIT = |offset| 1 << offset;

    let buffer_size: u32 = rx_buffer_size / RX_BUFFER_MULTIPLE;
    let dmacfg: u32 = readv((MACB_IOBASE + GEM_DMACFG) as *const u32)
        & !GEM_BF(GEM_RXBS_OFFSET, GEM_RXBS_SIZE, -1);
    dmacfg |= GEM_BF(GEM_RXBS_OFFSET, GEM_RXBS_SIZE, buffer_size);

    if config.dma_burst_length != 0 {
        dmacfg = GEM_BFINS(
            GEM_FBLDO_OFFSET,
            GEM_FBLDO_SIZE,
            config.dma_burst_length,
            dmacfg,
        );
    }

    dmacfg |= GEM_BIT(GEM_TXPBMS_OFFSET) | GEM_BF(GEM_RXBMS_OFFSET, GEM_RXBMS_SIZE, -1);
    dmacfg &= !GEM_BIT(GEM_ENDIA_PKT_OFFSET);

    if is_big_endian() {
        dmacfg |= GEM_BIT(GEM_ENDIA_DESC_OFFSET);
    } else {
        dmacfg &= !GEM_BIT(GEM_ENDIA_DESC_OFFSET);
    }

    dmacfg &= !GEM_BIT(GEM_ADDR64_OFFSET);
    if config.hw_dma_cap & HW_DMA_CAP_64B != 0 {
        dmacfg |= GEM_BIT(GEM_ADDR64_OFFSET);
    }

    info!(
        "Write GEM_DMACFG @ {:#x}, dmacfg = {:#x}",
        MACB_IOBASE + GEM_DMACFG,
        dmacfg
    );
    writev((MACB_IOBASE + GEM_DMACFG) as *mut u32, dmacfg);
    0
}

fn gmac_init_multi_queues(config: macb_config) {
    let mut num_queues = 1;
    // bit 0 is never set but queue 0 always exists
    let queue_mask: u32 = 0xff & readv((MACB_IOBASE + GEM_DCFG6) as *const u32);
    queue_mask |= 0x1;

    for i in 1..MACB_MAX_QUEUES {
        if (queue_mask & (1 << i)) != 0 {
            num_queues += 1;
        }
    }

    let dummy_desc_dma =
        MEM::dma_alloc_coherent((1 * DMA_DESC_SIZE + (MEM::PAGE_SIZE - 1)) / MEM::PAGE_SIZE);
    let dummy_desc = unsafe {
        slice::from_raw_parts_mut(
            dummy_desc_dma as *mut DmaDesc,
            1 * DMA_DESC_SIZE / size_of::<DmaDesc>(),
        )
    };
    dummy_desc[0].ctrl = 1 << MACB_TX_USED_OFFSET;
    dummy_desc[0].addr = 0;
    flush_dcache_range(); // dummy_desc_dma, len: MACB_TX_DUMMY_DMA_DESC_SIZE

    let paddr: u64 = dummy_desc_dma;

    let GEM_TBQP = |hw_q| 0x0440 + ((hw_q) << 2);
    let GEM_RBQP = |hw_q| 0x0480 + ((hw_q) << 2);
    let GEM_TBQPH = |hw_q| 0x04C8;
    let GEM_RBQPH = |hw_q| 0x04D4;

    for i in 1..num_queues {
        writev(
            (MACB_IOBASE + GEM_TBQP(i - 1)) as *mut u32,
            lower_32_bits(paddr),
        );
        writev(
            (MACB_IOBASE + GEM_RBQP(i - 1)) as *mut u32,
            lower_32_bits(paddr),
        );
        if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
            writev(
                (MACB_IOBASE + GEM_TBQPH(i - 1)) as *mut u32,
                upper_32_bits(paddr),
            );
            writev(
                (MACB_IOBASE + GEM_RBQPH(i - 1)) as *mut u32,
                upper_32_bits(paddr),
            );
        }
    }
}

fn flush_dcache_range() {}

fn is_big_endian() -> bool {
    cfg!(target_endian = "big")
}

fn upper_32_bits(n: u64) -> u32 {
    ((n >> 16) >> 16) as u32
}

fn lower_32_bits(n: u64) -> u32 {
    (n & 0xffffffff) as u32
}
