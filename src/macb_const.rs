//cadence/macb.h
//Atmel MACB Ethernet Controller driver

pub(crate) const MACB_GREGS_NBR: u32 = 16;
pub(crate) const MACB_GREGS_VERSION: u32 = 2;
pub(crate) const MACB_MAX_QUEUES: u32 = 8;

/* MACB register offsets */
pub(crate) const MACB_NCR: u32 = 0x0000; /* Network Control */
pub(crate) const MACB_NCFGR: u32 = 0x0004; /* Network Config */
pub(crate) const MACB_NSR: u32 = 0x0008; /* Network Status */
pub(crate) const MACB_TAR: u32 = 0x000c; /* AT91RM9200 only */
pub(crate) const MACB_TCR: u32 = 0x0010; /* AT91RM9200 only */
pub(crate) const MACB_TSR: u32 = 0x0014; /* Transmit Status */
pub(crate) const MACB_RBQP: u32 = 0x0018; /* RX Q Base Address */
pub(crate) const MACB_TBQP: u32 = 0x001c; /* TX Q Base Address */
pub(crate) const MACB_RSR: u32 = 0x0020; /* Receive Status */
pub(crate) const MACB_ISR: u32 = 0x0024; /* Interrupt Status */
pub(crate) const MACB_IER: u32 = 0x0028; /* Interrupt Enable */
pub(crate) const MACB_IDR: u32 = 0x002c; /* Interrupt Disable */
pub(crate) const MACB_IMR: u32 = 0x0030; /* Interrupt Mask */
pub(crate) const MACB_MAN: u32 = 0x0034; /* PHY Maintenance */
pub(crate) const MACB_PTR: u32 = 0x0038;
pub(crate) const MACB_PFR: u32 = 0x003c;
pub(crate) const MACB_FTO: u32 = 0x0040;
pub(crate) const MACB_SCF: u32 = 0x0044;
pub(crate) const MACB_MCF: u32 = 0x0048;
pub(crate) const MACB_FRO: u32 = 0x004c;
pub(crate) const MACB_FCSE: u32 = 0x0050;
pub(crate) const MACB_ALE: u32 = 0x0054;
pub(crate) const MACB_DTF: u32 = 0x0058;
pub(crate) const MACB_LCOL: u32 = 0x005c;
pub(crate) const MACB_EXCOL: u32 = 0x0060;
pub(crate) const MACB_TUND: u32 = 0x0064;
pub(crate) const MACB_CSE: u32 = 0x0068;
pub(crate) const MACB_RRE: u32 = 0x006c;
pub(crate) const MACB_ROVR: u32 = 0x0070;
pub(crate) const MACB_RSE: u32 = 0x0074;
pub(crate) const MACB_ELE: u32 = 0x0078;
pub(crate) const MACB_RJA: u32 = 0x007c;
pub(crate) const MACB_USF: u32 = 0x0080;
pub(crate) const MACB_STE: u32 = 0x0084;
pub(crate) const MACB_RLE: u32 = 0x0088;
pub(crate) const MACB_TPF: u32 = 0x008c;
pub(crate) const MACB_HRB: u32 = 0x0090;
pub(crate) const MACB_HRT: u32 = 0x0094;
pub(crate) const MACB_SA1B: u32 = 0x0098;
pub(crate) const MACB_SA1T: u32 = 0x009c;
pub(crate) const MACB_SA2B: u32 = 0x00a0;
pub(crate) const MACB_SA2T: u32 = 0x00a4;
pub(crate) const MACB_SA3B: u32 = 0x00a8;
pub(crate) const MACB_SA3T: u32 = 0x00ac;
pub(crate) const MACB_SA4B: u32 = 0x00b0;
pub(crate) const MACB_SA4T: u32 = 0x00b4;
pub(crate) const MACB_TID: u32 = 0x00b8;
pub(crate) const MACB_TPQ: u32 = 0x00bc;
pub(crate) const MACB_USRIO: u32 = 0x00c0;
pub(crate) const MACB_WOL: u32 = 0x00c4;
pub(crate) const MACB_MID: u32 = 0x00fc;
pub(crate) const MACB_TBQPH: u32 = 0x04C8;
pub(crate) const MACB_RBQPH: u32 = 0x04D4;

/* GEM register offsets. */
pub(crate) const GEM_NCR: u32 = 0x0000; /* Network Control */
pub(crate) const GEM_NCFGR: u32 = 0x0004; /* Network Config */
pub(crate) const GEM_USRIO: u32 = 0x000c; /* User IO */
pub(crate) const GEM_DMACFG: u32 = 0x0010; /* DMA Configuration */
pub(crate) const GEM_JML: u32 = 0x0048; /* Jumbo Max Length */
pub(crate) const GEM_HS_MAC_CONFIG: u32 = 0x0050; /* GEM high speed config */
pub(crate) const GEM_HRB: u32 = 0x0080; /* Hash Bottom */
pub(crate) const GEM_HRT: u32 = 0x0084; /* Hash Top */
pub(crate) const GEM_SA1B: u32 = 0x0088; /* Specific1 Bottom */
pub(crate) const GEM_SA1T: u32 = 0x008C; /* Specific1 Top */
pub(crate) const GEM_SA2B: u32 = 0x0090; /* Specific2 Bottom */
pub(crate) const GEM_SA2T: u32 = 0x0094; /* Specific2 Top */
pub(crate) const GEM_SA3B: u32 = 0x0098; /* Specific3 Bottom */
pub(crate) const GEM_SA3T: u32 = 0x009C; /* Specific3 Top */
pub(crate) const GEM_SA4B: u32 = 0x00A0; /* Specific4 Bottom */
pub(crate) const GEM_SA4T: u32 = 0x00A4; /* Specific4 Top */
pub(crate) const GEM_WOL: u32 = 0x00b8; /* Wake on LAN */
pub(crate) const GEM_EFTSH: u32 = 0x00e8; /* PTP Event Frame Transmitted Seconds Register 47:32 */
pub(crate) const GEM_EFRSH: u32 = 0x00ec; /* PTP Event Frame Received Seconds Register 47:32 */
pub(crate) const GEM_PEFTSH: u32 = 0x00f0; /* PTP Peer Event Frame Transmitted Seconds Register 47:32 */
pub(crate) const GEM_PEFRSH: u32 = 0x00f4; /* PTP Peer Event Frame Received Seconds Register 47:32 */
pub(crate) const GEM_OTX: u32 = 0x0100; /* Octets transmitted */
pub(crate) const GEM_OCTTXL: u32 = 0x0100; /* Octets transmitted [31:0] */
pub(crate) const GEM_OCTTXH: u32 = 0x0104; /* Octets transmitted [47:32] */
pub(crate) const GEM_TXCNT: u32 = 0x0108; /* Frames Transmitted counter */
pub(crate) const GEM_TXBCCNT: u32 = 0x010c; /* Broadcast Frames counter */
pub(crate) const GEM_TXMCCNT: u32 = 0x0110; /* Multicast Frames counter */
pub(crate) const GEM_TXPAUSECNT: u32 = 0x0114; /* Pause Frames Transmitted Counter */
pub(crate) const GEM_TX64CNT: u32 = 0x0118; /* 64 byte Frames TX counter */
pub(crate) const GEM_TX65CNT: u32 = 0x011c; /* 65-127 byte Frames TX counter */
pub(crate) const GEM_TX128CNT: u32 = 0x0120; /* 128-255 byte Frames TX counter */
pub(crate) const GEM_TX256CNT: u32 = 0x0124; /* 256-511 byte Frames TX counter */
pub(crate) const GEM_TX512CNT: u32 = 0x0128; /* 512-1023 byte Frames TX counter */
pub(crate) const GEM_TX1024CNT: u32 = 0x012c; /* 1024-1518 byte Frames TX counter */
pub(crate) const GEM_TX1519CNT: u32 = 0x0130; /* 1519+ byte Frames TX counter */
pub(crate) const GEM_TXURUNCNT: u32 = 0x0134; /* TX under run error counter */
pub(crate) const GEM_SNGLCOLLCNT: u32 = 0x0138; /* Single Collision Frame Counter */
pub(crate) const GEM_MULTICOLLCNT: u32 = 0x013c; /* Multiple Collision Frame Counter */
pub(crate) const GEM_EXCESSCOLLCNT: u32 = 0x0140; /* Excessive Collision Frame Counter */
pub(crate) const GEM_LATECOLLCNT: u32 = 0x0144; /* Late Collision Frame Counter */
pub(crate) const GEM_TXDEFERCNT: u32 = 0x0148; /* Deferred Transmission Frame Counter */
pub(crate) const GEM_TXCSENSECNT: u32 = 0x014c; /* Carrier Sense Error Counter */
pub(crate) const GEM_ORX: u32 = 0x0150; /* Octets received */
pub(crate) const GEM_OCTRXL: u32 = 0x0150; /* Octets received [31:0] */
pub(crate) const GEM_OCTRXH: u32 = 0x0154; /* Octets received [47:32] */
pub(crate) const GEM_RXCNT: u32 = 0x0158; /* Frames Received Counter */
pub(crate) const GEM_RXBROADCNT: u32 = 0x015c; /* Broadcast Frames Received Counter */
pub(crate) const GEM_RXMULTICNT: u32 = 0x0160; /* Multicast Frames Received Counter */
pub(crate) const GEM_RXPAUSECNT: u32 = 0x0164; /* Pause Frames Received Counter */
pub(crate) const GEM_RX64CNT: u32 = 0x0168; /* 64 byte Frames RX Counter */
pub(crate) const GEM_RX65CNT: u32 = 0x016c; /* 65-127 byte Frames RX Counter */
pub(crate) const GEM_RX128CNT: u32 = 0x0170; /* 128-255 byte Frames RX Counter */
pub(crate) const GEM_RX256CNT: u32 = 0x0174; /* 256-511 byte Frames RX Counter */
pub(crate) const GEM_RX512CNT: u32 = 0x0178; /* 512-1023 byte Frames RX Counter */
pub(crate) const GEM_RX1024CNT: u32 = 0x017c; /* 1024-1518 byte Frames RX Counter */
pub(crate) const GEM_RX1519CNT: u32 = 0x0180; /* 1519+ byte Frames RX Counter */
pub(crate) const GEM_RXUNDRCNT: u32 = 0x0184; /* Undersize Frames Received Counter */
pub(crate) const GEM_RXOVRCNT: u32 = 0x0188; /* Oversize Frames Received Counter */
pub(crate) const GEM_RXJABCNT: u32 = 0x018c; /* Jabbers Received Counter */
pub(crate) const GEM_RXFCSCNT: u32 = 0x0190; /* Frame Check Sequence Error Counter */
pub(crate) const GEM_RXLENGTHCNT: u32 = 0x0194; /* Length Field Error Counter */
pub(crate) const GEM_RXSYMBCNT: u32 = 0x0198; /* Symbol Error Counter */
pub(crate) const GEM_RXALIGNCNT: u32 = 0x019c; /* Alignment Error Counter */
pub(crate) const GEM_RXRESERRCNT: u32 = 0x01a0; /* Receive Resource Error Counter */
pub(crate) const GEM_RXORCNT: u32 = 0x01a4; /* Receive Overrun Counter */
pub(crate) const GEM_RXIPCCNT: u32 = 0x01a8; /* IP header Checksum Error Counter */
pub(crate) const GEM_RXTCPCCNT: u32 = 0x01ac; /* TCP Checksum Error Counter */
pub(crate) const GEM_RXUDPCCNT: u32 = 0x01b0; /* UDP Checksum Error Counter */
pub(crate) const GEM_TISUBN: u32 = 0x01bc; /* 1588 Timer Increment Sub-ns */
pub(crate) const GEM_TSH: u32 = 0x01c0; /* 1588 Timer Seconds High */
pub(crate) const GEM_TSL: u32 = 0x01d0; /* 1588 Timer Seconds Low */
pub(crate) const GEM_TN: u32 = 0x01d4; /* 1588 Timer Nanoseconds */
pub(crate) const GEM_TA: u32 = 0x01d8; /* 1588 Timer Adjust */
pub(crate) const GEM_TI: u32 = 0x01dc; /* 1588 Timer Increment */
pub(crate) const GEM_EFTSL: u32 = 0x01e0; /* PTP Event Frame Tx Seconds Low */
pub(crate) const GEM_EFTN: u32 = 0x01e4; /* PTP Event Frame Tx Nanoseconds */
pub(crate) const GEM_EFRSL: u32 = 0x01e8; /* PTP Event Frame Rx Seconds Low */
pub(crate) const GEM_EFRN: u32 = 0x01ec; /* PTP Event Frame Rx Nanoseconds */
pub(crate) const GEM_PEFTSL: u32 = 0x01f0; /* PTP Peer Event Frame Tx Secs Low */
pub(crate) const GEM_PEFTN: u32 = 0x01f4; /* PTP Peer Event Frame Tx Ns */
pub(crate) const GEM_PEFRSL: u32 = 0x01f8; /* PTP Peer Event Frame Rx Sec Low */
pub(crate) const GEM_PEFRN: u32 = 0x01fc; /* PTP Peer Event Frame Rx Ns */
pub(crate) const GEM_PCSCNTRL: u32 = 0x0200; /* PCS Control */
pub(crate) const GEM_PCSSTS: u32 = 0x0204; /* PCS Status */
pub(crate) const GEM_PCSPHYTOPID: u32 = 0x0208; /* PCS PHY Top ID */
pub(crate) const GEM_PCSPHYBOTID: u32 = 0x020c; /* PCS PHY Bottom ID */
pub(crate) const GEM_PCSANADV: u32 = 0x0210; /* PCS AN Advertisement */
pub(crate) const GEM_PCSANLPBASE: u32 = 0x0214; /* PCS AN Link Partner Base */
pub(crate) const GEM_PCSANEXP: u32 = 0x0218; /* PCS AN Expansion */
pub(crate) const GEM_PCSANNPTX: u32 = 0x021c; /* PCS AN Next Page TX */
pub(crate) const GEM_PCSANNPLP: u32 = 0x0220; /* PCS AN Next Page LP */
pub(crate) const GEM_PCSANEXTSTS: u32 = 0x023c; /* PCS AN Extended Status */
pub(crate) const GEM_DCFG1: u32 = 0x0280; /* Design Config 1 */
pub(crate) const GEM_DCFG2: u32 = 0x0284; /* Design Config 2 */
pub(crate) const GEM_DCFG3: u32 = 0x0288; /* Design Config 3 */
pub(crate) const GEM_DCFG4: u32 = 0x028c; /* Design Config 4 */
pub(crate) const GEM_DCFG5: u32 = 0x0290; /* Design Config 5 */
pub(crate) const GEM_DCFG6: u32 = 0x0294; /* Design Config 6 */
pub(crate) const GEM_DCFG7: u32 = 0x0298; /* Design Config 7 */
pub(crate) const GEM_DCFG8: u32 = 0x029C; /* Design Config 8 */
pub(crate) const GEM_DCFG10: u32 = 0x02A4; /* Design Config 10 */
pub(crate) const GEM_DCFG12: u32 = 0x02AC; /* Design Config 12 */
pub(crate) const GEM_USX_CONTROL: u32 = 0x0A80; /* High speed PCS control register */
pub(crate) const GEM_USX_STATUS: u32 = 0x0A88; /* High speed PCS status register */

pub(crate) const GEM_TXBDCTRL: u32 = 0x04cc; /* TX Buffer Descriptor control register */
pub(crate) const GEM_RXBDCTRL: u32 = 0x04d0; /* RX Buffer Descriptor control register */

/* Screener Type 2 match registers */
pub(crate) const GEM_SCRT2: u32 = 0x540;

/* EtherType registers */
pub(crate) const GEM_ETHT: u32 = 0x06E0;

/* Type 2 compare registers */
pub(crate) const GEM_T2CMPW0: u32 = 0x0700;
pub(crate) const GEM_T2CMPW1: u32 = 0x0704;
//pub(crate) const T2CMP_OFST(t2idx)	(t2idx * 2)

/*
/* type 2 compare registers
 * each location requires 3 compare regs
 */
pub(crate) const GEM_IP4SRC_CMP(idx)		(idx * 3)
pub(crate) const GEM_IP4DST_CMP(idx)		(idx * 3 + 1)
pub(crate) const GEM_PORT_CMP(idx)		(idx * 3 + 2)

/* Which screening type 2 EtherType register will be used (0 - 7) */
pub(crate) const SCRT2_ETHT		0

pub(crate) const GEM_ISR(hw_q)		(0x0400 + ((hw_q) << 2))
pub(crate) const GEM_TBQP(hw_q)		(0x0440 + ((hw_q) << 2))
pub(crate) const GEM_TBQPH(hw_q)		(0x04C8)
pub(crate) const GEM_RBQP(hw_q)		(0x0480 + ((hw_q) << 2))
pub(crate) const GEM_RBQS(hw_q)		(0x04A0 + ((hw_q) << 2))
pub(crate) const GEM_RBQPH(hw_q)		(0x04D4)
pub(crate) const GEM_IER(hw_q)		(0x0600 + ((hw_q) << 2))
pub(crate) const GEM_IDR(hw_q)		(0x0620 + ((hw_q) << 2))
pub(crate) const GEM_IMR(hw_q)		(0x0640 + ((hw_q) << 2))

*/

/* Bitfields in NCR */
pub(crate) const MACB_LB_OFFSET: u32 = 0; /* reserved */
pub(crate) const MACB_LB_SIZE: u32 = 1;
pub(crate) const MACB_LLB_OFFSET: u32 = 1; /* Loop back local */
pub(crate) const MACB_LLB_SIZE: u32 = 1;
pub(crate) const MACB_RE_OFFSET: u32 = 2; /* Receive enable */
pub(crate) const MACB_RE_SIZE: u32 = 1;
pub(crate) const MACB_TE_OFFSET: u32 = 3; /* Transmit enable */
pub(crate) const MACB_TE_SIZE: u32 = 1;
pub(crate) const MACB_MPE_OFFSET: u32 = 4; /* Management port enable */
pub(crate) const MACB_MPE_SIZE: u32 = 1;
pub(crate) const MACB_CLRSTAT_OFFSET: u32 = 5; /* Clear stats regs */
pub(crate) const MACB_CLRSTAT_SIZE: u32 = 1;
pub(crate) const MACB_INCSTAT_OFFSET: u32 = 6; /* Incremental stats regs */
pub(crate) const MACB_INCSTAT_SIZE: u32 = 1;
pub(crate) const MACB_WESTAT_OFFSET: u32 = 7; /* Write enable stats regs */
pub(crate) const MACB_WESTAT_SIZE: u32 = 1;
pub(crate) const MACB_BP_OFFSET: u32 = 8; /* Back pressure */
pub(crate) const MACB_BP_SIZE: u32 = 1;
pub(crate) const MACB_TSTART_OFFSET: u32 = 9; /* Start transmission */
pub(crate) const MACB_TSTART_SIZE: u32 = 1;
pub(crate) const MACB_THALT_OFFSET: u32 = 10; /* Transmit halt */
pub(crate) const MACB_THALT_SIZE: u32 = 1;
pub(crate) const MACB_NCR_TPF_OFFSET: u32 = 11; /* Transmit pause frame */
pub(crate) const MACB_NCR_TPF_SIZE: u32 = 1;
pub(crate) const MACB_TZQ_OFFSET: u32 = 12; /* Transmit zero quantum pause frame */
pub(crate) const MACB_TZQ_SIZE: u32 = 1;
pub(crate) const MACB_SRTSM_OFFSET: u32 = 15; /* Store Receive Timestamp to Memory */
pub(crate) const MACB_OSSMODE_OFFSET: u32 = 24; /* Enable One Step Synchro Mode */
pub(crate) const MACB_OSSMODE_SIZE: u32 = 1;
pub(crate) const MACB_MIIONRGMII_OFFSET: u32 = 28; /* MII Usage on RGMII Interface */
pub(crate) const MACB_MIIONRGMII_SIZE: u32 = 1;

/* Bitfields in NCFGR */
pub(crate) const MACB_SPD_OFFSET: u32 = 0; /* Speed */
pub(crate) const MACB_SPD_SIZE: u32 = 1;
pub(crate) const MACB_FD_OFFSET: u32 = 1; /* Full duplex */
pub(crate) const MACB_FD_SIZE: u32 = 1;
pub(crate) const MACB_BIT_RATE_OFFSET: u32 = 2; /* Discard non-VLAN frames */
pub(crate) const MACB_BIT_RATE_SIZE: u32 = 1;
pub(crate) const MACB_JFRAME_OFFSET: u32 = 3; /* reserved */
pub(crate) const MACB_JFRAME_SIZE: u32 = 1;
pub(crate) const MACB_CAF_OFFSET: u32 = 4; /* Copy all frames */
pub(crate) const MACB_CAF_SIZE: u32 = 1;
pub(crate) const MACB_NBC_OFFSET: u32 = 5; /* No broadcast */
pub(crate) const MACB_NBC_SIZE: u32 = 1;
pub(crate) const MACB_NCFGR_MTI_OFFSET: u32 = 6; /* Multicast hash enable */
pub(crate) const MACB_NCFGR_MTI_SIZE: u32 = 1;
pub(crate) const MACB_UNI_OFFSET: u32 = 7; /* Unicast hash enable */
pub(crate) const MACB_UNI_SIZE: u32 = 1;
pub(crate) const MACB_BIG_OFFSET: u32 = 8; /* Receive 1536 byte frames */
pub(crate) const MACB_BIG_SIZE: u32 = 1;
pub(crate) const MACB_EAE_OFFSET: u32 = 9; /* External address match enable */
pub(crate) const MACB_EAE_SIZE: u32 = 1;
pub(crate) const MACB_CLK_OFFSET: u32 = 10;
pub(crate) const MACB_CLK_SIZE: u32 = 2;
pub(crate) const MACB_RTY_OFFSET: u32 = 12; /* Retry test */
pub(crate) const MACB_RTY_SIZE: u32 = 1;
pub(crate) const MACB_PAE_OFFSET: u32 = 13; /* Pause enable */
pub(crate) const MACB_PAE_SIZE: u32 = 1;
pub(crate) const MACB_RM9200_RMII_OFFSET: u32 = 13; /* AT91RM9200 only */
pub(crate) const MACB_RM9200_RMII_SIZE: u32 = 1; /* AT91RM9200 only */
pub(crate) const MACB_RBOF_OFFSET: u32 = 14; /* Receive buffer offset */
pub(crate) const MACB_RBOF_SIZE: u32 = 2;
pub(crate) const MACB_RLCE_OFFSET: u32 = 16; /* Length field error frame discard */
pub(crate) const MACB_RLCE_SIZE: u32 = 1;
pub(crate) const MACB_DRFCS_OFFSET: u32 = 17; /* FCS remove */
pub(crate) const MACB_DRFCS_SIZE: u32 = 1;
pub(crate) const MACB_EFRHD_OFFSET: u32 = 18;
pub(crate) const MACB_EFRHD_SIZE: u32 = 1;
pub(crate) const MACB_IRXFCS_OFFSET: u32 = 19;
pub(crate) const MACB_IRXFCS_SIZE: u32 = 1;

/* GEM specific NCR bitfields. */
pub(crate) const GEM_ENABLE_HS_MAC_OFFSET: u32 = 31;
pub(crate) const GEM_ENABLE_HS_MAC_SIZE: u32 = 1;

/* GEM specific NCFGR bitfields. */
pub(crate) const GEM_FD_OFFSET: u32 = 1; /* Full duplex */
pub(crate) const GEM_FD_SIZE: u32 = 1;
pub(crate) const GEM_GBE_OFFSET: u32 = 10; /* Gigabit mode enable */
pub(crate) const GEM_GBE_SIZE: u32 = 1;
pub(crate) const GEM_PCSSEL_OFFSET: u32 = 11;
pub(crate) const GEM_PCSSEL_SIZE: u32 = 1;
pub(crate) const GEM_PAE_OFFSET: u32 = 13; /* Pause enable */
pub(crate) const GEM_PAE_SIZE: u32 = 1;
pub(crate) const GEM_CLK_OFFSET: u32 = 18; /* MDC clock division */
pub(crate) const GEM_CLK_SIZE: u32 = 3;
pub(crate) const GEM_DBW_OFFSET: u32 = 21; /* Data bus width */
pub(crate) const GEM_DBW_SIZE: u32 = 2;
pub(crate) const GEM_RXCOEN_OFFSET: u32 = 24;
pub(crate) const GEM_RXCOEN_SIZE: u32 = 1;
pub(crate) const GEM_SGMIIEN_OFFSET: u32 = 27;
pub(crate) const GEM_SGMIIEN_SIZE: u32 = 1;

/* Constants for data bus width. */
pub(crate) const GEM_DBW32: u32 = 0; /* 32 bit AMBA AHB data bus width */
pub(crate) const GEM_DBW64: u32 = 1; /* 64 bit AMBA AHB data bus width */
pub(crate) const GEM_DBW128: u32 = 2; /* 128 bit AMBA AHB data bus width */

/* Bitfields in DMACFG. */
pub(crate) const GEM_FBLDO_OFFSET: u32 = 0; /* fixed burst length for DMA */
pub(crate) const GEM_FBLDO_SIZE: u32 = 5;
pub(crate) const GEM_ENDIA_DESC_OFFSET: u32 = 6; /* endian swap mode for management descriptor access */
pub(crate) const GEM_ENDIA_DESC_SIZE: u32 = 1;
pub(crate) const GEM_ENDIA_PKT_OFFSET: u32 = 7; /* endian swap mode for packet data access */
pub(crate) const GEM_ENDIA_PKT_SIZE: u32 = 1;
pub(crate) const GEM_RXBMS_OFFSET: u32 = 8; /* RX packet buffer memory size select */
pub(crate) const GEM_RXBMS_SIZE: u32 = 2;
pub(crate) const GEM_TXPBMS_OFFSET: u32 = 10; /* TX packet buffer memory size select */
pub(crate) const GEM_TXPBMS_SIZE: u32 = 1;
pub(crate) const GEM_TXCOEN_OFFSET: u32 = 11; /* TX IP/TCP/UDP checksum gen offload */
pub(crate) const GEM_TXCOEN_SIZE: u32 = 1;
pub(crate) const GEM_RXBS_OFFSET: u32 = 16; /* DMA receive buffer size */
pub(crate) const GEM_RXBS_SIZE: u32 = 8;
pub(crate) const GEM_DDRP_OFFSET: u32 = 24; /* disc_when_no_ahb */
pub(crate) const GEM_DDRP_SIZE: u32 = 1;
pub(crate) const GEM_RXEXT_OFFSET: u32 = 28; /* RX extended Buffer Descriptor mode */
pub(crate) const GEM_RXEXT_SIZE: u32 = 1;
pub(crate) const GEM_TXEXT_OFFSET: u32 = 29; /* TX extended Buffer Descriptor mode */
pub(crate) const GEM_TXEXT_SIZE: u32 = 1;
pub(crate) const GEM_ADDR64_OFFSET: u32 = 30; /* Address bus width - 64b or 32b */
pub(crate) const GEM_ADDR64_SIZE: u32 = 1;

/* Bitfields in NSR */
pub(crate) const MACB_NSR_LINK_OFFSET: u32 = 0; /* pcs_link_state */
pub(crate) const MACB_NSR_LINK_SIZE: u32 = 1;
pub(crate) const MACB_MDIO_OFFSET: u32 = 1; /* status of the mdio_in pin */
pub(crate) const MACB_MDIO_SIZE: u32 = 1;
pub(crate) const MACB_IDLE_OFFSET: u32 = 2; /* The PHY management logic is idle */
pub(crate) const MACB_IDLE_SIZE: u32 = 1;

/* Bitfields in TSR */
pub(crate) const MACB_UBR_OFFSET: u32 = 0; /* Used bit read */
pub(crate) const MACB_UBR_SIZE: u32 = 1;
pub(crate) const MACB_COL_OFFSET: u32 = 1; /* Collision occurred */
pub(crate) const MACB_COL_SIZE: u32 = 1;
pub(crate) const MACB_TSR_RLE_OFFSET: u32 = 2; /* Retry limit exceeded */
pub(crate) const MACB_TSR_RLE_SIZE: u32 = 1;
pub(crate) const MACB_TGO_OFFSET: u32 = 3; /* Transmit go */
pub(crate) const MACB_TGO_SIZE: u32 = 1;
pub(crate) const MACB_BEX_OFFSET: u32 = 4; /* TX frame corruption due to AHB error */
pub(crate) const MACB_BEX_SIZE: u32 = 1;
pub(crate) const MACB_RM9200_BNQ_OFFSET: u32 = 4; /* AT91RM9200 only */
pub(crate) const MACB_RM9200_BNQ_SIZE: u32 = 1; /* AT91RM9200 only */
pub(crate) const MACB_COMP_OFFSET: u32 = 5; /* Trnasmit complete */
pub(crate) const MACB_COMP_SIZE: u32 = 1;
pub(crate) const MACB_UND_OFFSET: u32 = 6; /* Trnasmit under run */
pub(crate) const MACB_UND_SIZE: u32 = 1;

/* Bitfields in RSR */
pub(crate) const MACB_BNA_OFFSET: u32 = 0; /* Buffer not available */
pub(crate) const MACB_BNA_SIZE: u32 = 1;
pub(crate) const MACB_REC_OFFSET: u32 = 1; /* Frame received */
pub(crate) const MACB_REC_SIZE: u32 = 1;
pub(crate) const MACB_OVR_OFFSET: u32 = 2; /* Receive overrun */
pub(crate) const MACB_OVR_SIZE: u32 = 1;

/* Bitfields in ISR/IER/IDR/IMR */
pub(crate) const MACB_MFD_OFFSET: u32 = 0; /* Management frame sent */
pub(crate) const MACB_MFD_SIZE: u32 = 1;
pub(crate) const MACB_RCOMP_OFFSET: u32 = 1; /* Receive complete */
pub(crate) const MACB_RCOMP_SIZE: u32 = 1;
pub(crate) const MACB_RXUBR_OFFSET: u32 = 2; /* RX used bit read */
pub(crate) const MACB_RXUBR_SIZE: u32 = 1;
pub(crate) const MACB_TXUBR_OFFSET: u32 = 3; /* TX used bit read */
pub(crate) const MACB_TXUBR_SIZE: u32 = 1;
pub(crate) const MACB_ISR_TUND_OFFSET: u32 = 4; /* Enable TX buffer under run interrupt */
pub(crate) const MACB_ISR_TUND_SIZE: u32 = 1;
pub(crate) const MACB_ISR_RLE_OFFSET: u32 = 5; /* EN retry exceeded/late coll interrupt */
pub(crate) const MACB_ISR_RLE_SIZE: u32 = 1;
pub(crate) const MACB_TXERR_OFFSET: u32 = 6; /* EN TX frame corrupt from error interrupt */
pub(crate) const MACB_TXERR_SIZE: u32 = 1;
pub(crate) const MACB_RM9200_TBRE_OFFSET: u32 = 6; /* EN may send new frame interrupt (RM9200) */
pub(crate) const MACB_RM9200_TBRE_SIZE: u32 = 1;
pub(crate) const MACB_TCOMP_OFFSET: u32 = 7; /* Enable transmit complete interrupt */
pub(crate) const MACB_TCOMP_SIZE: u32 = 1;
pub(crate) const MACB_ISR_LINK_OFFSET: u32 = 9; /* Enable link change interrupt */
pub(crate) const MACB_ISR_LINK_SIZE: u32 = 1;
pub(crate) const MACB_ISR_ROVR_OFFSET: u32 = 10; /* Enable receive overrun interrupt */
pub(crate) const MACB_ISR_ROVR_SIZE: u32 = 1;
pub(crate) const MACB_HRESP_OFFSET: u32 = 11; /* Enable hrsep not OK interrupt */
pub(crate) const MACB_HRESP_SIZE: u32 = 1;
pub(crate) const MACB_PFR_OFFSET: u32 = 12; /* Enable pause frame w/ quantum interrupt */
pub(crate) const MACB_PFR_SIZE: u32 = 1;
pub(crate) const MACB_PTZ_OFFSET: u32 = 13; /* Enable pause time zero interrupt */
pub(crate) const MACB_PTZ_SIZE: u32 = 1;
pub(crate) const MACB_WOL_OFFSET: u32 = 14; /* Enable wake-on-lan interrupt */
pub(crate) const MACB_WOL_SIZE: u32 = 1;
pub(crate) const MACB_DRQFR_OFFSET: u32 = 18; /* PTP Delay Request Frame Received */
pub(crate) const MACB_DRQFR_SIZE: u32 = 1;
pub(crate) const MACB_SFR_OFFSET: u32 = 19; /* PTP Sync Frame Received */
pub(crate) const MACB_SFR_SIZE: u32 = 1;
pub(crate) const MACB_DRQFT_OFFSET: u32 = 20; /* PTP Delay Request Frame Transmitted */
pub(crate) const MACB_DRQFT_SIZE: u32 = 1;
pub(crate) const MACB_SFT_OFFSET: u32 = 21; /* PTP Sync Frame Transmitted */
pub(crate) const MACB_SFT_SIZE: u32 = 1;
pub(crate) const MACB_PDRQFR_OFFSET: u32 = 22; /* PDelay Request Frame Received */
pub(crate) const MACB_PDRQFR_SIZE: u32 = 1;
pub(crate) const MACB_PDRSFR_OFFSET: u32 = 23; /* PDelay Response Frame Received */
pub(crate) const MACB_PDRSFR_SIZE: u32 = 1;
pub(crate) const MACB_PDRQFT_OFFSET: u32 = 24; /* PDelay Request Frame Transmitted */
pub(crate) const MACB_PDRQFT_SIZE: u32 = 1;
pub(crate) const MACB_PDRSFT_OFFSET: u32 = 25; /* PDelay Response Frame Transmitted */
pub(crate) const MACB_PDRSFT_SIZE: u32 = 1;
pub(crate) const MACB_SRI_OFFSET: u32 = 26; /* TSU Seconds Register Increment */
pub(crate) const MACB_SRI_SIZE: u32 = 1;
pub(crate) const GEM_WOL_OFFSET: u32 = 28; /* Enable wake-on-lan interrupt */
pub(crate) const GEM_WOL_SIZE: u32 = 1;

/* Timer increment fields */
pub(crate) const MACB_TI_CNS_OFFSET: u32 = 0;
pub(crate) const MACB_TI_CNS_SIZE: u32 = 8;
pub(crate) const MACB_TI_ACNS_OFFSET: u32 = 8;
pub(crate) const MACB_TI_ACNS_SIZE: u32 = 8;
pub(crate) const MACB_TI_NIT_OFFSET: u32 = 16;
pub(crate) const MACB_TI_NIT_SIZE: u32 = 8;

/* Bitfields in MAN */
pub(crate) const MACB_DATA_OFFSET: u32 = 0; /* data */
pub(crate) const MACB_DATA_SIZE: u32 = 16;
pub(crate) const MACB_CODE_OFFSET: u32 = 16; /* Must be written to 10 */
pub(crate) const MACB_CODE_SIZE: u32 = 2;
pub(crate) const MACB_REGA_OFFSET: u32 = 18; /* Register address */
pub(crate) const MACB_REGA_SIZE: u32 = 5;
pub(crate) const MACB_PHYA_OFFSET: u32 = 23; /* PHY address */
pub(crate) const MACB_PHYA_SIZE: u32 = 5;
pub(crate) const MACB_RW_OFFSET: u32 = 28; /* Operation. 10 is read. 01 is write. */
pub(crate) const MACB_RW_SIZE: u32 = 2;
pub(crate) const MACB_SOF_OFFSET: u32 = 30; /* Must be written to 1 for Clause 22 */
pub(crate) const MACB_SOF_SIZE: u32 = 2;

/* Bitfields in USRIO (AVR32) */
pub(crate) const MACB_MII_OFFSET: u32 = 0;
pub(crate) const MACB_MII_SIZE: u32 = 1;
pub(crate) const MACB_EAM_OFFSET: u32 = 1;
pub(crate) const MACB_EAM_SIZE: u32 = 1;
pub(crate) const MACB_TX_PAUSE_OFFSET: u32 = 2;
pub(crate) const MACB_TX_PAUSE_SIZE: u32 = 1;
pub(crate) const MACB_TX_PAUSE_ZERO_OFFSET: u32 = 3;
pub(crate) const MACB_TX_PAUSE_ZERO_SIZE: u32 = 1;

/* Bitfields in USRIO (AT91) */
pub(crate) const MACB_RMII_OFFSET: u32 = 0;
pub(crate) const MACB_RMII_SIZE: u32 = 1;
pub(crate) const GEM_RGMII_OFFSET: u32 = 0; /* GEM gigabit mode */
pub(crate) const GEM_RGMII_SIZE: u32 = 1;
pub(crate) const MACB_CLKEN_OFFSET: u32 = 1;
pub(crate) const MACB_CLKEN_SIZE: u32 = 1;

/* Bitfields in WOL */
pub(crate) const MACB_IP_OFFSET: u32 = 0;
pub(crate) const MACB_IP_SIZE: u32 = 16;
pub(crate) const MACB_MAG_OFFSET: u32 = 16;
pub(crate) const MACB_MAG_SIZE: u32 = 1;
pub(crate) const MACB_ARP_OFFSET: u32 = 17;
pub(crate) const MACB_ARP_SIZE: u32 = 1;
pub(crate) const MACB_SA1_OFFSET: u32 = 18;
pub(crate) const MACB_SA1_SIZE: u32 = 1;
pub(crate) const MACB_WOL_MTI_OFFSET: u32 = 19;
pub(crate) const MACB_WOL_MTI_SIZE: u32 = 1;

/* Bitfields in MID */
pub(crate) const MACB_IDNUM_OFFSET: u32 = 16;
pub(crate) const MACB_IDNUM_SIZE: u32 = 12;
pub(crate) const MACB_REV_OFFSET: u32 = 0;
pub(crate) const MACB_REV_SIZE: u32 = 16;

/* Bitfield in HS_MAC_CONFIG */
pub(crate) const GEM_HS_MAC_SPEED_OFFSET: u32 = 0;
pub(crate) const GEM_HS_MAC_SPEED_SIZE: u32 = 3;

/* Bitfields in PCSCNTRL */
pub(crate) const GEM_PCSAUTONEG_OFFSET: u32 = 12;
pub(crate) const GEM_PCSAUTONEG_SIZE: u32 = 1;

/* Bitfields in DCFG1. */
pub(crate) const GEM_IRQCOR_OFFSET: u32 = 23;
pub(crate) const GEM_IRQCOR_SIZE: u32 = 1;
pub(crate) const GEM_DBWDEF_OFFSET: u32 = 25;
pub(crate) const GEM_DBWDEF_SIZE: u32 = 3;
pub(crate) const GEM_NO_PCS_OFFSET: u32 = 0;
pub(crate) const GEM_NO_PCS_SIZE: u32 = 1;

/* Bitfields in DCFG2. */
pub(crate) const GEM_RX_PKT_BUFF_OFFSET: u32 = 20;
pub(crate) const GEM_RX_PKT_BUFF_SIZE: u32 = 1;
pub(crate) const GEM_TX_PKT_BUFF_OFFSET: u32 = 21;
pub(crate) const GEM_TX_PKT_BUFF_SIZE: u32 = 1;

/* Bitfields in DCFG5. */
pub(crate) const GEM_TSU_OFFSET: u32 = 8;
pub(crate) const GEM_TSU_SIZE: u32 = 1;

/* Bitfields in DCFG6. */
pub(crate) const GEM_PBUF_LSO_OFFSET: u32 = 27;
pub(crate) const GEM_PBUF_LSO_SIZE: u32 = 1;
pub(crate) const GEM_DAW64_OFFSET: u32 = 23;
pub(crate) const GEM_DAW64_SIZE: u32 = 1;

/* Bitfields in DCFG8. */
pub(crate) const GEM_T1SCR_OFFSET: u32 = 24;
pub(crate) const GEM_T1SCR_SIZE: u32 = 8;
pub(crate) const GEM_T2SCR_OFFSET: u32 = 16;
pub(crate) const GEM_T2SCR_SIZE: u32 = 8;
pub(crate) const GEM_SCR2ETH_OFFSET: u32 = 8;
pub(crate) const GEM_SCR2ETH_SIZE: u32 = 8;
pub(crate) const GEM_SCR2CMP_OFFSET: u32 = 0;
pub(crate) const GEM_SCR2CMP_SIZE: u32 = 8;

/* Bitfields in DCFG10 */
pub(crate) const GEM_TXBD_RDBUFF_OFFSET: u32 = 12;
pub(crate) const GEM_TXBD_RDBUFF_SIZE: u32 = 4;
pub(crate) const GEM_RXBD_RDBUFF_OFFSET: u32 = 8;
pub(crate) const GEM_RXBD_RDBUFF_SIZE: u32 = 4;

/* Bitfields in DCFG12. */
pub(crate) const GEM_HIGH_SPEED_OFFSET: u32 = 26;
pub(crate) const GEM_HIGH_SPEED_SIZE: u32 = 1;

/* Bitfields in USX_CONTROL. */
pub(crate) const GEM_USX_CTRL_SPEED_OFFSET: u32 = 14;
pub(crate) const GEM_USX_CTRL_SPEED_SIZE: u32 = 3;
pub(crate) const GEM_SERDES_RATE_OFFSET: u32 = 12;
pub(crate) const GEM_SERDES_RATE_SIZE: u32 = 2;
pub(crate) const GEM_RX_SCR_BYPASS_OFFSET: u32 = 9;
pub(crate) const GEM_RX_SCR_BYPASS_SIZE: u32 = 1;
pub(crate) const GEM_TX_SCR_BYPASS_OFFSET: u32 = 8;
pub(crate) const GEM_TX_SCR_BYPASS_SIZE: u32 = 1;
pub(crate) const GEM_TX_EN_OFFSET: u32 = 1;
pub(crate) const GEM_TX_EN_SIZE: u32 = 1;
pub(crate) const GEM_SIGNAL_OK_OFFSET: u32 = 0;
pub(crate) const GEM_SIGNAL_OK_SIZE: u32 = 1;

/* Bitfields in USX_STATUS. */
pub(crate) const GEM_USX_BLOCK_LOCK_OFFSET: u32 = 0;
pub(crate) const GEM_USX_BLOCK_LOCK_SIZE: u32 = 1;

/* Bitfields in TISUBN */
pub(crate) const GEM_SUBNSINCR_OFFSET: u32 = 0;
pub(crate) const GEM_SUBNSINCRL_OFFSET: u32 = 24;
pub(crate) const GEM_SUBNSINCRL_SIZE: u32 = 8;
pub(crate) const GEM_SUBNSINCRH_OFFSET: u32 = 0;
pub(crate) const GEM_SUBNSINCRH_SIZE: u32 = 16;
pub(crate) const GEM_SUBNSINCR_SIZE: u32 = 24;

/* Bitfields in TI */
pub(crate) const GEM_NSINCR_OFFSET: u32 = 0;
pub(crate) const GEM_NSINCR_SIZE: u32 = 8;

/* Bitfields in TSH */
pub(crate) const GEM_TSH_OFFSET: u32 = 0; /* TSU timer value (s). MSB [47:32] of seconds timer count */
pub(crate) const GEM_TSH_SIZE: u32 = 16;

/* Bitfields in TSL */
pub(crate) const GEM_TSL_OFFSET: u32 = 0; /* TSU timer value (s). LSB [31:0] of seconds timer count */
pub(crate) const GEM_TSL_SIZE: u32 = 32;

/* Bitfields in TN */
pub(crate) const GEM_TN_OFFSET: u32 = 0; /* TSU timer value (ns) */
pub(crate) const GEM_TN_SIZE: u32 = 30;

/* Bitfields in TXBDCTRL */
pub(crate) const GEM_TXTSMODE_OFFSET: u32 = 4; /* TX Descriptor Timestamp Insertion mode */
pub(crate) const GEM_TXTSMODE_SIZE: u32 = 2;

/* Bitfields in RXBDCTRL */
pub(crate) const GEM_RXTSMODE_OFFSET: u32 = 4; /* RX Descriptor Timestamp Insertion mode */
pub(crate) const GEM_RXTSMODE_SIZE: u32 = 2;

/* Bitfields in SCRT2 */
pub(crate) const GEM_QUEUE_OFFSET: u32 = 0; /* Queue Number */
pub(crate) const GEM_QUEUE_SIZE: u32 = 4;
pub(crate) const GEM_VLANPR_OFFSET: u32 = 4; /* VLAN Priority */
pub(crate) const GEM_VLANPR_SIZE: u32 = 3;
pub(crate) const GEM_VLANEN_OFFSET: u32 = 8; /* VLAN Enable */
pub(crate) const GEM_VLANEN_SIZE: u32 = 1;
pub(crate) const GEM_ETHT2IDX_OFFSET: u32 = 9; /* Index to screener type 2 EtherType register */
pub(crate) const GEM_ETHT2IDX_SIZE: u32 = 3;
pub(crate) const GEM_ETHTEN_OFFSET: u32 = 12; /* EtherType Enable */
pub(crate) const GEM_ETHTEN_SIZE: u32 = 1;
pub(crate) const GEM_CMPA_OFFSET: u32 = 13; /* Compare A - Index to screener type 2 Compare register */
pub(crate) const GEM_CMPA_SIZE: u32 = 5;
pub(crate) const GEM_CMPAEN_OFFSET: u32 = 18; /* Compare A Enable */
pub(crate) const GEM_CMPAEN_SIZE: u32 = 1;
pub(crate) const GEM_CMPB_OFFSET: u32 = 19; /* Compare B - Index to screener type 2 Compare register */
pub(crate) const GEM_CMPB_SIZE: u32 = 5;
pub(crate) const GEM_CMPBEN_OFFSET: u32 = 24; /* Compare B Enable */
pub(crate) const GEM_CMPBEN_SIZE: u32 = 1;
pub(crate) const GEM_CMPC_OFFSET: u32 = 25; /* Compare C - Index to screener type 2 Compare register */
pub(crate) const GEM_CMPC_SIZE: u32 = 5;
pub(crate) const GEM_CMPCEN_OFFSET: u32 = 30; /* Compare C Enable */
pub(crate) const GEM_CMPCEN_SIZE: u32 = 1;

/* Bitfields in ETHT */
pub(crate) const GEM_ETHTCMP_OFFSET: u32 = 0; /* EtherType compare value */
pub(crate) const GEM_ETHTCMP_SIZE: u32 = 16;

/* Bitfields in T2CMPW0 */
pub(crate) const GEM_T2CMP_OFFSET: u32 = 16; /* 0xFFFF0000 compare value */
pub(crate) const GEM_T2CMP_SIZE: u32 = 16;
pub(crate) const GEM_T2MASK_OFFSET: u32 = 0; /* 0x0000FFFF compare value or mask */
pub(crate) const GEM_T2MASK_SIZE: u32 = 16;

/* Bitfields in T2CMPW1 */
pub(crate) const GEM_T2DISMSK_OFFSET: u32 = 9; /* disable mask */
pub(crate) const GEM_T2DISMSK_SIZE: u32 = 1;
pub(crate) const GEM_T2CMPOFST_OFFSET: u32 = 7; /* compare offset */
pub(crate) const GEM_T2CMPOFST_SIZE: u32 = 2;
pub(crate) const GEM_T2OFST_OFFSET: u32 = 0; /* offset value */
pub(crate) const GEM_T2OFST_SIZE: u32 = 7;

/* Offset for screener type 2 compare values (T2CMPOFST).
 * Note the offset is applied after the specified point,
 * e.g. GEM_T2COMPOFST_ETYPE denotes the EtherType field, so an offset
 * of 12 bytes from this would be the source IP address in an IP header
 */
pub(crate) const GEM_T2COMPOFST_SOF: u32 = 0;
pub(crate) const GEM_T2COMPOFST_ETYPE: u32 = 1;
pub(crate) const GEM_T2COMPOFST_IPHDR: u32 = 2;
pub(crate) const GEM_T2COMPOFST_TCPUDP: u32 = 3;

/* offset from EtherType to IP address */
pub(crate) const ETYPE_SRCIP_OFFSET: u32 = 12;
pub(crate) const ETYPE_DSTIP_OFFSET: u32 = 16;

/* offset from IP header to port */
pub(crate) const IPHDR_SRCPORT_OFFSET: u32 = 0;
pub(crate) const IPHDR_DSTPORT_OFFSET: u32 = 2;

/* Transmit DMA buffer descriptor Word 1 */
pub(crate) const GEM_DMA_TXVALID_OFFSET: u32 = 23; /* timestamp has been captured in the Buffer Descriptor */
pub(crate) const GEM_DMA_TXVALID_SIZE: u32 = 1;

/* Receive DMA buffer descriptor Word 0 */
pub(crate) const GEM_DMA_RXVALID_OFFSET: u32 = 2; /* indicates a valid timestamp in the Buffer Descriptor */
pub(crate) const GEM_DMA_RXVALID_SIZE: u32 = 1;

/* DMA buffer descriptor Word 2 (32 bit addressing) or Word 4 (64 bit addressing) */
pub(crate) const GEM_DMA_SECL_OFFSET: u32 = 30; /* Timestamp seconds[1:0]  */
pub(crate) const GEM_DMA_SECL_SIZE: u32 = 2;
pub(crate) const GEM_DMA_NSEC_OFFSET: u32 = 0; /* Timestamp nanosecs [29:0] */
pub(crate) const GEM_DMA_NSEC_SIZE: u32 = 30;

/* DMA buffer descriptor Word 3 (32 bit addressing) or Word 5 (64 bit addressing) */

/* New hardware supports 12 bit precision of timestamp in DMA buffer descriptor.
 * Old hardware supports only 6 bit precision but it is enough for PTP.
 * Less accuracy is used always instead of checking hardware version.
 */
/*
pub(crate) const GEM_DMA_SECH_OFFSET			0 /* Timestamp seconds[5:2] */
pub(crate) const GEM_DMA_SECH_SIZE			4
pub(crate) const GEM_DMA_SEC_WIDTH			(GEM_DMA_SECH_SIZE + GEM_DMA_SECL_SIZE)
pub(crate) const GEM_DMA_SEC_TOP				(1 << GEM_DMA_SEC_WIDTH)
pub(crate) const GEM_DMA_SEC_MASK			(GEM_DMA_SEC_TOP - 1)
*/

/* Bitfields in ADJ */
pub(crate) const GEM_ADDSUB_OFFSET: u32 = 31;
pub(crate) const GEM_ADDSUB_SIZE: u32 = 1;
/* Constants for CLK */
pub(crate) const MACB_CLK_DIV8: u32 = 0;
pub(crate) const MACB_CLK_DIV16: u32 = 1;
pub(crate) const MACB_CLK_DIV32: u32 = 2;
pub(crate) const MACB_CLK_DIV64: u32 = 3;

/* GEM specific constants for CLK. */
pub(crate) const GEM_CLK_DIV8: u32 = 0;
pub(crate) const GEM_CLK_DIV16: u32 = 1;
pub(crate) const GEM_CLK_DIV32: u32 = 2;
pub(crate) const GEM_CLK_DIV48: u32 = 3;
pub(crate) const GEM_CLK_DIV64: u32 = 4;
pub(crate) const GEM_CLK_DIV96: u32 = 5;
pub(crate) const GEM_CLK_DIV128: u32 = 6;
pub(crate) const GEM_CLK_DIV224: u32 = 7;

/* Constants for MAN register */
pub(crate) const MACB_MAN_C22_SOF: u32 = 1;
pub(crate) const MACB_MAN_C22_WRITE: u32 = 1;
pub(crate) const MACB_MAN_C22_READ: u32 = 2;
pub(crate) const MACB_MAN_C22_CODE: u32 = 2;

pub(crate) const MACB_MAN_C45_SOF: u32 = 0;
pub(crate) const MACB_MAN_C45_ADDR: u32 = 0;
pub(crate) const MACB_MAN_C45_WRITE: u32 = 1;
pub(crate) const MACB_MAN_C45_POST_READ_INCR: u32 = 2;
pub(crate) const MACB_MAN_C45_READ: u32 = 3;
pub(crate) const MACB_MAN_C45_CODE: u32 = 2;

/* Capability mask bits */
pub(crate) const MACB_CAPS_ISR_CLEAR_ON_WRITE: u32 = 0x00000001;
pub(crate) const MACB_CAPS_USRIO_HAS_CLKEN: u32 = 0x00000002;
pub(crate) const MACB_CAPS_USRIO_DEFAULT_IS_MII_GMII: u32 = 0x00000004;
pub(crate) const MACB_CAPS_NO_GIGABIT_HALF: u32 = 0x00000008;
pub(crate) const MACB_CAPS_USRIO_DISABLED: u32 = 0x00000010;
pub(crate) const MACB_CAPS_JUMBO: u32 = 0x00000020;
pub(crate) const MACB_CAPS_GEM_HAS_PTP: u32 = 0x00000040;
pub(crate) const MACB_CAPS_BD_RD_PREFETCH: u32 = 0x00000080;
pub(crate) const MACB_CAPS_NEEDS_RSTONUBR: u32 = 0x00000100;
pub(crate) const MACB_CAPS_MIIONRGMII: u32 = 0x00000200;
pub(crate) const MACB_CAPS_CLK_HW_CHG: u32 = 0x04000000;
pub(crate) const MACB_CAPS_MACB_IS_EMAC: u32 = 0x08000000;
pub(crate) const MACB_CAPS_FIFO_MODE: u32 = 0x10000000;
pub(crate) const MACB_CAPS_GIGABIT_MODE_AVAILABLE: u32 = 0x20000000;
pub(crate) const MACB_CAPS_SG_DISABLED: u32 = 0x40000000;
pub(crate) const MACB_CAPS_MACB_IS_GEM: u32 = 0x80000000;
pub(crate) const MACB_CAPS_PCS: u32 = 0x01000000;
pub(crate) const MACB_CAPS_HIGH_SPEED: u32 = 0x02000000;

/* LSO settings */
pub(crate) const MACB_LSO_UFO_ENABLE: u32 = 0x01;
pub(crate) const MACB_LSO_TSO_ENABLE: u32 = 0x02;

/*
/* Bit manipulation macros */
pub(crate) const MACB_BIT(name)					\
    (1 << MACB_##name##_OFFSET)
pub(crate) const MACB_BF(name,value)				\
    (((value) & ((1 << MACB_##name##_SIZE) - 1))	\
     << MACB_##name##_OFFSET)
pub(crate) const MACB_BFEXT(name,value)\
    (((value) >> MACB_##name##_OFFSET)		\
     & ((1 << MACB_##name##_SIZE) - 1))
pub(crate) const MACB_BFINS(name,value,old)			\
    (((old) & ~(((1 << MACB_##name##_SIZE) - 1)	\
            << MACB_##name##_OFFSET))		\
     | MACB_BF(name,value))

pub(crate) const GEM_BIT(name)					\
    (1 << GEM_##name##_OFFSET)
pub(crate) const GEM_BF(name, value)				\
    (((value) & ((1 << GEM_##name##_SIZE) - 1))	\
     << GEM_##name##_OFFSET)
pub(crate) const GEM_BFEXT(name, value)\
    (((value) >> GEM_##name##_OFFSET)		\
     & ((1 << GEM_##name##_SIZE) - 1))
pub(crate) const GEM_BFINS(name, value, old)			\
    (((old) & ~(((1 << GEM_##name##_SIZE) - 1)	\
            << GEM_##name##_OFFSET))		\
     | GEM_BF(name, value))

/* Register access macros */
pub(crate) const macb_readl(port, reg)		(port)->macb_reg_readl((port), MACB_##reg)
pub(crate) const macb_writel(port, reg, value)	(port)->macb_reg_writel((port), MACB_##reg, (value))
pub(crate) const gem_readl(port, reg)		(port)->macb_reg_readl((port), GEM_##reg)
pub(crate) const gem_writel(port, reg, value)	(port)->macb_reg_writel((port), GEM_##reg, (value))
pub(crate) const queue_readl(queue, reg)		(queue)->bp->macb_reg_readl((queue)->bp, (queue)->reg)
pub(crate) const queue_writel(queue, reg, value)	(queue)->bp->macb_reg_writel((queue)->bp, (queue)->reg, (value))
pub(crate) const gem_readl_n(port, reg, idx)		(port)->macb_reg_readl((port), GEM_##reg + idx * 4)
pub(crate) const gem_writel_n(port, reg, idx, value)	(port)->macb_reg_writel((port), GEM_##reg + idx * 4, (value))

pub(crate) const PTP_TS_BUFFER_SIZE		128 /* must be power of 2 */

/* struct macb_dma_desc - Hardware DMA descriptor
 * @addr: DMA address of data buffer
 * @ctrl: Control and status bits
 */
struct macb_dma_desc {
    u32	addr;
    u32	ctrl;
};

#ifdef MACB_EXT_DESC
pub(crate) const HW_DMA_CAP_32B		0
pub(crate) const HW_DMA_CAP_64B		(1 << 0)
pub(crate) const HW_DMA_CAP_PTP		(1 << 1)
pub(crate) const HW_DMA_CAP_64B_PTP	(HW_DMA_CAP_64B | HW_DMA_CAP_PTP)

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
pub(crate) const MACB_RX_USED_OFFSET: u32 = 0;
pub(crate) const MACB_RX_USED_SIZE: u32 = 1;
pub(crate) const MACB_RX_WRAP_OFFSET: u32 = 1;
pub(crate) const MACB_RX_WRAP_SIZE: u32 = 1;
pub(crate) const MACB_RX_WADDR_OFFSET: u32 = 2;
pub(crate) const MACB_RX_WADDR_SIZE: u32 = 30;

pub(crate) const MACB_RX_FRMLEN_OFFSET: u32 = 0;
pub(crate) const MACB_RX_FRMLEN_SIZE: u32 = 12;
pub(crate) const MACB_RX_OFFSET_OFFSET: u32 = 12;
pub(crate) const MACB_RX_OFFSET_SIZE: u32 = 2;
pub(crate) const MACB_RX_SOF_OFFSET: u32 = 14;
pub(crate) const MACB_RX_SOF_SIZE: u32 = 1;
pub(crate) const MACB_RX_EOF_OFFSET: u32 = 15;
pub(crate) const MACB_RX_EOF_SIZE: u32 = 1;
pub(crate) const MACB_RX_CFI_OFFSET: u32 = 16;
pub(crate) const MACB_RX_CFI_SIZE: u32 = 1;
pub(crate) const MACB_RX_VLAN_PRI_OFFSET: u32 = 17;
pub(crate) const MACB_RX_VLAN_PRI_SIZE: u32 = 3;
pub(crate) const MACB_RX_PRI_TAG_OFFSET: u32 = 20;
pub(crate) const MACB_RX_PRI_TAG_SIZE: u32 = 1;
pub(crate) const MACB_RX_VLAN_TAG_OFFSET: u32 = 21;
pub(crate) const MACB_RX_VLAN_TAG_SIZE: u32 = 1;
pub(crate) const MACB_RX_TYPEID_MATCH_OFFSET: u32 = 22;
pub(crate) const MACB_RX_TYPEID_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_SA4_MATCH_OFFSET: u32 = 23;
pub(crate) const MACB_RX_SA4_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_SA3_MATCH_OFFSET: u32 = 24;
pub(crate) const MACB_RX_SA3_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_SA2_MATCH_OFFSET: u32 = 25;
pub(crate) const MACB_RX_SA2_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_SA1_MATCH_OFFSET: u32 = 26;
pub(crate) const MACB_RX_SA1_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_EXT_MATCH_OFFSET: u32 = 28;
pub(crate) const MACB_RX_EXT_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_UHASH_MATCH_OFFSET: u32 = 29;
pub(crate) const MACB_RX_UHASH_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_MHASH_MATCH_OFFSET: u32 = 30;
pub(crate) const MACB_RX_MHASH_MATCH_SIZE: u32 = 1;
pub(crate) const MACB_RX_BROADCAST_OFFSET: u32 = 31;
pub(crate) const MACB_RX_BROADCAST_SIZE: u32 = 1;

pub(crate) const MACB_RX_FRMLEN_MASK: u32 = 0xFFF;
pub(crate) const MACB_RX_JFRMLEN_MASK: u32 = 0x3FFF;

/* RX checksum offload disabled: bit 24 clear in NCFGR */
pub(crate) const GEM_RX_TYPEID_MATCH_OFFSET: u32 = 22;
pub(crate) const GEM_RX_TYPEID_MATCH_SIZE: u32 = 2;

/* RX checksum offload enabled: bit 24 set in NCFGR */
pub(crate) const GEM_RX_CSUM_OFFSET: u32 = 22;
pub(crate) const GEM_RX_CSUM_SIZE: u32 = 2;

pub(crate) const MACB_TX_FRMLEN_OFFSET: u32 = 0;
pub(crate) const MACB_TX_FRMLEN_SIZE: u32 = 11;
pub(crate) const MACB_TX_LAST_OFFSET: u32 = 15;
pub(crate) const MACB_TX_LAST_SIZE: u32 = 1;
pub(crate) const MACB_TX_NOCRC_OFFSET: u32 = 16;
pub(crate) const MACB_TX_NOCRC_SIZE: u32 = 1;
pub(crate) const MACB_MSS_MFS_OFFSET: u32 = 16;
pub(crate) const MACB_MSS_MFS_SIZE: u32 = 14;
pub(crate) const MACB_TX_LSO_OFFSET: u32 = 17;
pub(crate) const MACB_TX_LSO_SIZE: u32 = 2;
pub(crate) const MACB_TX_TCP_SEQ_SRC_OFFSET: u32 = 19;
pub(crate) const MACB_TX_TCP_SEQ_SRC_SIZE: u32 = 1;
pub(crate) const MACB_TX_BUF_EXHAUSTED_OFFSET: u32 = 27;
pub(crate) const MACB_TX_BUF_EXHAUSTED_SIZE: u32 = 1;
pub(crate) const MACB_TX_UNDERRUN_OFFSET: u32 = 28;
pub(crate) const MACB_TX_UNDERRUN_SIZE: u32 = 1;
pub(crate) const MACB_TX_ERROR_OFFSET: u32 = 29;
pub(crate) const MACB_TX_ERROR_SIZE: u32 = 1;
pub(crate) const MACB_TX_WRAP_OFFSET: u32 = 30;
pub(crate) const MACB_TX_WRAP_SIZE: u32 = 1;
pub(crate) const MACB_TX_USED_OFFSET: u32 = 31;
pub(crate) const MACB_TX_USED_SIZE: u32 = 1;

pub(crate) const GEM_TX_FRMLEN_OFFSET: u32 = 0;
pub(crate) const GEM_TX_FRMLEN_SIZE: u32 = 14;

/* Buffer descriptor constants */
pub(crate) const GEM_RX_CSUM_NONE: u32 = 0;
pub(crate) const GEM_RX_CSUM_IP_ONLY: u32 = 1;
pub(crate) const GEM_RX_CSUM_IP_TCP: u32 = 2;
pub(crate) const GEM_RX_CSUM_IP_UDP: u32 = 3;

/* limit RX checksum offload to TCP and UDP packets */
pub(crate) const GEM_RX_CSUM_CHECKED_MASK: u32 = 2;

/* Scaled PPM fraction */
pub(crate) const PPM_FRACTION: u32 = 16;

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