use core::mem::size_of;
use core::slice;
use alloc::boxed::Box;
use alloc::vec::Vec;
use log::*;

use crate::macb_const::*;
use crate::mii_const::*;
use crate::eth_macb::*;
use crate::phy_mscc::vsc8541_config;

/*
 * These buffer sizes must be power of 2 and divisible
 * by RX_BUFFER_MULTIPLE
 */
pub const MACB_RX_BUFFER_SIZE: usize = 128;
pub const GEM_RX_BUFFER_SIZE: usize = 2048;
pub const RX_BUFFER_MULTIPLE: usize = 64;

pub const MACB_RX_RING_SIZE: usize = 32;
pub const MACB_TX_RING_SIZE: usize = 16;

pub const MACB_TX_TIMEOUT: u64 = 1000;
pub const MACB_AUTONEG_TIMEOUT: u64 = 5000000;

pub const HW_DMA_CAP_32B: u32 = 0;
pub const HW_DMA_CAP_64B: u32 = 1;
pub const DMA_DESC_SIZE: usize = 16;

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

#[derive(Debug)]
#[derive(PartialEq)]
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
    BASEX1000 = 12,
    BASEX2500 = 13,
    XGMII = 14,
    XAUI = 15,
    RXAUI = 16,
    SFI = 17,
    INTERNAL = 18,
    AUI_25G = 19,
    XLAUI = 20,
    CAUI2 = 21,
    CAUI4 = 22,
    NCSI = 23,
    BASER10G = 24,
    USXGMII = 25,
    NONE = 26, /* Must be last */

    COUNT = 27,
}

struct MacbDevice<'a> {
	regs: u32, //MACB_IOBASE
	is_big_endian: bool,
	config: MacbConfig,

		rx_tail: u32,
		tx_head: u32,
		tx_tail: u32,
		next_rx_tail: u32,
		wrapped: bool,
    /*
	void			*rx_buffer;
	void			*tx_buffer;
    */
	rx_ring: &'a mut [DmaDesc],
	tx_ring: &'a mut [DmaDesc],
	rx_buffer_size: usize,
	rx_buffer_dma: usize,
	rx_ring_dma: usize,
	tx_ring_dma: usize,
	dummy_desc: &'a mut [DmaDesc],
	dummy_desc_dma: usize,

	phy_addr: u32,
	pclk_rate: u64,
	phy_interface: PhyInterfaceMode,
}

struct MacbConfig {
    dma_burst_length: u32,
    hw_dma_cap: u32,
    caps: u32,
    clk_init: usize, // fn clk_init
    //clk_init: Box<dyn Fn(u64)>,

    usrio_mii: u32,
    usrio_rmii: u32,
    usrio_rgmii: u32,
    usrio_clken: u32,
}

fn macb_start(macb: &mut MacbDevice, name: &str) -> i32 {
    let rx_buffer_size = GEM_RX_BUFFER_SIZE;
    let name = "ethernet@10090000";

    // sifive_config
    let config = MacbConfig {
        dma_burst_length: 16,
        hw_dma_cap: HW_DMA_CAP_32B,
        caps: 0,
        //clk_init: Box::new(macb_sifive_clk_init),
        clk_init: macb_sifive_clk_init as usize,

        // macb_usrio_cfg
        usrio_mii: 1 << MACB_MII_OFFSET,
        usrio_rmii: 1 << MACB_RMII_OFFSET,
        usrio_rgmii: 1 << GEM_RGMII_OFFSET,
        usrio_clken: 1 << MACB_CLKEN_OFFSET,
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
    let rx_buffer_dma: usize = MEM::dma_alloc_coherent(alloc_rx_buffer_pages);

    info!("Set ring desc buffer for RX");
    let count = 0;
    let paddr: u64 = rx_buffer_dma as u64;
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

        recv_buffers.push(phys_to_virt(paddr as usize));
        paddr += (rx_buffer_size as u64);

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

        send_buffers.push(phys_to_virt(paddr as usize));
    }
    flush_dcache_range(); // TX dma ring

    info!(
        "send_buffers length: {}, recv_buffers length: {}",
        send_buffers.len(),
        recv_buffers.len()
    );

    let dummy_desc_dma =
        MEM::dma_alloc_coherent((1 * DMA_DESC_SIZE + (MEM::PAGE_SIZE - 1)) / MEM::PAGE_SIZE);
    let dummy_desc = unsafe {
        slice::from_raw_parts_mut(
            dummy_desc_dma as *mut DmaDesc,
            1 * DMA_DESC_SIZE / size_of::<DmaDesc>(),
        )
    };

    let pclk_rate = 125125000; // from eth_macb.rs
    let mut macb = MacbDevice {
	regs: MACB_IOBASE,
	is_big_endian: is_big_endian(),
	config,

		rx_tail: 0,
		tx_head: 0,
		tx_tail: 0,
		next_rx_tail: 0,
    
	wrapped: false, // ?
    /*
	void			*rx_buffer;
	void			*tx_buffer;
    */
	rx_ring,
	tx_ring,
	rx_buffer_size,
	rx_buffer_dma,
	rx_ring_dma,
	tx_ring_dma,
	dummy_desc,
	dummy_desc_dma,

	phy_addr: 0,
	pclk_rate,
	phy_interface: PhyInterfaceMode::GMII,
};
    /*
     macb.rx_tail = 0;
     macb.tx_head = 0;
     macb.tx_tail = 0;
     macb.next_rx_tail = 0;
    */

    writev(
        (MACB_IOBASE + MACB_RBQP) as *mut u32,
        lower_32_bits(rx_ring_dma as u64),
    );
    writev(
        (MACB_IOBASE + MACB_TBQP) as *mut u32,
        lower_32_bits(tx_ring_dma as u64),
    );
    if config.hw_dma_cap & HW_DMA_CAP_64B != 0 {
        writev(
            (MACB_IOBASE + MACB_RBQPH) as *mut u32,
            upper_32_bits(rx_ring_dma as u64),
        );
        writev(
            (MACB_IOBASE + MACB_TBQPH) as *mut u32,
            upper_32_bits(tx_ring_dma as u64),
        );
    }

    let val: u32 = 0;
    if macb_is_gem() {
        // Initialize DMA properties
        gmac_configure_dma(&mut macb);
        // Check the multi queue and initialize the queue for tx
        gmac_init_multi_queues(&mut macb);

        if (macb.phy_interface == PhyInterfaceMode::RGMII)
            || (macb.phy_interface == PhyInterfaceMode::RGMII_ID)
            || (macb.phy_interface == PhyInterfaceMode::RGMII_RXID)
            || (macb.phy_interface == PhyInterfaceMode::RGMII_TXID)
        {
            val = config.usrio_rgmii;
        } else if macb.phy_interface == PhyInterfaceMode::RMII {
            val = config.usrio_rmii;
        } else if macb.phy_interface == PhyInterfaceMode::MII {
            val = config.usrio_mii;
        }

        if (config.caps & (MACB_CAPS_USRIO_HAS_CLKEN as u32)) != 0 {
            val |= config.usrio_clken;
        }

        writev((MACB_IOBASE + GEM_USRIO) as *mut u32, val);

        if macb.phy_interface == PhyInterfaceMode::SGMII {
            let ncfgr: u32 = readv((MACB_IOBASE + MACB_NCFGR) as *const u32);
            ncfgr |= (1 << GEM_SGMIIEN_OFFSET) | (1 << GEM_PCSSEL_OFFSET);
            info!("Write MACB_NCFGR: {:#x} when SGMII", ncfgr);
            writev((MACB_IOBASE + MACB_NCFGR) as *mut u32, ncfgr);
        }
    } else {
        if macb.phy_interface == PhyInterfaceMode::RMII {
            writev((MACB_IOBASE + MACB_USRIO) as *mut u32, 0);
        } else {
            writev((MACB_IOBASE + MACB_USRIO) as *mut u32, config.usrio_mii);
        }
    }

    let ret = macb_phy_init(&mut macb, name);
    if ret != 0 {
        return ret;
    }

    // Enable TX and RX
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, (1 << MACB_TE_OFFSET) | (1 << MACB_RE_OFFSET));

    0
}

fn macb_send(macb_tx_head: mut u32, packet: &[u8]) -> i32 {
    let mut tx_head = macb_tx_head;
    let length = packet.len();
    let paddr: u64 = flush_dcache_range(packet, length);

    let mut ctrl: u64 = length & TXBUF_FRMLEN_MASK;
    ctrl |= 1 << MACB_TX_LAST_OFFSET;
    if tx_head == (MACB_TX_RING_SIZE - 1) {
        ctrl |= 1 << MACB_TX_WRAP_OFFSET;
        macb_tx_head = 0;
    } else {
        macb_tx_head += 1;
    }
    if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
        tx_head = tx_head * 2;
        tx_ring[tx_head + 1].addrh = upper_32_bits(paddr);
    }
    tx_ring[tx_head].ctrl = ctrl;
    tx_ring[tx_head].addr = lower_32_bits(paddr);

    // barrier(); // For memory
    flush_dcache_range(); // TX ring dma desc
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, (1 << MACB_TE_OFFSET) | (1 << MACB_RE_OFFSET) | (1 << MACB_TSTART_OFFSET));

    /*
     * I guess this is necessary because the networking core may
     * re-use the transmit buffer as soon as we return...
     */
    for i in 0..=MACB_TX_TIMEOUT {
        //barrier();
        invalidate_dcache_range(); // TX ring dma desc
        ctrl = tx_ring[tx_head].ctl;
        if ctrl & (1 << MACB_TX_USED_OFFSET) != 0 {
            break;
        }
        usdelay(1);
    }
    // dma_unmap_single(paddr, length, DMA_TO_DEVICE);

    if i <= MACB_TX_TIMEOUT {
        if ctrl & MACB_BIT(TX_UNDERRUN) != 0 {
            info!("TX underrun");
        }
        if ctrl & MACB_BIT(TX_BUF_EXHAUSTED) != 0 {
            info!("TX buffers exhausted in mid frame");
        }
    } else {
        info!("TX timeout");
    }
    0
}

fn macb_recv(macb_rx_tail: mut u32, macb_next_rx_tail: mut u32, macb_wrapped: mut bool, packet: &mut [u8]) -> i32 {
    let mut status: u32 = 0;
    let mut flag = false;
    let mut length = 0;
    let mut count = 0;
    let mut next_rx_tail: u32 = macb_next_rx_tail;

    macb_wrapped = false;
    loop {
        count += 1;
        macb_invalidate_ring_desc(); // RX DMA DESC
        if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
            next_rx_tail = next_rx_tail * 2;
        }
        if rx_ring[next_rx_tail].addr & MACB_BIT(RX_USED) == 0 {
            return -11; // EAGAIN
        }
        status = rx_ring[next_rx_tail].ctrl;
        if status & (1 << MACB_RX_SOF_OFFSET) != 0 {
        if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
            next_rx_tail = next_rx_tail / 2;
            flag = true;
        }
        if next_rx_tail != macb_rx_tail {
            reclaim_rx_buffers(macb, next_rx_tail);
        }
        macb_wrapped = false;
        }

		if (status & (1 << MACB_RX_EOF_OFFSET)) {
			buffer = rx_buffer + rx_buffer_size * macb_rx_tail;
			length = status & RXBUF_FRMLEN_MASK;

			invalidate_dcache_range(); // rx_buffer_dma
			if macb_wrapped {
				let headlen = rx_buffer_size * (MACB_RX_RING_SIZE - macb_rx_tail);
				let taillen = length - headlen;
                /*
				memcpy((void *)net_rx_packets[0],
				       buffer, headlen);
				memcpy((void *)net_rx_packets[0] + headlen,
				       macb->rx_buffer, taillen);
				*packetp = (void *)net_rx_packets[0];
                */
			} else {
				*packet = buffer;
			}

			if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
				if !flag {
					next_rx_tail = next_rx_tail / 2;
                }
			}

            next_rx_tail += 1;
			if next_rx_tail >= MACB_RX_RING_SIZE {
				next_rx_tail = 0;
            }
			macb_next_rx_tail = next_rx_tail;

			info!("RX count: {}, length: {}", count, length);
			return length;
		} else {
			if (config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
				if !flag {
					next_rx_tail = next_rx_tail / 2;
                }
				flag = false;
			}

            next_rx_tail += 1;
			if next_rx_tail >= MACB_RX_RING_SIZE {
				macb_wrapped = true;
				next_rx_tail = 0;
			}
		}
		// barrier();
    }

}

fn reclaim_rx_buffers(mut macb: MacbDevice, new_tail: u32) {
    let mut count = 0;
	let mut i = macb.rx_tail;
	info!("reclaim_rx_buffers, rx_tail: {}, new_tail: {}}", i, new_tail);

	invalidate_dcache_range(); // RX ring dma
	while (i > new_tail) {
		if (macb.config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
			count = i * 2;
        } else {
			count = i;
        }
		macb.rx_ring[count].addr &= !(1 << MACB_RX_USED_OFFSET);
		i += 1;
		if i > MACB_RX_RING_SIZE {
			i = 0;
        }
	}
	while (i < new_tail) {
		if (macb.config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
			count = i * 2;
        } else {
			count = i;
        }
		macb.rx_ring[count].addr &= !(1 << MACB_RX_USED_OFFSET);
		i += 1;
	}
	// barrier();
	flush_dcache_range(); // RX ring dma
	macb.rx_tail = new_tail;
}

fn macb_phy_init(macb: &mut MacbDevice, name: &str) -> i32 {
    // Auto-detect phy_addr
    let mut ret = macb_phy_find(macb);
    if ret != 0 {
        return ret;
    }

    // Check if the PHY is up to snuff...
    let phy_id: u16 = macb_mdio_read(macb.phy_addr, MII_PHYSID1);
    if phy_id == 0xffff {
        error!("No PHY present");
        return -10; // ENODEV
    }

    // Find macb->phydev
    phy_connect_dev(&macb);
    phy_config();

    let mut status = macb_mdio_read(macb.phy_addr, MII_BMSR) as u32;
    if (status & BMSR_LSTATUS) == 0 {
        // Try to re-negotiate if we don't have link already.
        macb_phy_reset(&macb, name);
        let mut i = 0;
        while i < (MACB_AUTONEG_TIMEOUT / 100) {
            i += 1;
            status = macb_mdio_read(macb.phy_addr, MII_BMSR) as u32;
            if (status & BMSR_LSTATUS) != 0 {
                // Delay a bit after the link is established, so that the next xfer does not fail
                msdelay(10);
                break;
            }
            usdelay(100);
        }
    }

    if (status & BMSR_LSTATUS) == 0 {
        error!("{} link down (status: {:#x})", name, status);
        return -100; // ENETDOWN
    }

    let mut ncfgr: u32 = 0;
    let mut lpa: u16 = 0;
    let mut adv: u16 = 0;

    // First check for GMAC and that it is GiB capable
    if gem_is_gigabit_capable() {
        lpa = macb_mdio_read(macb.phy_addr, MII_STAT1000);

        if (lpa & (LPA_1000FULL | LPA_1000HALF | LPA_1000XFULL | LPA_1000XHALF) as u16) != 0 {
            let duplex = if (lpa & (LPA_1000FULL | LPA_1000XFULL) as u16) == 0 { 0 }else{ 1 }; 
            let duplex_str = if duplex == 1 { "full" }else{ "half" };
            info!("{} GiB capable, link up, 1000Mbps {}-duplex (lpa: {:#x})", name, duplex_str, lpa);
            
            ncfgr = readv((MACB_IOBASE + MACB_NCFGR) as *const u32);
            ncfgr &= !((1<< MACB_SPD_OFFSET) | (1 << MACB_FD_OFFSET));
            ncfgr |= 1 << GEM_GBE_OFFSET;
            if duplex == 1 {
                ncfgr |= 1 << MACB_FD_OFFSET;
            }

            writev((MACB_IOBASE + MACB_NCFGR) as *mut u32, ncfgr);

            macb_linkspd_cb(_1000BASET);

            return 0;
        }
    }

    // fall back for EMAC checking
    adv = macb_mdio_read(macb.phy_addr, MII_ADVERTISE);
    lpa = macb_mdio_read(macb.phy_addr, MII_LPA);
    let media = mii_nway_result(lpa & adv);

    let speed = if (media & (ADVERTISE_100FULL | ADVERTISE_100HALF)) == 0 { 0 }else{ 1 };
    let speed_str = if speed == 1 {"100"}else{"10"};
    let duplex = if (media & ADVERTISE_FULL) == 0 { 0 }else{ 0 };
    let duplex_str = if duplex == 1 { "full" }else{ "half" };
    info!("{} link up, {}Mbps {}-duplex (lpa: {:#x})", name, speed_str, duplex_str, lpa);

    ncfgr = readv((MACB_IOBASE + MACB_NCFGR) as *const u32);
    ncfgr &= !((1 << MACB_SPD_OFFSET) | (1 << MACB_FD_OFFSET) | (1 << GEM_GBE_OFFSET));
    if speed == 1 {
        ncfgr |= 1 << MACB_SPD_OFFSET;
        macb_linkspd_cb(_100BASET);
    }else {
        macb_linkspd_cb(_10BASET);
    }
    if duplex == 1 {
        ncfgr |= (1 << MACB_FD_OFFSET);
    }
    writev((MACB_IOBASE + MACB_NCFGR) as *mut u32, ncfgr);
    0
}

fn macb_phy_find(macb: &mut MacbDevice) -> i32 {
    //let mut phy_addr: u16 = 0;

    let mut phy_id: u16 = macb_mdio_read(macb.phy_addr, MII_PHYSID1);
    if phy_id != 0xffff {
        info!("PHY present at {}", macb.phy_addr);
        return 0;
    }
    // Search for PHY...
    for i in 0..32 {
        macb.phy_addr = i;
        phy_id = macb_mdio_read(macb.phy_addr, MII_PHYSID1);
        if phy_id != 0xffff {
            info!("PHY present at {}", i);
            return 0;
        }
    }

    // PHY isn't up to snuff
    error!("PHY not found");
    return -19; //ENODEV
}

fn macb_phy_reset(macb: &MacbDevice, name: &str) {
    let status = 0;
    let adv = ADVERTISE_CSMA | ADVERTISE_ALL;
    macb_mdio_write(macb.phy_addr, MII_ADVERTISE, adv as u16);
    info!("{} Starting autonegotiation...", name);
    macb_mdio_write(macb.phy_addr, MII_BMCR, (BMCR_ANENABLE | BMCR_ANRESTART) as u16);

    let mut i = 0;
    while i < (MACB_AUTONEG_TIMEOUT / 100) {
        i += 1;
        status = macb_mdio_read(phy_addr, MII_BMSR) as u32;
        if (status & BMSR_ANEGCOMPLETE) != 0 {
            break;
        }
        usdelay(100);
    }

    if (status & BMSR_ANEGCOMPLETE) != 0 {
        info!("{} Autonegotiation complete", name);
    } else {
        warn!("{} Autonegotiation timed out (status={:#x})", name, status);
    }
}

fn phy_connect_dev(macb: &MacbDevice) {
    let phydev_addr = macb.phy_addr as u32;
    let phydev_interface = macb.phy_interface;
    let phydev_flags = 0;

    let phydev = 0xff;
    let mask: u32 = if phydev_addr >= 0 { 1 << phydev_addr } else { 0xffffffff };
    /*
    // Find phydev by maskaddr and interface
    if phydev == 0 {
        phydev = phy_find_by_mask(bus, mask, interface);
    }
    */
    /*
    phydev->flags: 0, 
    phydev->addr: 0, 
    phydev->interface: 1,
    */

    if phydev != 0 {
    /* Soft Reset the PHY */
    phy_reset(phydev_addr, phydev_interface, phydev_flags);
    info!("Ethernet connected to PHY");

    // phy_config needs phydev
    vsc8541_config(phydev_addr, phydev_interface);

    } else {
        error!("Could not get PHY for ethernet: addr {}\n", macb.phy_addr);
    }
}

fn phy_reset(phydev_addr: u32, _interface: PhyInterfaceMode, phydev_flags: u32) -> i32 {
    let mut timeout = 500;
    let devad = MDIO_DEVAD_NONE;

    if (phydev_flags & PHY_FLAG_BROKEN_RESET) != 0 {
        info!("PHY soft reset not supported");
        return 0;
    }

    macb_mdio_write(phydev_addr, MII_BMCR, BMCR_RESET as u16);
    /*
     * Poll the control register for the reset bit to go to 0 (it is
     * auto-clearing).  This should happen within 0.5 seconds per the
     * IEEE spec.
     */
    let mut reg: u16 = macb_mdio_read(phydev_addr, MII_BMCR);
    while ((reg & BMCR_RESET as u16) != 0) && (timeout != 0) {
        timeout -= 1;
        reg = macb_mdio_read(phydev_addr, MII_BMCR);
        if reg < 0 {
            error!("PHY status read failed");
            return -1;
        }
        usdelay(1000);
    }
    if (reg & BMCR_RESET as u16) != 0 {
        error!("PHY reset timed out");
        return -1;
    }
    0
}

fn phy_config() {
    // Microsemi VSC8541 PHY driver config fn: vsc8541_config()
    // phy_config needs phydev struct after found by phy_connect
}

pub fn macb_mdio_write(phy_adr: u32, reg: u32, value: u16) {
    let mut netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl |= 1 << MACB_MPE_OFFSET;
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);

    // MACB_BF(name,value) 
    // (((value) & ((1 << MACB_x_SIZE) - 1)) << MACB_x_OFFSET)

    let frame: u32 = ((1 & ((1 << MACB_SOF_SIZE) - 1)) << MACB_SOF_OFFSET)
                    | ((1 & ((1 << MACB_RW_SIZE) - 1)) << MACB_RW_OFFSET)
                    | ((phy_adr & ((1 << MACB_PHYA_SIZE) - 1)) << MACB_PHYA_OFFSET)
                    | ((reg & ((1 << MACB_REGA_SIZE) - 1)) << MACB_REGA_OFFSET)
                    | ((2 & ((1 << MACB_CODE_SIZE) - 1)) << MACB_CODE_OFFSET)
                    | ((value as u32 & ((1 << MACB_DATA_SIZE) - 1)) << MACB_DATA_OFFSET);

    writev((MACB_IOBASE + MACB_MAN) as *mut u32, frame);
    while (readv((MACB_IOBASE + MACB_NSR) as *const u32) & (1 << MACB_IDLE_OFFSET)) == 0 {}

    netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl &= !(1 << MACB_MPE_OFFSET);
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);
}

pub fn macb_mdio_read(phy_adr: u32, reg: u32) -> u16 {
    let mut netctl = readv((MACB_IOBASE + MACB_NCR) as *const u32);
    netctl |= 1 << MACB_MPE_OFFSET;
    writev((MACB_IOBASE + MACB_NCR) as *mut u32, netctl);

    let mut frame: u32 = ((1 & ((1 << MACB_SOF_SIZE) - 1)) << MACB_SOF_OFFSET)
                    | ((2 & ((1 << MACB_RW_SIZE) - 1)) << MACB_RW_OFFSET)
                    | ((phy_adr & ((1 << MACB_PHYA_SIZE) - 1)) << MACB_PHYA_OFFSET)
                    | ((reg & ((1 << MACB_REGA_SIZE) - 1)) << MACB_REGA_OFFSET)
                    | ((2 & ((1 << MACB_CODE_SIZE) - 1)) << MACB_CODE_OFFSET);

    writev((MACB_IOBASE + MACB_MAN) as *mut u32, frame);
    while (readv((MACB_IOBASE + MACB_NSR) as *const u32) & (1 << MACB_IDLE_OFFSET)) == 0 {}

    frame = readv((MACB_IOBASE + MACB_MAN) as *const u32);

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

fn gmac_configure_dma(macb: &MacbDevice) -> i32 {
    let GEM_BF = |gem_offset, gem_size, value| (value & ((1 << gem_size) - 1)) << gem_offset;
    let GEM_BFINS = |gem_offset, gem_size, value, old| -> u32 {
        (old & !(((1 << gem_size) - 1) << gem_offset)) | GEM_BF(gem_offset, gem_size, value)
    };
    let GEM_BIT = |offset| 1 << offset;

    let buffer_size: u32 = (macb.rx_buffer_size / RX_BUFFER_MULTIPLE) as u32;
    let dmacfg: u32 = readv((MACB_IOBASE + GEM_DMACFG) as *const u32)
        & !GEM_BF(GEM_RXBS_OFFSET, GEM_RXBS_SIZE, -1);
    dmacfg |= GEM_BF(GEM_RXBS_OFFSET, GEM_RXBS_SIZE, buffer_size);

    if macb.config.dma_burst_length != 0 {
        dmacfg = GEM_BFINS(
            GEM_FBLDO_OFFSET,
            GEM_FBLDO_SIZE,
            macb.config.dma_burst_length,
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
    if macb.config.hw_dma_cap & HW_DMA_CAP_64B != 0 {
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

fn gmac_init_multi_queues(macb: &mut MacbDevice) {
    let mut num_queues = 1;
    // bit 0 is never set but queue 0 always exists
    let queue_mask: u32 = 0xff & readv((MACB_IOBASE + GEM_DCFG6) as *const u32);
    queue_mask |= 0x1;

    for i in 1..MACB_MAX_QUEUES {
        if (queue_mask & (1 << i)) != 0 {
            num_queues += 1;
        }
    }

    macb.dummy_desc[0].ctrl = 1 << MACB_TX_USED_OFFSET;
    macb.dummy_desc[0].addr = 0;
    flush_dcache_range(); // dummy_desc_dma, len: MACB_TX_DUMMY_DMA_DESC_SIZE

    let paddr: u64 = macb.dummy_desc_dma as u64;

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
        if (macb.config.hw_dma_cap & HW_DMA_CAP_64B) != 0 {
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
