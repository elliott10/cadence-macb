// linux/mii.h: definitions for MII-compatible transceivers

/* Generic MII registers. */
pub const MII_BMCR: u64 = 0x00; /* Basic mode control register */
pub const MII_BMSR: u64 = 0x01; /* Basic mode status register  */
pub const MII_PHYSID1: u64 = 0x02; /* PHYS ID 1                   */
pub const MII_PHYSID2: u64 = 0x03; /* PHYS ID 2                   */
pub const MII_ADVERTISE: u64 = 0x04; /* Advertisement control reg   */
pub const MII_LPA: u64 = 0x05; /* Link partner ability reg    */
pub const MII_EXPANSION: u64 = 0x06; /* Expansion register          */
pub const MII_CTRL1000: u64 = 0x09; /* 1000BASE-T control          */
pub const MII_STAT1000: u64 = 0x0a; /* 1000BASE-T status           */
pub const MII_MMD_CTRL: u64 = 0x0d; /* MMD Access Control Register */
pub const MII_MMD_DATA: u64 = 0x0e; /* MMD Access Data Register */
pub const MII_ESTATUS: u64 = 0x0f; /* Extended Status             */
pub const MII_DCOUNTER: u64 = 0x12; /* Disconnect counter          */
pub const MII_FCSCOUNTER: u64 = 0x13; /* False carrier counter       */
pub const MII_NWAYTEST: u64 = 0x14; /* N-way auto-neg test reg     */
pub const MII_RERRCOUNTER: u64 = 0x15; /* Receive error counter       */
pub const MII_SREVISION: u64 = 0x16; /* Silicon revision            */
pub const MII_RESV1: u64 = 0x17; /* Reserved...                 */
pub const MII_LBRERROR: u64 = 0x18; /* Lpback, rx, bypass error    */
pub const MII_PHYADDR: u64 = 0x19; /* PHY address                 */
pub const MII_RESV2: u64 = 0x1a; /* Reserved...                 */
pub const MII_TPISTATUS: u64 = 0x1b; /* TPI status for 10mbps       */
pub const MII_NCONFIG: u64 = 0x1c; /* Network interface config    */

/* Basic mode control register. */
pub const BMCR_RESV: u64 = 0x003f; /* Unused...                   */
pub const BMCR_SPEED1000: u64 = 0x0040; /* MSB of Speed (1000)         */
pub const BMCR_CTST: u64 = 0x0080; /* Collision test              */
pub const BMCR_FULLDPLX: u64 = 0x0100; /* Full duplex                 */
pub const BMCR_ANRESTART: u64 = 0x0200; /* Auto negotiation restart    */
pub const BMCR_ISOLATE: u64 = 0x0400; /* Isolate data paths from MII */
pub const BMCR_PDOWN: u64 = 0x0800; /* Enable low power state      */
pub const BMCR_ANENABLE: u64 = 0x1000; /* Enable auto negotiation     */
pub const BMCR_SPEED100: u64 = 0x2000; /* Select 100Mbps              */
pub const BMCR_LOOPBACK: u64 = 0x4000; /* TXD loopback bits           */
pub const BMCR_RESET: u64 = 0x8000; /* Reset to default state      */
pub const BMCR_SPEED10: u64 = 0x0000; /* Select 10Mbps               */

/* Basic mode status register. */
pub const BMSR_ERCAP: u64 = 0x0001; /* Ext-reg capability          */
pub const BMSR_JCD: u64 = 0x0002; /* Jabber detected             */
pub const BMSR_LSTATUS: u64 = 0x0004; /* Link status                 */
pub const BMSR_ANEGCAPABLE: u64 = 0x0008; /* Able to do auto-negotiation */
pub const BMSR_RFAULT: u64 = 0x0010; /* Remote fault detected       */
pub const BMSR_ANEGCOMPLETE: u64 = 0x0020; /* Auto-negotiation complete   */
pub const BMSR_RESV: u64 = 0x00c0; /* Unused...                   */
pub const BMSR_ESTATEN: u64 = 0x0100; /* Extended Status in R15      */
pub const BMSR_100HALF2: u64 = 0x0200; /* Can do 100BASE-T2 HDX       */
pub const BMSR_100FULL2: u64 = 0x0400; /* Can do 100BASE-T2 FDX       */
pub const BMSR_10HALF: u64 = 0x0800; /* Can do 10mbps, half-duplex  */
pub const BMSR_10FULL: u64 = 0x1000; /* Can do 10mbps, full-duplex  */
pub const BMSR_100HALF: u64 = 0x2000; /* Can do 100mbps, half-duplex */
pub const BMSR_100FULL: u64 = 0x4000; /* Can do 100mbps, full-duplex */
pub const BMSR_100BASE4: u64 = 0x8000; /* Can do 100mbps, 4k packets  */

/* Advertisement control register. */
pub const ADVERTISE_SLCT: u64 = 0x001f; /* Selector bits               */
pub const ADVERTISE_CSMA: u64 = 0x0001; /* Only selector supported     */
pub const ADVERTISE_10HALF: u64 = 0x0020; /* Try for 10mbps half-duplex  */
pub const ADVERTISE_1000XFULL: u64 = 0x0020; /* Try for 1000BASE-X full-duplex */
pub const ADVERTISE_10FULL: u64 = 0x0040; /* Try for 10mbps full-duplex  */
pub const ADVERTISE_1000XHALF: u64 = 0x0040; /* Try for 1000BASE-X half-duplex */
pub const ADVERTISE_100HALF: u64 = 0x0080; /* Try for 100mbps half-duplex */
pub const ADVERTISE_1000XPAUSE: u64 = 0x0080; /* Try for 1000BASE-X pause    */
pub const ADVERTISE_100FULL: u64 = 0x0100; /* Try for 100mbps full-duplex */
pub const ADVERTISE_1000XPSE_ASYM: u64 = 0x0100; /* Try for 1000BASE-X asym pause */
pub const ADVERTISE_100BASE4: u64 = 0x0200; /* Try for 100mbps 4k packets  */
pub const ADVERTISE_PAUSE_CAP: u64 = 0x0400; /* Try for pause               */
pub const ADVERTISE_PAUSE_ASYM: u64 = 0x0800; /* Try for asymetric pause     */
pub const ADVERTISE_RESV: u64 = 0x1000; /* Unused...                   */
pub const ADVERTISE_RFAULT: u64 = 0x2000; /* Say we can detect faults    */
pub const ADVERTISE_LPACK: u64 = 0x4000; /* Ack link partners response  */
pub const ADVERTISE_NPAGE: u64 = 0x8000; /* Next page bit               */

pub const ADVERTISE_FULL: u64 = (ADVERTISE_100FULL | ADVERTISE_10FULL | ADVERTISE_CSMA);
pub const ADVERTISE_ALL: u64 =
    (ADVERTISE_10HALF | ADVERTISE_10FULL | ADVERTISE_100HALF | ADVERTISE_100FULL);

/* Link partner ability register. */
pub const LPA_SLCT: u64 = 0x001f; /* Same as advertise selector  */
pub const LPA_10HALF: u64 = 0x0020; /* Can do 10mbps half-duplex   */
pub const LPA_1000XFULL: u64 = 0x0020; /* Can do 1000BASE-X full-duplex */
pub const LPA_10FULL: u64 = 0x0040; /* Can do 10mbps full-duplex   */
pub const LPA_1000XHALF: u64 = 0x0040; /* Can do 1000BASE-X half-duplex */
pub const LPA_100HALF: u64 = 0x0080; /* Can do 100mbps half-duplex  */
pub const LPA_1000XPAUSE: u64 = 0x0080; /* Can do 1000BASE-X pause     */
pub const LPA_100FULL: u64 = 0x0100; /* Can do 100mbps full-duplex  */
pub const LPA_1000XPAUSE_ASYM: u64 = 0x0100; /* Can do 1000BASE-X pause asym*/
pub const LPA_100BASE4: u64 = 0x0200; /* Can do 100mbps 4k packets   */
pub const LPA_PAUSE_CAP: u64 = 0x0400; /* Can pause                   */
pub const LPA_PAUSE_ASYM: u64 = 0x0800; /* Can pause asymetrically     */
pub const LPA_RESV: u64 = 0x1000; /* Unused...                   */
pub const LPA_RFAULT: u64 = 0x2000; /* Link partner faulted        */
pub const LPA_LPACK: u64 = 0x4000; /* Link partner acked us       */
pub const LPA_NPAGE: u64 = 0x8000; /* Next page bit               */

pub const LPA_DUPLEX: u64 = (LPA_10FULL | LPA_100FULL);
pub const LPA_100: u64 = (LPA_100FULL | LPA_100HALF | LPA_100BASE4);

/* Expansion register for auto-negotiation. */
pub const EXPANSION_NWAY: u64 = 0x0001; /* Can do N-way auto-nego      */
pub const EXPANSION_LCWP: u64 = 0x0002; /* Got new RX page code word   */
pub const EXPANSION_ENABLENPAGE: u64 = 0x0004; /* This enables npage words    */
pub const EXPANSION_NPCAPABLE: u64 = 0x0008; /* Link partner supports npage */
pub const EXPANSION_MFAULTS: u64 = 0x0010; /* Multiple faults detected    */
pub const EXPANSION_RESV: u64 = 0xffe0; /* Unused...                   */

pub const ESTATUS_1000_XFULL: u64 = 0x8000; /* Can do 1000BaseX Full       */
pub const ESTATUS_1000_XHALF: u64 = 0x4000; /* Can do 1000BaseX Half       */
pub const ESTATUS_1000_TFULL: u64 = 0x2000; /* Can do 1000BT Full          */
pub const ESTATUS_1000_THALF: u64 = 0x1000; /* Can do 1000BT Half          */

/* N-way test register. */
pub const NWAYTEST_RESV1: u64 = 0x00ff; /* Unused...                   */
pub const NWAYTEST_LOOPBACK: u64 = 0x0100; /* Enable loopback for N-way   */
pub const NWAYTEST_RESV2: u64 = 0xfe00; /* Unused...                   */

/* MAC and PHY tx_config_Reg[15:0] for SGMII in-band auto-negotiation.*/
pub const ADVERTISE_SGMII: u64 = 0x0001; /* MAC can do SGMII            */
pub const LPA_SGMII: u64 = 0x0001; /* PHY can do SGMII            */
pub const LPA_SGMII_SPD_MASK: u64 = 0x0c00; /* SGMII speed mask            */
pub const LPA_SGMII_FULL_DUPLEX: u64 = 0x1000; /* SGMII full duplex           */
pub const LPA_SGMII_DPX_SPD_MASK: u64 = 0x1C00; /* SGMII duplex and speed bits */
pub const LPA_SGMII_10: u64 = 0x0000; /* 10Mbps                      */
pub const LPA_SGMII_10HALF: u64 = 0x0000; /* Can do 10mbps half-duplex   */
pub const LPA_SGMII_10FULL: u64 = 0x1000; /* Can do 10mbps full-duplex   */
pub const LPA_SGMII_100: u64 = 0x0400; /* 100Mbps                     */
pub const LPA_SGMII_100HALF: u64 = 0x0400; /* Can do 100mbps half-duplex  */
pub const LPA_SGMII_100FULL: u64 = 0x1400; /* Can do 100mbps full-duplex  */
pub const LPA_SGMII_1000: u64 = 0x0800; /* 1000Mbps                    */
pub const LPA_SGMII_1000HALF: u64 = 0x0800; /* Can do 1000mbps half-duplex */
pub const LPA_SGMII_1000FULL: u64 = 0x1800; /* Can do 1000mbps full-duplex */
pub const LPA_SGMII_LINK: u64 = 0x8000; /* PHY link with copper-side partner */

/* 1000BASE-T Control register */
pub const ADVERTISE_1000FULL: u64 = 0x0200; /* Advertise 1000BASE-T full duplex */
pub const ADVERTISE_1000HALF: u64 = 0x0100; /* Advertise 1000BASE-T half duplex */
pub const CTL1000_PREFER_MASTER: u64 = 0x0400; /* prefer to operate as master */
pub const CTL1000_AS_MASTER: u64 = 0x0800;
pub const CTL1000_ENABLE_MASTER: u64 = 0x1000;

/* 1000BASE-T Status register */
pub const LPA_1000MSFAIL: u64 = 0x8000; /* Master/Slave resolution failure */
pub const LPA_1000MSRES: u64 = 0x4000; /* Master/Slave resolution status */
pub const LPA_1000LOCALRXOK: u64 = 0x2000; /* Link partner local receiver status */
pub const LPA_1000REMRXOK: u64 = 0x1000; /* Link partner remote receiver status */
pub const LPA_1000FULL: u64 = 0x0800; /* Link partner 1000BASE-T full duplex */
pub const LPA_1000HALF: u64 = 0x0400; /* Link partner 1000BASE-T half duplex */

/* Flow control flags */
pub const FLOW_CTRL_TX: u64 = 0x01;
pub const FLOW_CTRL_RX: u64 = 0x02;

/* MMD Access Control register fields */
pub const MII_MMD_CTRL_DEVAD_MASK: u64 = 0x1f; /* Mask MMD DEVAD*/
pub const MII_MMD_CTRL_ADDR: u64 = 0x0000; /* Address */
pub const MII_MMD_CTRL_NOINCR: u64 = 0x4000; /* no post increment */
pub const MII_MMD_CTRL_INCR_RDWT: u64 = 0x8000; /* post increment on reads & writes */
pub const MII_MMD_CTRL_INCR_ON_WT: u64 = 0xC000; /* post increment on writes only */

// MDIO
pub const MDIO_PRTAD_NONE: i32 = (-1);
pub const MDIO_DEVAD_NONE: i32 = (-1);
pub const MDIO_EMULATE_C22: i32 = 4;

// PHY
pub const PHY_FIXED_ID: u32 = 0xa5a55a5a;
pub const PHY_NCSI_ID: u32 = 0xbeefcafe;
pub const PHY_GMII2RGMII_ID: u32 = 0x5a5a5a5a;
pub const PHY_MAX_ADDR: u32 = 32;
pub const PHY_FLAG_BROKEN_RESET: u32 = (1 << 0); /* soft reset not supported */

/* phy seed setup */
pub const AUTO       : u32 =             99;
pub const _1000BASET  : u32 =            1000;
pub const _100BASET   : u32 =            100;
pub const _10BASET    : u32 =            10;
pub const HALF        : u32 =            22;
pub const FULL        : u32 =            44;

/* phy register offsets */
pub const MII_MIPSCR   : u32 =           0x11;

/* MII_LPA */
pub const PHY_ANLPAR_PSB_802_3  : u32 =  0x0001;
pub const PHY_ANLPAR_PSB_802_9  : u32 =  0x0002;

/* MII_CTRL1000 masks */
pub const PHY_1000BTCR_1000FD   : u32 =  0x0200;
pub const PHY_1000BTCR_1000HD   : u32 =  0x0100;

/* MII_STAT1000 masks */
pub const PHY_1000BTSR_MSCF     : u32 =  0x8000;
pub const PHY_1000BTSR_MSCR     : u32 =  0x4000;
pub const PHY_1000BTSR_LRS      : u32 =  0x2000;
pub const PHY_1000BTSR_RRS      : u32 =  0x1000;
pub const PHY_1000BTSR_1000FD   : u32 =  0x0800;
pub const PHY_1000BTSR_1000HD   : u32 =  0x0400;

/* phy EXSR */
pub const ESTATUS_1000XF        : u32 =  0x8000;
pub const ESTATUS_1000XH        : u32 =  0x4000;
