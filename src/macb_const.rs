//cadence/macb.h
//Atmel MACB Ethernet Controller driver

pub const MACB_GREGS_NBR: u64 = 16;
pub const MACB_GREGS_VERSION: u64 = 2;
pub const MACB_MAX_QUEUES: u64 = 8;

/* MACB register offsets */
pub const MACB_NCR: u64 = 0x0000; /* Network Control */
pub const MACB_NCFGR: u64 = 0x0004; /* Network Config */
pub const MACB_NSR: u64 = 0x0008; /* Network Status */
pub const MACB_TAR: u64 = 0x000c; /* AT91RM9200 only */
pub const MACB_TCR: u64 = 0x0010; /* AT91RM9200 only */
pub const MACB_TSR: u64 = 0x0014; /* Transmit Status */
pub const MACB_RBQP: u64 = 0x0018; /* RX Q Base Address */
pub const MACB_TBQP: u64 = 0x001c; /* TX Q Base Address */
pub const MACB_RSR: u64 = 0x0020; /* Receive Status */
pub const MACB_ISR: u64 = 0x0024; /* Interrupt Status */
pub const MACB_IER: u64 = 0x0028; /* Interrupt Enable */
pub const MACB_IDR: u64 = 0x002c; /* Interrupt Disable */
pub const MACB_IMR: u64 = 0x0030; /* Interrupt Mask */
pub const MACB_MAN: u64 = 0x0034; /* PHY Maintenance */
pub const MACB_PTR: u64 = 0x0038;
pub const MACB_PFR: u64 = 0x003c;
pub const MACB_FTO: u64 = 0x0040;
pub const MACB_SCF: u64 = 0x0044;
pub const MACB_MCF: u64 = 0x0048;
pub const MACB_FRO: u64 = 0x004c;
pub const MACB_FCSE: u64 = 0x0050;
pub const MACB_ALE: u64 = 0x0054;
pub const MACB_DTF: u64 = 0x0058;
pub const MACB_LCOL: u64 = 0x005c;
pub const MACB_EXCOL: u64 = 0x0060;
pub const MACB_TUND: u64 = 0x0064;
pub const MACB_CSE: u64 = 0x0068;
pub const MACB_RRE: u64 = 0x006c;
pub const MACB_ROVR: u64 = 0x0070;
pub const MACB_RSE: u64 = 0x0074;
pub const MACB_ELE: u64 = 0x0078;
pub const MACB_RJA: u64 = 0x007c;
pub const MACB_USF: u64 = 0x0080;
pub const MACB_STE: u64 = 0x0084;
pub const MACB_RLE: u64 = 0x0088;
pub const MACB_TPF: u64 = 0x008c;
pub const MACB_HRB: u64 = 0x0090;
pub const MACB_HRT: u64 = 0x0094;
pub const MACB_SA1B: u64 = 0x0098;
pub const MACB_SA1T: u64 = 0x009c;
pub const MACB_SA2B: u64 = 0x00a0;
pub const MACB_SA2T: u64 = 0x00a4;
pub const MACB_SA3B: u64 = 0x00a8;
pub const MACB_SA3T: u64 = 0x00ac;
pub const MACB_SA4B: u64 = 0x00b0;
pub const MACB_SA4T: u64 = 0x00b4;
pub const MACB_TID: u64 = 0x00b8;
pub const MACB_TPQ: u64 = 0x00bc;
pub const MACB_USRIO: u64 = 0x00c0;
pub const MACB_WOL: u64 = 0x00c4;
pub const MACB_MID: u64 = 0x00fc;
pub const MACB_TBQPH: u64 = 0x04C8;
pub const MACB_RBQPH: u64 = 0x04D4;

/* GEM register offsets. */
pub const GEM_NCR: u64 = 0x0000; /* Network Control */
pub const GEM_NCFGR: u64 = 0x0004; /* Network Config */
pub const GEM_USRIO: u64 = 0x000c; /* User IO */
pub const GEM_DMACFG: u64 = 0x0010; /* DMA Configuration */
pub const GEM_JML: u64 = 0x0048; /* Jumbo Max Length */
pub const GEM_HS_MAC_CONFIG: u64 = 0x0050; /* GEM high speed config */
pub const GEM_HRB: u64 = 0x0080; /* Hash Bottom */
pub const GEM_HRT: u64 = 0x0084; /* Hash Top */
pub const GEM_SA1B: u64 = 0x0088; /* Specific1 Bottom */
pub const GEM_SA1T: u64 = 0x008C; /* Specific1 Top */
pub const GEM_SA2B: u64 = 0x0090; /* Specific2 Bottom */
pub const GEM_SA2T: u64 = 0x0094; /* Specific2 Top */
pub const GEM_SA3B: u64 = 0x0098; /* Specific3 Bottom */
pub const GEM_SA3T: u64 = 0x009C; /* Specific3 Top */
pub const GEM_SA4B: u64 = 0x00A0; /* Specific4 Bottom */
pub const GEM_SA4T: u64 = 0x00A4; /* Specific4 Top */
pub const GEM_WOL: u64 = 0x00b8; /* Wake on LAN */
pub const GEM_EFTSH: u64 = 0x00e8; /* PTP Event Frame Transmitted Seconds Register 47:32 */
pub const GEM_EFRSH: u64 = 0x00ec; /* PTP Event Frame Received Seconds Register 47:32 */
pub const GEM_PEFTSH: u64 = 0x00f0; /* PTP Peer Event Frame Transmitted Seconds Register 47:32 */
pub const GEM_PEFRSH: u64 = 0x00f4; /* PTP Peer Event Frame Received Seconds Register 47:32 */
pub const GEM_OTX: u64 = 0x0100; /* Octets transmitted */
pub const GEM_OCTTXL: u64 = 0x0100; /* Octets transmitted [31:0] */
pub const GEM_OCTTXH: u64 = 0x0104; /* Octets transmitted [47:32] */
pub const GEM_TXCNT: u64 = 0x0108; /* Frames Transmitted counter */
pub const GEM_TXBCCNT: u64 = 0x010c; /* Broadcast Frames counter */
pub const GEM_TXMCCNT: u64 = 0x0110; /* Multicast Frames counter */
pub const GEM_TXPAUSECNT: u64 = 0x0114; /* Pause Frames Transmitted Counter */
pub const GEM_TX64CNT: u64 = 0x0118; /* 64 byte Frames TX counter */
pub const GEM_TX65CNT: u64 = 0x011c; /* 65-127 byte Frames TX counter */
pub const GEM_TX128CNT: u64 = 0x0120; /* 128-255 byte Frames TX counter */
pub const GEM_TX256CNT: u64 = 0x0124; /* 256-511 byte Frames TX counter */
pub const GEM_TX512CNT: u64 = 0x0128; /* 512-1023 byte Frames TX counter */
pub const GEM_TX1024CNT: u64 = 0x012c; /* 1024-1518 byte Frames TX counter */
pub const GEM_TX1519CNT: u64 = 0x0130; /* 1519+ byte Frames TX counter */
pub const GEM_TXURUNCNT: u64 = 0x0134; /* TX under run error counter */
pub const GEM_SNGLCOLLCNT: u64 = 0x0138; /* Single Collision Frame Counter */
pub const GEM_MULTICOLLCNT: u64 = 0x013c; /* Multiple Collision Frame Counter */
pub const GEM_EXCESSCOLLCNT: u64 = 0x0140; /* Excessive Collision Frame Counter */
pub const GEM_LATECOLLCNT: u64 = 0x0144; /* Late Collision Frame Counter */
pub const GEM_TXDEFERCNT: u64 = 0x0148; /* Deferred Transmission Frame Counter */
pub const GEM_TXCSENSECNT: u64 = 0x014c; /* Carrier Sense Error Counter */
pub const GEM_ORX: u64 = 0x0150; /* Octets received */
pub const GEM_OCTRXL: u64 = 0x0150; /* Octets received [31:0] */
pub const GEM_OCTRXH: u64 = 0x0154; /* Octets received [47:32] */
pub const GEM_RXCNT: u64 = 0x0158; /* Frames Received Counter */
pub const GEM_RXBROADCNT: u64 = 0x015c; /* Broadcast Frames Received Counter */
pub const GEM_RXMULTICNT: u64 = 0x0160; /* Multicast Frames Received Counter */
pub const GEM_RXPAUSECNT: u64 = 0x0164; /* Pause Frames Received Counter */
pub const GEM_RX64CNT: u64 = 0x0168; /* 64 byte Frames RX Counter */
pub const GEM_RX65CNT: u64 = 0x016c; /* 65-127 byte Frames RX Counter */
pub const GEM_RX128CNT: u64 = 0x0170; /* 128-255 byte Frames RX Counter */
pub const GEM_RX256CNT: u64 = 0x0174; /* 256-511 byte Frames RX Counter */
pub const GEM_RX512CNT: u64 = 0x0178; /* 512-1023 byte Frames RX Counter */
pub const GEM_RX1024CNT: u64 = 0x017c; /* 1024-1518 byte Frames RX Counter */
pub const GEM_RX1519CNT: u64 = 0x0180; /* 1519+ byte Frames RX Counter */
pub const GEM_RXUNDRCNT: u64 = 0x0184; /* Undersize Frames Received Counter */
pub const GEM_RXOVRCNT: u64 = 0x0188; /* Oversize Frames Received Counter */
pub const GEM_RXJABCNT: u64 = 0x018c; /* Jabbers Received Counter */
pub const GEM_RXFCSCNT: u64 = 0x0190; /* Frame Check Sequence Error Counter */
pub const GEM_RXLENGTHCNT: u64 = 0x0194; /* Length Field Error Counter */
pub const GEM_RXSYMBCNT: u64 = 0x0198; /* Symbol Error Counter */
pub const GEM_RXALIGNCNT: u64 = 0x019c; /* Alignment Error Counter */
pub const GEM_RXRESERRCNT: u64 = 0x01a0; /* Receive Resource Error Counter */
pub const GEM_RXORCNT: u64 = 0x01a4; /* Receive Overrun Counter */
pub const GEM_RXIPCCNT: u64 = 0x01a8; /* IP header Checksum Error Counter */
pub const GEM_RXTCPCCNT: u64 = 0x01ac; /* TCP Checksum Error Counter */
pub const GEM_RXUDPCCNT: u64 = 0x01b0; /* UDP Checksum Error Counter */
pub const GEM_TISUBN: u64 = 0x01bc; /* 1588 Timer Increment Sub-ns */
pub const GEM_TSH: u64 = 0x01c0; /* 1588 Timer Seconds High */
pub const GEM_TSL: u64 = 0x01d0; /* 1588 Timer Seconds Low */
pub const GEM_TN: u64 = 0x01d4; /* 1588 Timer Nanoseconds */
pub const GEM_TA: u64 = 0x01d8; /* 1588 Timer Adjust */
pub const GEM_TI: u64 = 0x01dc; /* 1588 Timer Increment */
pub const GEM_EFTSL: u64 = 0x01e0; /* PTP Event Frame Tx Seconds Low */
pub const GEM_EFTN: u64 = 0x01e4; /* PTP Event Frame Tx Nanoseconds */
pub const GEM_EFRSL: u64 = 0x01e8; /* PTP Event Frame Rx Seconds Low */
pub const GEM_EFRN: u64 = 0x01ec; /* PTP Event Frame Rx Nanoseconds */
pub const GEM_PEFTSL: u64 = 0x01f0; /* PTP Peer Event Frame Tx Secs Low */
pub const GEM_PEFTN: u64 = 0x01f4; /* PTP Peer Event Frame Tx Ns */
pub const GEM_PEFRSL: u64 = 0x01f8; /* PTP Peer Event Frame Rx Sec Low */
pub const GEM_PEFRN: u64 = 0x01fc; /* PTP Peer Event Frame Rx Ns */
pub const GEM_PCSCNTRL: u64 = 0x0200; /* PCS Control */
pub const GEM_PCSSTS: u64 = 0x0204; /* PCS Status */
pub const GEM_PCSPHYTOPID: u64 = 0x0208; /* PCS PHY Top ID */
pub const GEM_PCSPHYBOTID: u64 = 0x020c; /* PCS PHY Bottom ID */
pub const GEM_PCSANADV: u64 = 0x0210; /* PCS AN Advertisement */
pub const GEM_PCSANLPBASE: u64 = 0x0214; /* PCS AN Link Partner Base */
pub const GEM_PCSANEXP: u64 = 0x0218; /* PCS AN Expansion */
pub const GEM_PCSANNPTX: u64 = 0x021c; /* PCS AN Next Page TX */
pub const GEM_PCSANNPLP: u64 = 0x0220; /* PCS AN Next Page LP */
pub const GEM_PCSANEXTSTS: u64 = 0x023c; /* PCS AN Extended Status */
pub const GEM_DCFG1: u64 = 0x0280; /* Design Config 1 */
pub const GEM_DCFG2: u64 = 0x0284; /* Design Config 2 */
pub const GEM_DCFG3: u64 = 0x0288; /* Design Config 3 */
pub const GEM_DCFG4: u64 = 0x028c; /* Design Config 4 */
pub const GEM_DCFG5: u64 = 0x0290; /* Design Config 5 */
pub const GEM_DCFG6: u64 = 0x0294; /* Design Config 6 */
pub const GEM_DCFG7: u64 = 0x0298; /* Design Config 7 */
pub const GEM_DCFG8: u64 = 0x029C; /* Design Config 8 */
pub const GEM_DCFG10: u64 = 0x02A4; /* Design Config 10 */
pub const GEM_DCFG12: u64 = 0x02AC; /* Design Config 12 */
pub const GEM_USX_CONTROL: u64 = 0x0A80; /* High speed PCS control register */
pub const GEM_USX_STATUS: u64 = 0x0A88; /* High speed PCS status register */

pub const GEM_TXBDCTRL: u64 = 0x04cc; /* TX Buffer Descriptor control register */
pub const GEM_RXBDCTRL: u64 = 0x04d0; /* RX Buffer Descriptor control register */

/* Screener Type 2 match registers */
pub const GEM_SCRT2: u64 = 0x540;

/* EtherType registers */
pub const GEM_ETHT: u64 = 0x06E0;

/* Type 2 compare registers */
pub const GEM_T2CMPW0: u64 = 0x0700;
pub const GEM_T2CMPW1: u64 = 0x0704;
//pub const T2CMP_OFST(t2idx)	(t2idx * 2)

/*
/* type 2 compare registers
 * each location requires 3 compare regs
 */
pub const GEM_IP4SRC_CMP(idx)		(idx * 3)
pub const GEM_IP4DST_CMP(idx)		(idx * 3 + 1)
pub const GEM_PORT_CMP(idx)		(idx * 3 + 2)

/* Which screening type 2 EtherType register will be used (0 - 7) */
pub const SCRT2_ETHT		0

pub const GEM_ISR(hw_q)		(0x0400 + ((hw_q) << 2))
pub const GEM_TBQP(hw_q)		(0x0440 + ((hw_q) << 2))
pub const GEM_TBQPH(hw_q)		(0x04C8)
pub const GEM_RBQP(hw_q)		(0x0480 + ((hw_q) << 2))
pub const GEM_RBQS(hw_q)		(0x04A0 + ((hw_q) << 2))
pub const GEM_RBQPH(hw_q)		(0x04D4)
pub const GEM_IER(hw_q)		(0x0600 + ((hw_q) << 2))
pub const GEM_IDR(hw_q)		(0x0620 + ((hw_q) << 2))
pub const GEM_IMR(hw_q)		(0x0640 + ((hw_q) << 2))

*/

/* Bitfields in NCR */
pub const MACB_LB_OFFSET: u64 = 0; /* reserved */
pub const MACB_LB_SIZE: u64 = 1;
pub const MACB_LLB_OFFSET: u64 = 1; /* Loop back local */
pub const MACB_LLB_SIZE: u64 = 1;
pub const MACB_RE_OFFSET: u64 = 2; /* Receive enable */
pub const MACB_RE_SIZE: u64 = 1;
pub const MACB_TE_OFFSET: u64 = 3; /* Transmit enable */
pub const MACB_TE_SIZE: u64 = 1;
pub const MACB_MPE_OFFSET: u64 = 4; /* Management port enable */
pub const MACB_MPE_SIZE: u64 = 1;
pub const MACB_CLRSTAT_OFFSET: u64 = 5; /* Clear stats regs */
pub const MACB_CLRSTAT_SIZE: u64 = 1;
pub const MACB_INCSTAT_OFFSET: u64 = 6; /* Incremental stats regs */
pub const MACB_INCSTAT_SIZE: u64 = 1;
pub const MACB_WESTAT_OFFSET: u64 = 7; /* Write enable stats regs */
pub const MACB_WESTAT_SIZE: u64 = 1;
pub const MACB_BP_OFFSET: u64 = 8; /* Back pressure */
pub const MACB_BP_SIZE: u64 = 1;
pub const MACB_TSTART_OFFSET: u64 = 9; /* Start transmission */
pub const MACB_TSTART_SIZE: u64 = 1;
pub const MACB_THALT_OFFSET: u64 = 10; /* Transmit halt */
pub const MACB_THALT_SIZE: u64 = 1;
pub const MACB_NCR_TPF_OFFSET: u64 = 11; /* Transmit pause frame */
pub const MACB_NCR_TPF_SIZE: u64 = 1;
pub const MACB_TZQ_OFFSET: u64 = 12; /* Transmit zero quantum pause frame */
pub const MACB_TZQ_SIZE: u64 = 1;
pub const MACB_SRTSM_OFFSET: u64 = 15; /* Store Receive Timestamp to Memory */
pub const MACB_OSSMODE_OFFSET: u64 = 24; /* Enable One Step Synchro Mode */
pub const MACB_OSSMODE_SIZE: u64 = 1;
pub const MACB_MIIONRGMII_OFFSET: u64 = 28; /* MII Usage on RGMII Interface */
pub const MACB_MIIONRGMII_SIZE: u64 = 1;

/* Bitfields in NCFGR */
pub const MACB_SPD_OFFSET: u64 = 0; /* Speed */
pub const MACB_SPD_SIZE: u64 = 1;
pub const MACB_FD_OFFSET: u64 = 1; /* Full duplex */
pub const MACB_FD_SIZE: u64 = 1;
pub const MACB_BIT_RATE_OFFSET: u64 = 2; /* Discard non-VLAN frames */
pub const MACB_BIT_RATE_SIZE: u64 = 1;
pub const MACB_JFRAME_OFFSET: u64 = 3; /* reserved */
pub const MACB_JFRAME_SIZE: u64 = 1;
pub const MACB_CAF_OFFSET: u64 = 4; /* Copy all frames */
pub const MACB_CAF_SIZE: u64 = 1;
pub const MACB_NBC_OFFSET: u64 = 5; /* No broadcast */
pub const MACB_NBC_SIZE: u64 = 1;
pub const MACB_NCFGR_MTI_OFFSET: u64 = 6; /* Multicast hash enable */
pub const MACB_NCFGR_MTI_SIZE: u64 = 1;
pub const MACB_UNI_OFFSET: u64 = 7; /* Unicast hash enable */
pub const MACB_UNI_SIZE: u64 = 1;
pub const MACB_BIG_OFFSET: u64 = 8; /* Receive 1536 byte frames */
pub const MACB_BIG_SIZE: u64 = 1;
pub const MACB_EAE_OFFSET: u64 = 9; /* External address match enable */
pub const MACB_EAE_SIZE: u64 = 1;
pub const MACB_CLK_OFFSET: u64 = 10;
pub const MACB_CLK_SIZE: u64 = 2;
pub const MACB_RTY_OFFSET: u64 = 12; /* Retry test */
pub const MACB_RTY_SIZE: u64 = 1;
pub const MACB_PAE_OFFSET: u64 = 13; /* Pause enable */
pub const MACB_PAE_SIZE: u64 = 1;
pub const MACB_RM9200_RMII_OFFSET: u64 = 13; /* AT91RM9200 only */
pub const MACB_RM9200_RMII_SIZE: u64 = 1; /* AT91RM9200 only */
pub const MACB_RBOF_OFFSET: u64 = 14; /* Receive buffer offset */
pub const MACB_RBOF_SIZE: u64 = 2;
pub const MACB_RLCE_OFFSET: u64 = 16; /* Length field error frame discard */
pub const MACB_RLCE_SIZE: u64 = 1;
pub const MACB_DRFCS_OFFSET: u64 = 17; /* FCS remove */
pub const MACB_DRFCS_SIZE: u64 = 1;
pub const MACB_EFRHD_OFFSET: u64 = 18;
pub const MACB_EFRHD_SIZE: u64 = 1;
pub const MACB_IRXFCS_OFFSET: u64 = 19;
pub const MACB_IRXFCS_SIZE: u64 = 1;

/* GEM specific NCR bitfields. */
pub const GEM_ENABLE_HS_MAC_OFFSET: u64 = 31;
pub const GEM_ENABLE_HS_MAC_SIZE: u64 = 1;

/* GEM specific NCFGR bitfields. */
pub const GEM_FD_OFFSET: u64 = 1; /* Full duplex */
pub const GEM_FD_SIZE: u64 = 1;
pub const GEM_GBE_OFFSET: u64 = 10; /* Gigabit mode enable */
pub const GEM_GBE_SIZE: u64 = 1;
pub const GEM_PCSSEL_OFFSET: u64 = 11;
pub const GEM_PCSSEL_SIZE: u64 = 1;
pub const GEM_PAE_OFFSET: u64 = 13; /* Pause enable */
pub const GEM_PAE_SIZE: u64 = 1;
pub const GEM_CLK_OFFSET: u64 = 18; /* MDC clock division */
pub const GEM_CLK_SIZE: u64 = 3;
pub const GEM_DBW_OFFSET: u64 = 21; /* Data bus width */
pub const GEM_DBW_SIZE: u64 = 2;
pub const GEM_RXCOEN_OFFSET: u64 = 24;
pub const GEM_RXCOEN_SIZE: u64 = 1;
pub const GEM_SGMIIEN_OFFSET: u64 = 27;
pub const GEM_SGMIIEN_SIZE: u64 = 1;

/* Constants for data bus width. */
pub const GEM_DBW32: u64 = 0; /* 32 bit AMBA AHB data bus width */
pub const GEM_DBW64: u64 = 1; /* 64 bit AMBA AHB data bus width */
pub const GEM_DBW128: u64 = 2; /* 128 bit AMBA AHB data bus width */

/* Bitfields in DMACFG. */
pub const GEM_FBLDO_OFFSET: u64 = 0; /* fixed burst length for DMA */
pub const GEM_FBLDO_SIZE: u64 = 5;
pub const GEM_ENDIA_DESC_OFFSET: u64 = 6; /* endian swap mode for management descriptor access */
pub const GEM_ENDIA_DESC_SIZE: u64 = 1;
pub const GEM_ENDIA_PKT_OFFSET: u64 = 7; /* endian swap mode for packet data access */
pub const GEM_ENDIA_PKT_SIZE: u64 = 1;
pub const GEM_RXBMS_OFFSET: u64 = 8; /* RX packet buffer memory size select */
pub const GEM_RXBMS_SIZE: u64 = 2;
pub const GEM_TXPBMS_OFFSET: u64 = 10; /* TX packet buffer memory size select */
pub const GEM_TXPBMS_SIZE: u64 = 1;
pub const GEM_TXCOEN_OFFSET: u64 = 11; /* TX IP/TCP/UDP checksum gen offload */
pub const GEM_TXCOEN_SIZE: u64 = 1;
pub const GEM_RXBS_OFFSET: u64 = 16; /* DMA receive buffer size */
pub const GEM_RXBS_SIZE: u64 = 8;
pub const GEM_DDRP_OFFSET: u64 = 24; /* disc_when_no_ahb */
pub const GEM_DDRP_SIZE: u64 = 1;
pub const GEM_RXEXT_OFFSET: u64 = 28; /* RX extended Buffer Descriptor mode */
pub const GEM_RXEXT_SIZE: u64 = 1;
pub const GEM_TXEXT_OFFSET: u64 = 29; /* TX extended Buffer Descriptor mode */
pub const GEM_TXEXT_SIZE: u64 = 1;
pub const GEM_ADDR64_OFFSET: u64 = 30; /* Address bus width - 64b or 32b */
pub const GEM_ADDR64_SIZE: u64 = 1;

/* Bitfields in NSR */
pub const MACB_NSR_LINK_OFFSET: u64 = 0; /* pcs_link_state */
pub const MACB_NSR_LINK_SIZE: u64 = 1;
pub const MACB_MDIO_OFFSET: u64 = 1; /* status of the mdio_in pin */
pub const MACB_MDIO_SIZE: u64 = 1;
pub const MACB_IDLE_OFFSET: u64 = 2; /* The PHY management logic is idle */
pub const MACB_IDLE_SIZE: u64 = 1;

/* Bitfields in TSR */
pub const MACB_UBR_OFFSET: u64 = 0; /* Used bit read */
pub const MACB_UBR_SIZE: u64 = 1;
pub const MACB_COL_OFFSET: u64 = 1; /* Collision occurred */
pub const MACB_COL_SIZE: u64 = 1;
pub const MACB_TSR_RLE_OFFSET: u64 = 2; /* Retry limit exceeded */
pub const MACB_TSR_RLE_SIZE: u64 = 1;
pub const MACB_TGO_OFFSET: u64 = 3; /* Transmit go */
pub const MACB_TGO_SIZE: u64 = 1;
pub const MACB_BEX_OFFSET: u64 = 4; /* TX frame corruption due to AHB error */
pub const MACB_BEX_SIZE: u64 = 1;
pub const MACB_RM9200_BNQ_OFFSET: u64 = 4; /* AT91RM9200 only */
pub const MACB_RM9200_BNQ_SIZE: u64 = 1; /* AT91RM9200 only */
pub const MACB_COMP_OFFSET: u64 = 5; /* Trnasmit complete */
pub const MACB_COMP_SIZE: u64 = 1;
pub const MACB_UND_OFFSET: u64 = 6; /* Trnasmit under run */
pub const MACB_UND_SIZE: u64 = 1;

/* Bitfields in RSR */
pub const MACB_BNA_OFFSET: u64 = 0; /* Buffer not available */
pub const MACB_BNA_SIZE: u64 = 1;
pub const MACB_REC_OFFSET: u64 = 1; /* Frame received */
pub const MACB_REC_SIZE: u64 = 1;
pub const MACB_OVR_OFFSET: u64 = 2; /* Receive overrun */
pub const MACB_OVR_SIZE: u64 = 1;

/* Bitfields in ISR/IER/IDR/IMR */
pub const MACB_MFD_OFFSET: u64 = 0; /* Management frame sent */
pub const MACB_MFD_SIZE: u64 = 1;
pub const MACB_RCOMP_OFFSET: u64 = 1; /* Receive complete */
pub const MACB_RCOMP_SIZE: u64 = 1;
pub const MACB_RXUBR_OFFSET: u64 = 2; /* RX used bit read */
pub const MACB_RXUBR_SIZE: u64 = 1;
pub const MACB_TXUBR_OFFSET: u64 = 3; /* TX used bit read */
pub const MACB_TXUBR_SIZE: u64 = 1;
pub const MACB_ISR_TUND_OFFSET: u64 = 4; /* Enable TX buffer under run interrupt */
pub const MACB_ISR_TUND_SIZE: u64 = 1;
pub const MACB_ISR_RLE_OFFSET: u64 = 5; /* EN retry exceeded/late coll interrupt */
pub const MACB_ISR_RLE_SIZE: u64 = 1;
pub const MACB_TXERR_OFFSET: u64 = 6; /* EN TX frame corrupt from error interrupt */
pub const MACB_TXERR_SIZE: u64 = 1;
pub const MACB_RM9200_TBRE_OFFSET: u64 = 6; /* EN may send new frame interrupt (RM9200) */
pub const MACB_RM9200_TBRE_SIZE: u64 = 1;
pub const MACB_TCOMP_OFFSET: u64 = 7; /* Enable transmit complete interrupt */
pub const MACB_TCOMP_SIZE: u64 = 1;
pub const MACB_ISR_LINK_OFFSET: u64 = 9; /* Enable link change interrupt */
pub const MACB_ISR_LINK_SIZE: u64 = 1;
pub const MACB_ISR_ROVR_OFFSET: u64 = 10; /* Enable receive overrun interrupt */
pub const MACB_ISR_ROVR_SIZE: u64 = 1;
pub const MACB_HRESP_OFFSET: u64 = 11; /* Enable hrsep not OK interrupt */
pub const MACB_HRESP_SIZE: u64 = 1;
pub const MACB_PFR_OFFSET: u64 = 12; /* Enable pause frame w/ quantum interrupt */
pub const MACB_PFR_SIZE: u64 = 1;
pub const MACB_PTZ_OFFSET: u64 = 13; /* Enable pause time zero interrupt */
pub const MACB_PTZ_SIZE: u64 = 1;
pub const MACB_WOL_OFFSET: u64 = 14; /* Enable wake-on-lan interrupt */
pub const MACB_WOL_SIZE: u64 = 1;
pub const MACB_DRQFR_OFFSET: u64 = 18; /* PTP Delay Request Frame Received */
pub const MACB_DRQFR_SIZE: u64 = 1;
pub const MACB_SFR_OFFSET: u64 = 19; /* PTP Sync Frame Received */
pub const MACB_SFR_SIZE: u64 = 1;
pub const MACB_DRQFT_OFFSET: u64 = 20; /* PTP Delay Request Frame Transmitted */
pub const MACB_DRQFT_SIZE: u64 = 1;
pub const MACB_SFT_OFFSET: u64 = 21; /* PTP Sync Frame Transmitted */
pub const MACB_SFT_SIZE: u64 = 1;
pub const MACB_PDRQFR_OFFSET: u64 = 22; /* PDelay Request Frame Received */
pub const MACB_PDRQFR_SIZE: u64 = 1;
pub const MACB_PDRSFR_OFFSET: u64 = 23; /* PDelay Response Frame Received */
pub const MACB_PDRSFR_SIZE: u64 = 1;
pub const MACB_PDRQFT_OFFSET: u64 = 24; /* PDelay Request Frame Transmitted */
pub const MACB_PDRQFT_SIZE: u64 = 1;
pub const MACB_PDRSFT_OFFSET: u64 = 25; /* PDelay Response Frame Transmitted */
pub const MACB_PDRSFT_SIZE: u64 = 1;
pub const MACB_SRI_OFFSET: u64 = 26; /* TSU Seconds Register Increment */
pub const MACB_SRI_SIZE: u64 = 1;
pub const GEM_WOL_OFFSET: u64 = 28; /* Enable wake-on-lan interrupt */
pub const GEM_WOL_SIZE: u64 = 1;

/* Timer increment fields */
pub const MACB_TI_CNS_OFFSET: u64 = 0;
pub const MACB_TI_CNS_SIZE: u64 = 8;
pub const MACB_TI_ACNS_OFFSET: u64 = 8;
pub const MACB_TI_ACNS_SIZE: u64 = 8;
pub const MACB_TI_NIT_OFFSET: u64 = 16;
pub const MACB_TI_NIT_SIZE: u64 = 8;

/* Bitfields in MAN */
pub const MACB_DATA_OFFSET: u64 = 0; /* data */
pub const MACB_DATA_SIZE: u64 = 16;
pub const MACB_CODE_OFFSET: u64 = 16; /* Must be written to 10 */
pub const MACB_CODE_SIZE: u64 = 2;
pub const MACB_REGA_OFFSET: u64 = 18; /* Register address */
pub const MACB_REGA_SIZE: u64 = 5;
pub const MACB_PHYA_OFFSET: u64 = 23; /* PHY address */
pub const MACB_PHYA_SIZE: u64 = 5;
pub const MACB_RW_OFFSET: u64 = 28; /* Operation. 10 is read. 01 is write. */
pub const MACB_RW_SIZE: u64 = 2;
pub const MACB_SOF_OFFSET: u64 = 30; /* Must be written to 1 for Clause 22 */
pub const MACB_SOF_SIZE: u64 = 2;

/* Bitfields in USRIO (AVR32) */
pub const MACB_MII_OFFSET: u64 = 0;
pub const MACB_MII_SIZE: u64 = 1;
pub const MACB_EAM_OFFSET: u64 = 1;
pub const MACB_EAM_SIZE: u64 = 1;
pub const MACB_TX_PAUSE_OFFSET: u64 = 2;
pub const MACB_TX_PAUSE_SIZE: u64 = 1;
pub const MACB_TX_PAUSE_ZERO_OFFSET: u64 = 3;
pub const MACB_TX_PAUSE_ZERO_SIZE: u64 = 1;

/* Bitfields in USRIO (AT91) */
pub const MACB_RMII_OFFSET: u64 = 0;
pub const MACB_RMII_SIZE: u64 = 1;
pub const GEM_RGMII_OFFSET: u64 = 0; /* GEM gigabit mode */
pub const GEM_RGMII_SIZE: u64 = 1;
pub const MACB_CLKEN_OFFSET: u64 = 1;
pub const MACB_CLKEN_SIZE: u64 = 1;

/* Bitfields in WOL */
pub const MACB_IP_OFFSET: u64 = 0;
pub const MACB_IP_SIZE: u64 = 16;
pub const MACB_MAG_OFFSET: u64 = 16;
pub const MACB_MAG_SIZE: u64 = 1;
pub const MACB_ARP_OFFSET: u64 = 17;
pub const MACB_ARP_SIZE: u64 = 1;
pub const MACB_SA1_OFFSET: u64 = 18;
pub const MACB_SA1_SIZE: u64 = 1;
pub const MACB_WOL_MTI_OFFSET: u64 = 19;
pub const MACB_WOL_MTI_SIZE: u64 = 1;

/* Bitfields in MID */
pub const MACB_IDNUM_OFFSET: u64 = 16;
pub const MACB_IDNUM_SIZE: u64 = 12;
pub const MACB_REV_OFFSET: u64 = 0;
pub const MACB_REV_SIZE: u64 = 16;

/* Bitfield in HS_MAC_CONFIG */
pub const GEM_HS_MAC_SPEED_OFFSET: u64 = 0;
pub const GEM_HS_MAC_SPEED_SIZE: u64 = 3;

/* Bitfields in PCSCNTRL */
pub const GEM_PCSAUTONEG_OFFSET: u64 = 12;
pub const GEM_PCSAUTONEG_SIZE: u64 = 1;

/* Bitfields in DCFG1. */
pub const GEM_IRQCOR_OFFSET: u64 = 23;
pub const GEM_IRQCOR_SIZE: u64 = 1;
pub const GEM_DBWDEF_OFFSET: u64 = 25;
pub const GEM_DBWDEF_SIZE: u64 = 3;
pub const GEM_NO_PCS_OFFSET: u64 = 0;
pub const GEM_NO_PCS_SIZE: u64 = 1;

/* Bitfields in DCFG2. */
pub const GEM_RX_PKT_BUFF_OFFSET: u64 = 20;
pub const GEM_RX_PKT_BUFF_SIZE: u64 = 1;
pub const GEM_TX_PKT_BUFF_OFFSET: u64 = 21;
pub const GEM_TX_PKT_BUFF_SIZE: u64 = 1;

/* Bitfields in DCFG5. */
pub const GEM_TSU_OFFSET: u64 = 8;
pub const GEM_TSU_SIZE: u64 = 1;

/* Bitfields in DCFG6. */
pub const GEM_PBUF_LSO_OFFSET: u64 = 27;
pub const GEM_PBUF_LSO_SIZE: u64 = 1;
pub const GEM_DAW64_OFFSET: u64 = 23;
pub const GEM_DAW64_SIZE: u64 = 1;

/* Bitfields in DCFG8. */
pub const GEM_T1SCR_OFFSET: u64 = 24;
pub const GEM_T1SCR_SIZE: u64 = 8;
pub const GEM_T2SCR_OFFSET: u64 = 16;
pub const GEM_T2SCR_SIZE: u64 = 8;
pub const GEM_SCR2ETH_OFFSET: u64 = 8;
pub const GEM_SCR2ETH_SIZE: u64 = 8;
pub const GEM_SCR2CMP_OFFSET: u64 = 0;
pub const GEM_SCR2CMP_SIZE: u64 = 8;

/* Bitfields in DCFG10 */
pub const GEM_TXBD_RDBUFF_OFFSET: u64 = 12;
pub const GEM_TXBD_RDBUFF_SIZE: u64 = 4;
pub const GEM_RXBD_RDBUFF_OFFSET: u64 = 8;
pub const GEM_RXBD_RDBUFF_SIZE: u64 = 4;

/* Bitfields in DCFG12. */
pub const GEM_HIGH_SPEED_OFFSET: u64 = 26;
pub const GEM_HIGH_SPEED_SIZE: u64 = 1;

/* Bitfields in USX_CONTROL. */
pub const GEM_USX_CTRL_SPEED_OFFSET: u64 = 14;
pub const GEM_USX_CTRL_SPEED_SIZE: u64 = 3;
pub const GEM_SERDES_RATE_OFFSET: u64 = 12;
pub const GEM_SERDES_RATE_SIZE: u64 = 2;
pub const GEM_RX_SCR_BYPASS_OFFSET: u64 = 9;
pub const GEM_RX_SCR_BYPASS_SIZE: u64 = 1;
pub const GEM_TX_SCR_BYPASS_OFFSET: u64 = 8;
pub const GEM_TX_SCR_BYPASS_SIZE: u64 = 1;
pub const GEM_TX_EN_OFFSET: u64 = 1;
pub const GEM_TX_EN_SIZE: u64 = 1;
pub const GEM_SIGNAL_OK_OFFSET: u64 = 0;
pub const GEM_SIGNAL_OK_SIZE: u64 = 1;

/* Bitfields in USX_STATUS. */
pub const GEM_USX_BLOCK_LOCK_OFFSET: u64 = 0;
pub const GEM_USX_BLOCK_LOCK_SIZE: u64 = 1;

/* Bitfields in TISUBN */
pub const GEM_SUBNSINCR_OFFSET: u64 = 0;
pub const GEM_SUBNSINCRL_OFFSET: u64 = 24;
pub const GEM_SUBNSINCRL_SIZE: u64 = 8;
pub const GEM_SUBNSINCRH_OFFSET: u64 = 0;
pub const GEM_SUBNSINCRH_SIZE: u64 = 16;
pub const GEM_SUBNSINCR_SIZE: u64 = 24;

/* Bitfields in TI */
pub const GEM_NSINCR_OFFSET: u64 = 0;
pub const GEM_NSINCR_SIZE: u64 = 8;

/* Bitfields in TSH */
pub const GEM_TSH_OFFSET: u64 = 0; /* TSU timer value (s). MSB [47:32] of seconds timer count */
pub const GEM_TSH_SIZE: u64 = 16;

/* Bitfields in TSL */
pub const GEM_TSL_OFFSET: u64 = 0; /* TSU timer value (s). LSB [31:0] of seconds timer count */
pub const GEM_TSL_SIZE: u64 = 32;

/* Bitfields in TN */
pub const GEM_TN_OFFSET: u64 = 0; /* TSU timer value (ns) */
pub const GEM_TN_SIZE: u64 = 30;

/* Bitfields in TXBDCTRL */
pub const GEM_TXTSMODE_OFFSET: u64 = 4; /* TX Descriptor Timestamp Insertion mode */
pub const GEM_TXTSMODE_SIZE: u64 = 2;

/* Bitfields in RXBDCTRL */
pub const GEM_RXTSMODE_OFFSET: u64 = 4; /* RX Descriptor Timestamp Insertion mode */
pub const GEM_RXTSMODE_SIZE: u64 = 2;

/* Bitfields in SCRT2 */
pub const GEM_QUEUE_OFFSET: u64 = 0; /* Queue Number */
pub const GEM_QUEUE_SIZE: u64 = 4;
pub const GEM_VLANPR_OFFSET: u64 = 4; /* VLAN Priority */
pub const GEM_VLANPR_SIZE: u64 = 3;
pub const GEM_VLANEN_OFFSET: u64 = 8; /* VLAN Enable */
pub const GEM_VLANEN_SIZE: u64 = 1;
pub const GEM_ETHT2IDX_OFFSET: u64 = 9; /* Index to screener type 2 EtherType register */
pub const GEM_ETHT2IDX_SIZE: u64 = 3;
pub const GEM_ETHTEN_OFFSET: u64 = 12; /* EtherType Enable */
pub const GEM_ETHTEN_SIZE: u64 = 1;
pub const GEM_CMPA_OFFSET: u64 = 13; /* Compare A - Index to screener type 2 Compare register */
pub const GEM_CMPA_SIZE: u64 = 5;
pub const GEM_CMPAEN_OFFSET: u64 = 18; /* Compare A Enable */
pub const GEM_CMPAEN_SIZE: u64 = 1;
pub const GEM_CMPB_OFFSET: u64 = 19; /* Compare B - Index to screener type 2 Compare register */
pub const GEM_CMPB_SIZE: u64 = 5;
pub const GEM_CMPBEN_OFFSET: u64 = 24; /* Compare B Enable */
pub const GEM_CMPBEN_SIZE: u64 = 1;
pub const GEM_CMPC_OFFSET: u64 = 25; /* Compare C - Index to screener type 2 Compare register */
pub const GEM_CMPC_SIZE: u64 = 5;
pub const GEM_CMPCEN_OFFSET: u64 = 30; /* Compare C Enable */
pub const GEM_CMPCEN_SIZE: u64 = 1;

/* Bitfields in ETHT */
pub const GEM_ETHTCMP_OFFSET: u64 = 0; /* EtherType compare value */
pub const GEM_ETHTCMP_SIZE: u64 = 16;

/* Bitfields in T2CMPW0 */
pub const GEM_T2CMP_OFFSET: u64 = 16; /* 0xFFFF0000 compare value */
pub const GEM_T2CMP_SIZE: u64 = 16;
pub const GEM_T2MASK_OFFSET: u64 = 0; /* 0x0000FFFF compare value or mask */
pub const GEM_T2MASK_SIZE: u64 = 16;

/* Bitfields in T2CMPW1 */
pub const GEM_T2DISMSK_OFFSET: u64 = 9; /* disable mask */
pub const GEM_T2DISMSK_SIZE: u64 = 1;
pub const GEM_T2CMPOFST_OFFSET: u64 = 7; /* compare offset */
pub const GEM_T2CMPOFST_SIZE: u64 = 2;
pub const GEM_T2OFST_OFFSET: u64 = 0; /* offset value */
pub const GEM_T2OFST_SIZE: u64 = 7;

/* Offset for screener type 2 compare values (T2CMPOFST).
 * Note the offset is applied after the specified point,
 * e.g. GEM_T2COMPOFST_ETYPE denotes the EtherType field, so an offset
 * of 12 bytes from this would be the source IP address in an IP header
 */
pub const GEM_T2COMPOFST_SOF: u64 = 0;
pub const GEM_T2COMPOFST_ETYPE: u64 = 1;
pub const GEM_T2COMPOFST_IPHDR: u64 = 2;
pub const GEM_T2COMPOFST_TCPUDP: u64 = 3;

/* offset from EtherType to IP address */
pub const ETYPE_SRCIP_OFFSET: u64 = 12;
pub const ETYPE_DSTIP_OFFSET: u64 = 16;

/* offset from IP header to port */
pub const IPHDR_SRCPORT_OFFSET: u64 = 0;
pub const IPHDR_DSTPORT_OFFSET: u64 = 2;

/* Transmit DMA buffer descriptor Word 1 */
pub const GEM_DMA_TXVALID_OFFSET: u64 = 23; /* timestamp has been captured in the Buffer Descriptor */
pub const GEM_DMA_TXVALID_SIZE: u64 = 1;

/* Receive DMA buffer descriptor Word 0 */
pub const GEM_DMA_RXVALID_OFFSET: u64 = 2; /* indicates a valid timestamp in the Buffer Descriptor */
pub const GEM_DMA_RXVALID_SIZE: u64 = 1;

/* DMA buffer descriptor Word 2 (32 bit addressing) or Word 4 (64 bit addressing) */
pub const GEM_DMA_SECL_OFFSET: u64 = 30; /* Timestamp seconds[1:0]  */
pub const GEM_DMA_SECL_SIZE: u64 = 2;
pub const GEM_DMA_NSEC_OFFSET: u64 = 0; /* Timestamp nanosecs [29:0] */
pub const GEM_DMA_NSEC_SIZE: u64 = 30;

/* DMA buffer descriptor Word 3 (32 bit addressing) or Word 5 (64 bit addressing) */

/* New hardware supports 12 bit precision of timestamp in DMA buffer descriptor.
 * Old hardware supports only 6 bit precision but it is enough for PTP.
 * Less accuracy is used always instead of checking hardware version.
 */
/*
pub const GEM_DMA_SECH_OFFSET			0 /* Timestamp seconds[5:2] */
pub const GEM_DMA_SECH_SIZE			4
pub const GEM_DMA_SEC_WIDTH			(GEM_DMA_SECH_SIZE + GEM_DMA_SECL_SIZE)
pub const GEM_DMA_SEC_TOP				(1 << GEM_DMA_SEC_WIDTH)
pub const GEM_DMA_SEC_MASK			(GEM_DMA_SEC_TOP - 1)
*/

/* Bitfields in ADJ */
pub const GEM_ADDSUB_OFFSET: u64 = 31;
pub const GEM_ADDSUB_SIZE: u64 = 1;
/* Constants for CLK */
pub const MACB_CLK_DIV8: u64 = 0;
pub const MACB_CLK_DIV16: u64 = 1;
pub const MACB_CLK_DIV32: u64 = 2;
pub const MACB_CLK_DIV64: u64 = 3;

/* GEM specific constants for CLK. */
pub const GEM_CLK_DIV8: u64 = 0;
pub const GEM_CLK_DIV16: u64 = 1;
pub const GEM_CLK_DIV32: u64 = 2;
pub const GEM_CLK_DIV48: u64 = 3;
pub const GEM_CLK_DIV64: u64 = 4;
pub const GEM_CLK_DIV96: u64 = 5;
pub const GEM_CLK_DIV128: u64 = 6;
pub const GEM_CLK_DIV224: u64 = 7;

/* Constants for MAN register */
pub const MACB_MAN_C22_SOF: u64 = 1;
pub const MACB_MAN_C22_WRITE: u64 = 1;
pub const MACB_MAN_C22_READ: u64 = 2;
pub const MACB_MAN_C22_CODE: u64 = 2;

pub const MACB_MAN_C45_SOF: u64 = 0;
pub const MACB_MAN_C45_ADDR: u64 = 0;
pub const MACB_MAN_C45_WRITE: u64 = 1;
pub const MACB_MAN_C45_POST_READ_INCR: u64 = 2;
pub const MACB_MAN_C45_READ: u64 = 3;
pub const MACB_MAN_C45_CODE: u64 = 2;

/* Capability mask bits */
pub const MACB_CAPS_ISR_CLEAR_ON_WRITE: u64 = 0x00000001;
pub const MACB_CAPS_USRIO_HAS_CLKEN: u64 = 0x00000002;
pub const MACB_CAPS_USRIO_DEFAULT_IS_MII_GMII: u64 = 0x00000004;
pub const MACB_CAPS_NO_GIGABIT_HALF: u64 = 0x00000008;
pub const MACB_CAPS_USRIO_DISABLED: u64 = 0x00000010;
pub const MACB_CAPS_JUMBO: u64 = 0x00000020;
pub const MACB_CAPS_GEM_HAS_PTP: u64 = 0x00000040;
pub const MACB_CAPS_BD_RD_PREFETCH: u64 = 0x00000080;
pub const MACB_CAPS_NEEDS_RSTONUBR: u64 = 0x00000100;
pub const MACB_CAPS_MIIONRGMII: u64 = 0x00000200;
pub const MACB_CAPS_CLK_HW_CHG: u64 = 0x04000000;
pub const MACB_CAPS_MACB_IS_EMAC: u64 = 0x08000000;
pub const MACB_CAPS_FIFO_MODE: u64 = 0x10000000;
pub const MACB_CAPS_GIGABIT_MODE_AVAILABLE: u64 = 0x20000000;
pub const MACB_CAPS_SG_DISABLED: u64 = 0x40000000;
pub const MACB_CAPS_MACB_IS_GEM: u64 = 0x80000000;
pub const MACB_CAPS_PCS: u64 = 0x01000000;
pub const MACB_CAPS_HIGH_SPEED: u64 = 0x02000000;

/* LSO settings */
pub const MACB_LSO_UFO_ENABLE: u64 = 0x01;
pub const MACB_LSO_TSO_ENABLE: u64 = 0x02;

/*
/* Bit manipulation macros */
pub const MACB_BIT(name)					\
    (1 << MACB_##name##_OFFSET)
pub const MACB_BF(name,value)				\
    (((value) & ((1 << MACB_##name##_SIZE) - 1))	\
     << MACB_##name##_OFFSET)
pub const MACB_BFEXT(name,value)\
    (((value) >> MACB_##name##_OFFSET)		\
     & ((1 << MACB_##name##_SIZE) - 1))
pub const MACB_BFINS(name,value,old)			\
    (((old) & ~(((1 << MACB_##name##_SIZE) - 1)	\
            << MACB_##name##_OFFSET))		\
     | MACB_BF(name,value))

pub const GEM_BIT(name)					\
    (1 << GEM_##name##_OFFSET)
pub const GEM_BF(name, value)				\
    (((value) & ((1 << GEM_##name##_SIZE) - 1))	\
     << GEM_##name##_OFFSET)
pub const GEM_BFEXT(name, value)\
    (((value) >> GEM_##name##_OFFSET)		\
     & ((1 << GEM_##name##_SIZE) - 1))
pub const GEM_BFINS(name, value, old)			\
    (((old) & ~(((1 << GEM_##name##_SIZE) - 1)	\
            << GEM_##name##_OFFSET))		\
     | GEM_BF(name, value))

/* Register access macros */
pub const macb_readl(port, reg)		(port)->macb_reg_readl((port), MACB_##reg)
pub const macb_writel(port, reg, value)	(port)->macb_reg_writel((port), MACB_##reg, (value))
pub const gem_readl(port, reg)		(port)->macb_reg_readl((port), GEM_##reg)
pub const gem_writel(port, reg, value)	(port)->macb_reg_writel((port), GEM_##reg, (value))
pub const queue_readl(queue, reg)		(queue)->bp->macb_reg_readl((queue)->bp, (queue)->reg)
pub const queue_writel(queue, reg, value)	(queue)->bp->macb_reg_writel((queue)->bp, (queue)->reg, (value))
pub const gem_readl_n(port, reg, idx)		(port)->macb_reg_readl((port), GEM_##reg + idx * 4)
pub const gem_writel_n(port, reg, idx, value)	(port)->macb_reg_writel((port), GEM_##reg + idx * 4, (value))

pub const PTP_TS_BUFFER_SIZE		128 /* must be power of 2 */

/* struct macb_dma_desc - Hardware DMA descriptor
 * @addr: DMA address of data buffer
 * @ctrl: Control and status bits
 */
struct macb_dma_desc {
    u32	addr;
    u32	ctrl;
};

#ifdef MACB_EXT_DESC
pub const HW_DMA_CAP_32B		0
pub const HW_DMA_CAP_64B		(1 << 0)
pub const HW_DMA_CAP_PTP		(1 << 1)
pub const HW_DMA_CAP_64B_PTP	(HW_DMA_CAP_64B | HW_DMA_CAP_PTP)

struct macb_dma_desc_64 {
    u32 addrh;
    u32 resvd;
};

struct macb_dma_desc_ptp {
    u32	ts_1;
    u32	ts_2;
};

struct gem_tx_ts {
    struct sk_buff *skb;
    struct macb_dma_desc_ptp desc_ptp;
};
#endif

*/
/* DMA descriptor bitfields */
pub const MACB_RX_USED_OFFSET: u64 = 0;
pub const MACB_RX_USED_SIZE: u64 = 1;
pub const MACB_RX_WRAP_OFFSET: u64 = 1;
pub const MACB_RX_WRAP_SIZE: u64 = 1;
pub const MACB_RX_WADDR_OFFSET: u64 = 2;
pub const MACB_RX_WADDR_SIZE: u64 = 30;

pub const MACB_RX_FRMLEN_OFFSET: u64 = 0;
pub const MACB_RX_FRMLEN_SIZE: u64 = 12;
pub const MACB_RX_OFFSET_OFFSET: u64 = 12;
pub const MACB_RX_OFFSET_SIZE: u64 = 2;
pub const MACB_RX_SOF_OFFSET: u64 = 14;
pub const MACB_RX_SOF_SIZE: u64 = 1;
pub const MACB_RX_EOF_OFFSET: u64 = 15;
pub const MACB_RX_EOF_SIZE: u64 = 1;
pub const MACB_RX_CFI_OFFSET: u64 = 16;
pub const MACB_RX_CFI_SIZE: u64 = 1;
pub const MACB_RX_VLAN_PRI_OFFSET: u64 = 17;
pub const MACB_RX_VLAN_PRI_SIZE: u64 = 3;
pub const MACB_RX_PRI_TAG_OFFSET: u64 = 20;
pub const MACB_RX_PRI_TAG_SIZE: u64 = 1;
pub const MACB_RX_VLAN_TAG_OFFSET: u64 = 21;
pub const MACB_RX_VLAN_TAG_SIZE: u64 = 1;
pub const MACB_RX_TYPEID_MATCH_OFFSET: u64 = 22;
pub const MACB_RX_TYPEID_MATCH_SIZE: u64 = 1;
pub const MACB_RX_SA4_MATCH_OFFSET: u64 = 23;
pub const MACB_RX_SA4_MATCH_SIZE: u64 = 1;
pub const MACB_RX_SA3_MATCH_OFFSET: u64 = 24;
pub const MACB_RX_SA3_MATCH_SIZE: u64 = 1;
pub const MACB_RX_SA2_MATCH_OFFSET: u64 = 25;
pub const MACB_RX_SA2_MATCH_SIZE: u64 = 1;
pub const MACB_RX_SA1_MATCH_OFFSET: u64 = 26;
pub const MACB_RX_SA1_MATCH_SIZE: u64 = 1;
pub const MACB_RX_EXT_MATCH_OFFSET: u64 = 28;
pub const MACB_RX_EXT_MATCH_SIZE: u64 = 1;
pub const MACB_RX_UHASH_MATCH_OFFSET: u64 = 29;
pub const MACB_RX_UHASH_MATCH_SIZE: u64 = 1;
pub const MACB_RX_MHASH_MATCH_OFFSET: u64 = 30;
pub const MACB_RX_MHASH_MATCH_SIZE: u64 = 1;
pub const MACB_RX_BROADCAST_OFFSET: u64 = 31;
pub const MACB_RX_BROADCAST_SIZE: u64 = 1;

pub const MACB_RX_FRMLEN_MASK: u64 = 0xFFF;
pub const MACB_RX_JFRMLEN_MASK: u64 = 0x3FFF;

/* RX checksum offload disabled: bit 24 clear in NCFGR */
pub const GEM_RX_TYPEID_MATCH_OFFSET: u64 = 22;
pub const GEM_RX_TYPEID_MATCH_SIZE: u64 = 2;

/* RX checksum offload enabled: bit 24 set in NCFGR */
pub const GEM_RX_CSUM_OFFSET: u64 = 22;
pub const GEM_RX_CSUM_SIZE: u64 = 2;

pub const MACB_TX_FRMLEN_OFFSET: u64 = 0;
pub const MACB_TX_FRMLEN_SIZE: u64 = 11;
pub const MACB_TX_LAST_OFFSET: u64 = 15;
pub const MACB_TX_LAST_SIZE: u64 = 1;
pub const MACB_TX_NOCRC_OFFSET: u64 = 16;
pub const MACB_TX_NOCRC_SIZE: u64 = 1;
pub const MACB_MSS_MFS_OFFSET: u64 = 16;
pub const MACB_MSS_MFS_SIZE: u64 = 14;
pub const MACB_TX_LSO_OFFSET: u64 = 17;
pub const MACB_TX_LSO_SIZE: u64 = 2;
pub const MACB_TX_TCP_SEQ_SRC_OFFSET: u64 = 19;
pub const MACB_TX_TCP_SEQ_SRC_SIZE: u64 = 1;
pub const MACB_TX_BUF_EXHAUSTED_OFFSET: u64 = 27;
pub const MACB_TX_BUF_EXHAUSTED_SIZE: u64 = 1;
pub const MACB_TX_UNDERRUN_OFFSET: u64 = 28;
pub const MACB_TX_UNDERRUN_SIZE: u64 = 1;
pub const MACB_TX_ERROR_OFFSET: u64 = 29;
pub const MACB_TX_ERROR_SIZE: u64 = 1;
pub const MACB_TX_WRAP_OFFSET: u64 = 30;
pub const MACB_TX_WRAP_SIZE: u64 = 1;
pub const MACB_TX_USED_OFFSET: u64 = 31;
pub const MACB_TX_USED_SIZE: u64 = 1;

pub const GEM_TX_FRMLEN_OFFSET: u64 = 0;
pub const GEM_TX_FRMLEN_SIZE: u64 = 14;

/* Buffer descriptor constants */
pub const GEM_RX_CSUM_NONE: u64 = 0;
pub const GEM_RX_CSUM_IP_ONLY: u64 = 1;
pub const GEM_RX_CSUM_IP_TCP: u64 = 2;
pub const GEM_RX_CSUM_IP_UDP: u64 = 3;

/* limit RX checksum offload to TCP and UDP packets */
pub const GEM_RX_CSUM_CHECKED_MASK: u64 = 2;

/* Scaled PPM fraction */
pub const PPM_FRACTION: u64 = 16;

/*
static inline bool macb_is_gem(struct macb *bp)
{
    return !!(bp->caps & MACB_CAPS_MACB_IS_GEM);
}

static inline bool gem_has_ptp(struct macb *bp)
{
    return !!(bp->caps & MACB_CAPS_GEM_HAS_PTP);
}
*/
