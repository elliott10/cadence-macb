use log::*;

use crate::eth_macb::usdelay;
use crate::eth_macb_ops::PhyInterfaceMode;
use crate::eth_macb_ops::{macb_mdio_read, macb_mdio_write};
use crate::mii_const::*;

/* Microsemi PHY ID's */
const PHY_ID_VSC8530: u32 = 0x00070560;
const PHY_ID_VSC8531: u32 = 0x00070570;
const PHY_ID_VSC8502: u32 = 0x00070630;
const PHY_ID_VSC8540: u32 = 0x00070760;
const PHY_ID_VSC8541: u32 = 0x00070770;
const PHY_ID_VSC8574: u32 = 0x000704a0;
const PHY_ID_VSC8584: u32 = 0x000707c0;

/* Microsemi VSC85xx PHY Register Pages */
const MSCC_EXT_PAGE_ACCESS: u32 = 31; /* Page Access Register */
const MSCC_PHY_PAGE_STD: u32 = 0x0000; /* Standard registers */
const MSCC_PHY_PAGE_EXT1: u32 = 0x0001; /* Extended registers - page 1 */
const MSCC_PHY_PAGE_EXT2: u32 = 0x0002; /* Extended registers - page 2 */
const MSCC_PHY_PAGE_EXT3: u32 = 0x0003; /* Extended registers - page 3 */
const MSCC_PHY_PAGE_EXT4: u32 = 0x0004; /* Extended registers - page 4 */
const MSCC_PHY_PAGE_GPIO: u32 = 0x0010; /* GPIO registers */
const MSCC_PHY_PAGE_TEST: u32 = 0x2A30; /* TEST Page registers */
const MSCC_PHY_PAGE_TR: u32 = 0x52B5; /* Token Ring Page registers */

/* Std Page Register 18 */
const MSCC_PHY_BYPASS_CONTROL: u32 = 18;
const PARALLEL_DET_IGNORE_ADVERTISED: u32 = BIT(3);

/* Std Page Register 22 */
const MSCC_PHY_EXT_CNTL_STATUS: u32 = 22;
const SMI_BROADCAST_WR_EN: u32 = BIT(0);

/* Std Page Register 24 */
const MSCC_PHY_EXT_PHY_CNTL_2: u32 = 24;

/* Std Page Register 28 - PHY AUX Control/Status */
const MIIM_AUX_CNTRL_STAT_REG: u32 = 28;
const MIIM_AUX_CNTRL_STAT_ACTIPHY_TO: u32 = (0x0004);
const MIIM_AUX_CNTRL_STAT_F_DUPLEX: u32 = (0x0020);
const MIIM_AUX_CNTRL_STAT_SPEED_MASK: u32 = (0x0018);
const MIIM_AUX_CNTRL_STAT_SPEED_POS: u32 = (3);
const MIIM_AUX_CNTRL_STAT_SPEED_10M: u32 = (0x0);
const MIIM_AUX_CNTRL_STAT_SPEED_100M: u32 = (0x1);
const MIIM_AUX_CNTRL_STAT_SPEED_1000M: u32 = (0x2);

/* Std Page Register 23 - Extended PHY CTRL_1 */
const MSCC_PHY_EXT_PHY_CNTL_1_REG: u32 = 23;
const MAC_IF_SELECTION_MASK: u32 = (0x1800);
const MAC_IF_SELECTION_GMII: u32 = (0);
const MAC_IF_SELECTION_RMII: u32 = (1);
const MAC_IF_SELECTION_RGMII: u32 = (2);
const MAC_IF_SELECTION_POS: u32 = (11);
const MAC_IF_SELECTION_WIDTH: u32 = (2);
const VSC8584_MAC_IF_SELECTION_MASK: u32 = BIT(12);
const VSC8584_MAC_IF_SELECTION_SGMII: u32 = 0;
const VSC8584_MAC_IF_SELECTION_1000BASEX: u32 = 1;
const VSC8584_MAC_IF_SELECTION_POS: u32 = 12;
//const MEDIA_OP_MODE_MASK	: u32 =	  GENMASK(10, 8);
const MEDIA_OP_MODE_COPPER: u32 = 0;
const MEDIA_OP_MODE_SERDES: u32 = 1;
const MEDIA_OP_MODE_1000BASEX: u32 = 2;
const MEDIA_OP_MODE_100BASEFX: u32 = 3;
const MEDIA_OP_MODE_AMS_COPPER_SERDES: u32 = 5;
const MEDIA_OP_MODE_AMS_COPPER_1000BASEX: u32 = 6;
const MEDIA_OP_MODE_AMS_COPPER_100BASEFX: u32 = 7;
const MEDIA_OP_MODE_POS: u32 = 8;

/* Extended Page 1 Register 20E1 */
const MSCC_PHY_ACTIPHY_CNTL: u32 = 20;
const PHY_ADDR_REVERSED: u32 = BIT(9);

/* Extended Page 1 Register 23E1 */

const MSCC_PHY_EXT_PHY_CNTL_4: u32 = 23;
const PHY_CNTL_4_ADDR_POS: u32 = 11;

/* Extended Page 1 Register 25E1 */
const MSCC_PHY_VERIPHY_CNTL_2: u32 = 25;

/* Extended Page 1 Register 26E1 */
const MSCC_PHY_VERIPHY_CNTL_3: u32 = 26;

/* Extended Page 2 Register 16E2 */
const MSCC_PHY_CU_PMD_TX_CNTL: u32 = 16;

/* Extended Page 2 Register 20E2 */
const MSCC_PHY_RGMII_CNTL_REG: u32 = 20;
const VSC_FAST_LINK_FAIL2_ENA_MASK: u32 = (0x8000);
const RX_CLK_OUT_MASK: u32 = (0x0800);
const RX_CLK_OUT_POS: u32 = (11);
const RX_CLK_OUT_WIDTH: u32 = (1);
const RX_CLK_OUT_NORMAL: u32 = (0);
const RX_CLK_OUT_DISABLE: u32 = (1);
const RGMII_RX_CLK_DELAY_POS: u32 = (4);
const RGMII_RX_CLK_DELAY_WIDTH: u32 = (3);
const RGMII_RX_CLK_DELAY_MASK: u32 = (0x0070);
const RGMII_TX_CLK_DELAY_POS: u32 = (0);
const RGMII_TX_CLK_DELAY_WIDTH: u32 = (3);
const RGMII_TX_CLK_DELAY_MASK: u32 = (0x0007);

/* Extended Page 2 Register 27E2 */
const MSCC_PHY_WOL_MAC_CONTROL: u32 = 27;
const EDGE_RATE_CNTL_POS: u32 = (5);
const EDGE_RATE_CNTL_WIDTH: u32 = (3);
const EDGE_RATE_CNTL_MASK: u32 = (0x00E0);
const RMII_CLK_OUT_ENABLE_POS: u32 = (4);
const RMII_CLK_OUT_ENABLE_WIDTH: u32 = (1);
const RMII_CLK_OUT_ENABLE_MASK: u32 = (0x10);

/* Extended Page 3 Register 22E3 */
const MSCC_PHY_SERDES_TX_CRC_ERR_CNT: u32 = 22;

/* Extended page GPIO register 00G */
const MSCC_DW8051_CNTL_STATUS: u32 = 0;
const MICRO_NSOFT_RESET: u32 = BIT(15);
const RUN_FROM_INT_ROM: u32 = BIT(14);
const AUTOINC_ADDR: u32 = BIT(13);
const PATCH_RAM_CLK: u32 = BIT(12);
const MICRO_PATCH_EN: u32 = BIT(7);
const DW8051_CLK_EN: u32 = BIT(4);
const MICRO_CLK_EN: u32 = BIT(3);
//const MICRO_CLK_DIVIDE(x)	: u32 =	((x) >> 1);
const MSCC_DW8051_VLD_MASK: u32 = 0xf1ff;

/* Extended page GPIO register 09G */
//const MSCC_TRAP_ROM_ADDR(x)	: u32 =	((x) * 2 + 1);
const MSCC_TRAP_ROM_ADDR_SERDES_INIT: u32 = 0x3eb7;

/* Extended page GPIO register 10G */
//const MSCC_PATCH_RAM_ADDR(x)	: u32 =	(((x) + 1) * 2);
const MSCC_PATCH_RAM_ADDR_SERDES_INIT: u32 = 0x4012;

/* Extended page GPIO register 11G */
const MSCC_INT_MEM_ADDR: u32 = 11;

/* Extended page GPIO register 12G */
const MSCC_INT_MEM_CNTL: u32 = 12;
const READ_SFR: u32 = (BIT(14) | BIT(13));
const READ_PRAM: u32 = BIT(14);
const READ_ROM: u32 = BIT(13);
const READ_RAM: u32 = (0x00 << 13);
const INT_MEM_WRITE_EN: u32 = BIT(12);
//const EN_PATCH_RAM_TRAP_ADDR(x): u32 =	BIT((x) + 7);
//const INT_MEM_DATA_M: u32 = GENMASK(7, 0);
//const INT_MEM_DATA(x)	: u32 =		(INT_MEM_DATA_M & (x));

/* Extended page GPIO register 13G */
const MSCC_CLKOUT_CNTL: u32 = 13;
const CLKOUT_ENABLE: u32 = BIT(15);
//const CLKOUT_FREQ_MASK: u32 =		GENMASK(14, 13);
const CLKOUT_FREQ_25M: u32 = (0x0 << 13);
const CLKOUT_FREQ_50M: u32 = (0x1 << 13);
const CLKOUT_FREQ_125M: u32 = (0x2 << 13);

/* Extended page GPIO register 18G */
const MSCC_PHY_PROC_CMD: u32 = 18;
const PROC_CMD_NCOMPLETED: u32 = BIT(15);
const PROC_CMD_FAILED: u32 = BIT(14);
//const PROC_CMD_SGMII_PORT(x): u32 =		  ((x) << 8);
//const PROC_CMD_FIBER_PORT(x): u32 =		  BIT(8 + (x) % 4);
const PROC_CMD_QSGMII_PORT: u32 = (BIT(11) | BIT(10));
const PROC_CMD_RST_CONF_PORT: u32 = BIT(7);
const PROC_CMD_RECONF_PORT: u32 = (0 << 7);
const PROC_CMD_READ_MOD_WRITE_PORT: u32 = BIT(6);
const PROC_CMD_WRITE: u32 = BIT(6);
const PROC_CMD_READ: u32 = (0 << 6);
const PROC_CMD_FIBER_DISABLE: u32 = BIT(5);
const PROC_CMD_FIBER_100BASE_FX: u32 = BIT(4);
const PROC_CMD_FIBER_1000BASE_X: u32 = (0 << 4);
const PROC_CMD_SGMII_MAC: u32 = (BIT(5) | BIT(4));
const PROC_CMD_QSGMII_MAC: u32 = BIT(5);
const PROC_CMD_NO_MAC_CONF: u32 = (0x00 << 4);
const PROC_CMD_1588_DEFAULT_INIT: u32 = BIT(4);
//const PROC_CMD_NOP: u32 = GENMASK(3, 0);
const PROC_CMD_PHY_INIT: u32 = (BIT(3) | BIT(1));
const PROC_CMD_CRC16: u32 = BIT(3);
const PROC_CMD_FIBER_MEDIA_CONF: u32 = BIT(0);
const PROC_CMD_MCB_ACCESS_MAC_CONF: u32 = (0x0000 << 0);
const PROC_CMD_NCOMPLETED_TIMEOUT_MS: u32 = 500;

/* Extended page GPIO register 19G */
const MSCC_PHY_MAC_CFG_FASTLINK: u32 = 19;
//const MAC_CFG_MASK	: u32 =		  GENMASK(15, 14);
const MAC_CFG_SGMII: u32 = (0x00 << 14);
const MAC_CFG_QSGMII: u32 = BIT(14);

/* Test Registers */
const MSCC_PHY_TEST_PAGE_5: u32 = 5;

const MSCC_PHY_TEST_PAGE_8: u32 = 8;
const TR_CLK_DISABLE: u32 = BIT(15);

const MSCC_PHY_TEST_PAGE_9: u32 = 9;
const MSCC_PHY_TEST_PAGE_20: u32 = 20;
const MSCC_PHY_TEST_PAGE_24: u32 = 24;

/* Token Ring Page 0x52B5 Registers */
const MSCC_PHY_REG_TR_ADDR_16: u32 = 16;
const MSCC_PHY_REG_TR_DATA_17: u32 = 17;
const MSCC_PHY_REG_TR_DATA_18: u32 = 18;

/* Token Ring - Read Value in */
const MSCC_PHY_TR_16_READ: u32 = (0xA000);
/* Token Ring - Write Value out */
const MSCC_PHY_TR_16_WRITE: u32 = (0x8000);

/* Token Ring Registers */
const MSCC_PHY_TR_LINKDETCTRL_POS: u32 = (3);
const MSCC_PHY_TR_LINKDETCTRL_WIDTH: u32 = (2);
const MSCC_PHY_TR_LINKDETCTRL_VAL: u32 = (3);
const MSCC_PHY_TR_LINKDETCTRL_MASK: u32 = (0x0018);
const MSCC_PHY_TR_LINKDETCTRL_ADDR: u32 = (0x07F8);

const MSCC_PHY_TR_VGATHRESH100_POS: u32 = (0);
const MSCC_PHY_TR_VGATHRESH100_WIDTH: u32 = (7);
const MSCC_PHY_TR_VGATHRESH100_VAL: u32 = (0x0018);
const MSCC_PHY_TR_VGATHRESH100_MASK: u32 = (0x007f);
const MSCC_PHY_TR_VGATHRESH100_ADDR: u32 = (0x0FA4);

const MSCC_PHY_TR_VGAGAIN10_U_POS: u32 = (0);
const MSCC_PHY_TR_VGAGAIN10_U_WIDTH: u32 = (1);
const MSCC_PHY_TR_VGAGAIN10_U_MASK: u32 = (0x0001);
const MSCC_PHY_TR_VGAGAIN10_U_VAL: u32 = (0);

const MSCC_PHY_TR_VGAGAIN10_L_POS: u32 = (12);
const MSCC_PHY_TR_VGAGAIN10_L_WIDTH: u32 = (4);
const MSCC_PHY_TR_VGAGAIN10_L_MASK: u32 = (0xf000);
const MSCC_PHY_TR_VGAGAIN10_L_VAL: u32 = (0x0001);
const MSCC_PHY_TR_VGAGAIN10_ADDR: u32 = (0x0F92);

/* General Timeout Values */
const MSCC_PHY_RESET_TIMEOUT: u16 = (100);
const MSCC_PHY_MICRO_TIMEOUT: u16 = (500);

const VSC8584_REVB: u32 = 0x0001;
//const MSCC_DEV_REV_MASK: u32 =	GENMASK(3, 0);

const MSCC_VSC8574_REVB_INT8051_FW_START_ADDR: u32 = 0x4000;
const MSCC_VSC8574_REVB_INT8051_FW_CRC: u32 = 0x29e8;

const MSCC_VSC8584_REVB_INT8051_FW_START_ADDR: u32 = 0xe800;
const MSCC_VSC8584_REVB_INT8051_FW_CRC: u32 = 0xfb48;

const fn BIT(nr: u32) -> u32 {
    1 << nr
}

pub fn vsc8541_config(phydev_addr: u32, interface: PhyInterfaceMode) -> i32 {
    let mut retval: i32 = -22; // EINVAL
    let rmii_clk_out = 0;
    let edge_rate = 4;
    //let edge_rate = VSC_PHY_CLK_SLEW_RATE_4;

    info!("vsc8541_config");

    /* For VSC8530/31 and VSC8540/41 the init scripts are the same */
    mscc_vsc8531_vsc8541_init_scripts(phydev_addr);

    /* For VSC8540/41 the only MAC modes are (G)MII and RMII/RGMII. */
    match interface {
        PhyInterfaceMode::MII
        | PhyInterfaceMode::GMII
        | PhyInterfaceMode::RMII
        | PhyInterfaceMode::RGMII => {
            retval = vsc8531_vsc8541_mac_config(phydev_addr, interface);
            if retval != 0 {
                return retval;
            }
            retval = mscc_phy_soft_reset(phydev_addr);
            if retval != 0 {
                return retval;
            }
        }
        _ => {
            error!("PHY 8541 MAC i/f config Error: mac i/f = {:?}", interface);
            return -22;
        }
    }

    /* Default RMII Clk Output to 0=OFF/1=ON  */
    retval = vsc8531_vsc8541_clk_skew_config(phydev_addr, interface);
    if retval != 0 {
        return retval;
    }

    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_EXT2 as u16);
    let mut reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_WOL_MAC_CONTROL) as u32;
    /* Reg27E2 - Update Clk Slew Rate. */
    reg_val = bitfield_replace(reg_val, EDGE_RATE_CNTL_POS, EDGE_RATE_CNTL_WIDTH, edge_rate);
    /* Reg27E2 - Update RMII Clk Out. */
    reg_val = bitfield_replace(
        reg_val,
        RMII_CLK_OUT_ENABLE_POS,
        RMII_CLK_OUT_ENABLE_WIDTH,
        rmii_clk_out,
    );

    /* Update Reg27E2 */
    macb_mdio_write(phydev_addr, MSCC_PHY_WOL_MAC_CONTROL, reg_val as u16);
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);

    /* Configure the clk output */
    retval = vsc8531_vsc8541_clkout_config(phydev_addr);
    if retval != 0 {
        return retval;
    }

    genphy_config_aneg(phydev_addr)
}

fn mscc_phy_soft_reset(phydev_addr: u32) -> i32 {
    let mut timeout: u16 = MSCC_PHY_RESET_TIMEOUT;
    let mut reg_val: u16 = 0;

    info!("mscc_phy_soft_reset");
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);
    reg_val = macb_mdio_read(phydev_addr, MII_BMCR);
    macb_mdio_write(phydev_addr, MII_BMCR, reg_val | (BMCR_RESET as u16));

    reg_val = macb_mdio_read(phydev_addr, MII_BMCR);
    while (reg_val & (BMCR_RESET as u16) != 0) && (timeout > 0) {
        reg_val = macb_mdio_read(phydev_addr, MII_BMCR);
        timeout -= 1;
        usdelay(1000); /* 1 毫秒 */
    }
    if timeout == 0 {
        //error!("MSCC PHY Soft_Reset Error: mac i/f = {:#x}", interface);
        error!("MSCC PHY Soft_Reset Error");
        return -1;
    }
    0
}

fn mscc_vsc8531_vsc8541_init_scripts(phydev_addr: u32) {
    info!("mscc_vsc8531_vsc8541_init_scripts");

    // Set to Access Token Ring Registers
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_TR as u16);

    // Update LinkDetectCtrl default to optimized values
    // Determined during Silicon Validation Testing
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_LINKDETCTRL_ADDR | MSCC_PHY_TR_16_READ) as u16,
    );
    let mut reg_val: u16 = macb_mdio_read(phydev_addr, MSCC_PHY_REG_TR_DATA_17);
    reg_val = bitfield_replace(
        reg_val as u32,
        MSCC_PHY_TR_LINKDETCTRL_POS,
        MSCC_PHY_TR_LINKDETCTRL_WIDTH,
        MSCC_PHY_TR_LINKDETCTRL_VAL,
    ) as u16;

    macb_mdio_write(phydev_addr, MSCC_PHY_REG_TR_DATA_17, reg_val);
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_LINKDETCTRL_ADDR | MSCC_PHY_TR_16_WRITE) as u16,
    );

    // Update VgaThresh100 defaults to optimized values
    // Determined during Silicon Validation Testing
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_VGATHRESH100_ADDR | MSCC_PHY_TR_16_READ) as u16,
    );
    reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_REG_TR_DATA_18);
    reg_val = bitfield_replace(
        reg_val as u32,
        MSCC_PHY_TR_VGATHRESH100_POS,
        MSCC_PHY_TR_VGATHRESH100_WIDTH,
        MSCC_PHY_TR_VGATHRESH100_VAL,
    ) as u16;
    macb_mdio_write(phydev_addr, MSCC_PHY_REG_TR_DATA_18, reg_val);
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_VGATHRESH100_ADDR | MSCC_PHY_TR_16_WRITE) as u16,
    );

    // Update VgaGain10 defaults to optimized values
    // Determined during Silicon Validation Testing
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_VGAGAIN10_ADDR | MSCC_PHY_TR_16_READ) as u16,
    );
    reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_REG_TR_DATA_18);
    reg_val = bitfield_replace(
        reg_val as u32,
        MSCC_PHY_TR_VGAGAIN10_U_POS,
        MSCC_PHY_TR_VGAGAIN10_U_WIDTH,
        MSCC_PHY_TR_VGAGAIN10_U_VAL,
    ) as u16;

    macb_mdio_write(phydev_addr, MSCC_PHY_REG_TR_DATA_18, reg_val);

    reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_REG_TR_DATA_17);
    reg_val = bitfield_replace(
        reg_val as u32,
        MSCC_PHY_TR_VGAGAIN10_L_POS,
        MSCC_PHY_TR_VGAGAIN10_L_WIDTH,
        MSCC_PHY_TR_VGAGAIN10_L_VAL,
    ) as u16;
    macb_mdio_write(phydev_addr, MSCC_PHY_REG_TR_DATA_17, reg_val);
    macb_mdio_write(
        phydev_addr,
        MSCC_PHY_REG_TR_ADDR_16,
        (MSCC_PHY_TR_VGAGAIN10_ADDR | MSCC_PHY_TR_16_WRITE) as u16,
    );

    // Set back to Access Standard Page Registers
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);
}

fn vsc8531_vsc8541_mac_config(phydev_addr: u32, interface: PhyInterfaceMode) -> i32 {
    let mut reg_val: u16 = 0;
    let mut mac_if = 0;
    let mut rx_clk_out = 0;

    /* For VSC8530/31 the only MAC modes are RMII/RGMII. */
    /* For VSC8540/41 the only MAC modes are (G)MII and RMII/RGMII. */
    /* Setup MAC Configuration */
    match interface {
        PhyInterfaceMode::MII | PhyInterfaceMode::GMII => {
            //Set Reg23.12:11=0
            mac_if = MAC_IF_SELECTION_GMII;
            //Set Reg20E2.11=1
            rx_clk_out = RX_CLK_OUT_DISABLE;
        }
        PhyInterfaceMode::RMII => {
            //Set Reg23.12:11=1
            mac_if = MAC_IF_SELECTION_RMII;
            //Set Reg20E2.11=0
            rx_clk_out = RX_CLK_OUT_NORMAL;
        }
        PhyInterfaceMode::RGMII
        | PhyInterfaceMode::RGMII_ID
        | PhyInterfaceMode::RGMII_TXID
        | PhyInterfaceMode::RGMII_RXID => {
            //Set Reg23.12:11=2
            mac_if = MAC_IF_SELECTION_RGMII;
            //Set Reg20E2.11=0
            rx_clk_out = RX_CLK_OUT_NORMAL;
        }
        _ => {
            error!(
                "MSCC PHY - INVALID MAC i/f Config: mac i/f = {:?}",
                interface
            );
            return -1;
        }
    }

    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);
    reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_EXT_PHY_CNTL_1_REG);
    // Set MAC i/f bits Reg23.12:11
    reg_val = bitfield_replace(
        reg_val as u32,
        MAC_IF_SELECTION_POS,
        MAC_IF_SELECTION_WIDTH,
        mac_if,
    ) as u16;

    info!("vsc8531_vsc8541_mac_config reg_val: {:#x}", reg_val);

    // Update Reg23.12:11
    macb_mdio_write(phydev_addr, MSCC_PHY_EXT_PHY_CNTL_1_REG, reg_val);

    // Setup ExtPg_2 Register Access
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_EXT2 as u16);

    // Read Reg20E2
    reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_RGMII_CNTL_REG);
    reg_val = bitfield_replace(reg_val as u32, RX_CLK_OUT_POS, RX_CLK_OUT_WIDTH, rx_clk_out) as u16;

    info!("vsc8531_vsc8541_mac_config reg_val: {:#x}", reg_val);

    // Update Reg20E2.11
    macb_mdio_write(phydev_addr, MSCC_PHY_RGMII_CNTL_REG, reg_val);

    // Before leaving - Change back to Std Page Register Access
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);

    0
}

fn vsc8531_vsc8541_clk_skew_config(phydev_addr: u32, interface: PhyInterfaceMode) -> i32 {
    /*
    // VSC_PHY_RGMII_DELAY_200_PS: 0, VSC_PHY_RGMII_DELAY_2000_PS: 4
    let mut rx_clk_skew = VSC_PHY_RGMII_DELAY_200_PS;
    let mut tx_clk_skew = VSC_PHY_RGMII_DELAY_200_PS;
    */
    let mut rx_clk_skew = 0;
    let mut tx_clk_skew = 0;

    if (interface == PhyInterfaceMode::RGMII_RXID) || (interface == PhyInterfaceMode::RGMII_ID) {
        //rx_clk_skew = VSC_PHY_RGMII_DELAY_2000_PS;
        rx_clk_skew = 4;
    }
    if (interface == PhyInterfaceMode::RGMII_TXID) || (interface == PhyInterfaceMode::RGMII_ID) {
        //tx_clk_skew = VSC_PHY_RGMII_DELAY_2000_PS;
        tx_clk_skew = 4;
    }
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_EXT2 as u16);
    let mut reg_val = macb_mdio_read(phydev_addr, MSCC_PHY_RGMII_CNTL_REG) as u32;

    // Reg20E2 - Update RGMII RX_Clk Skews.
    reg_val = bitfield_replace(
        reg_val,
        RGMII_RX_CLK_DELAY_POS,
        RGMII_RX_CLK_DELAY_WIDTH,
        rx_clk_skew,
    );
    // Reg20E2 - Update RGMII TX_Clk Skews.
    reg_val = bitfield_replace(
        reg_val,
        RGMII_TX_CLK_DELAY_POS,
        RGMII_TX_CLK_DELAY_WIDTH,
        tx_clk_skew,
    );

    info!("vsc8531_vsc8541_clk_skew_config reg_val: {:#x}", reg_val);
    macb_mdio_write(phydev_addr, MSCC_PHY_RGMII_CNTL_REG, reg_val as u16);
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);
    0
}

fn vsc8531_vsc8541_clkout_config(phydev_addr: u32) -> i32 {
    let clkout_rate: u32 = 0; //clkout_rate: 0
    let mut reg_val = 0;

    match clkout_rate {
        0 => {
            reg_val = 0;
        }
        25000000 => {
            reg_val = CLKOUT_FREQ_25M | CLKOUT_ENABLE;
        }
        50000000 => {
            reg_val = CLKOUT_FREQ_50M | CLKOUT_ENABLE;
        }
        125000000 => {
            reg_val = CLKOUT_FREQ_125M | CLKOUT_ENABLE;
        }
        _ => {
            error!("PHY 8530/31 invalid clkout rate {}", clkout_rate);
            return -1;
        }
    }
    info!("vsc8531_vsc8541_clkout_config reg_val: {:#x}", reg_val);

    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_GPIO as u16);
    macb_mdio_write(phydev_addr, MSCC_CLKOUT_CNTL, reg_val as u16);
    macb_mdio_write(phydev_addr, MSCC_EXT_PAGE_ACCESS, MSCC_PHY_PAGE_STD as u16);
    0
}

// Replace the value of a bitfield found within a given register value
// Returns the newly modified uint value with the replaced field.
fn bitfield_replace(reg_val: u32, shift: u32, width: u32, bitfield_val: u32) -> u32 {
    // Produces a mask of set bits covering a range of a uint value
    let mask: u32 = ((1 << width) - 1) << shift;
    (reg_val & !mask) | ((bitfield_val << shift) & mask)
}

const AUTONEG_DISABLE: u32 = 0;
const AUTONEG_ENABLE: u32 = 1;
/**
 * genphy_config_aneg - restart auto-negotiation or write BMCR
 * @phydev: target phy_device struct
 *
 * Description: If auto-negotiation is enabled, we configure the
 *   advertising, and then restart auto-negotiation.  If it is not
 *   enabled, then we write the BMCR.
 */
pub fn genphy_config_aneg(phydev_addr: u32) -> i32 {
    let phydev_autoneg = AUTONEG_ENABLE;
    let phydev_speed = 0;
    let phydev_duplex = -1;
    let phydev_link = 0;

    let mut phydev_advertising = 0x2ff;
    let phydev_supported = 0x2ff;

    if phydev_autoneg != AUTONEG_ENABLE {
        return genphy_setup_forced(phydev_addr, phydev_speed, phydev_duplex);
    }
    let mut result = genphy_config_advert(phydev_addr, &mut phydev_advertising, phydev_supported);
    // error
    if result < 0 {
        return result;
    }
    if result == 0 {
        // Advertisment hasn't changed, but maybe aneg was never on to begin with?  Or maybe phy was isolated?
        let ctl = macb_mdio_read(phydev_addr, MII_BMCR) as u32;
        if ctl < 0 {
            return ctl as i32;
        }
        if (((ctl & BMCR_ANENABLE) == 0) || ((ctl & BMCR_ISOLATE) != 0)) {
            // do restart aneg
            result = 1;
        }
    }

    // Only restart aneg if we are advertising something different than we were before.
    if result > 0 {
        result = genphy_restart_aneg(phydev_addr);
    }
    result
}

fn genphy_setup_forced(phydev_addr: u32, speed: i32, duplex: i32) -> i32 {
    let mut ctl = BMCR_ANRESTART;

    if speed == SPEED_1000 as i32 {
        ctl |= BMCR_SPEED1000;
    } else if speed == SPEED_100 as i32 {
        ctl |= BMCR_SPEED100;
    }

    if duplex == DUPLEX_FULL as i32 {
        ctl |= BMCR_FULLDPLX;
    }

    info!("genphy_setup_forced Write MII_BMCR: {:#x}", ctl);
    macb_mdio_write(phydev_addr, MII_BMCR, ctl as u16);
    0
}

/**
 * genphy_config_advert - sanitize and advertise auto-negotiation parameters
 * @phydev: target phy_device struct
 *
 * Description: Writes MII_ADVERTISE with the appropriate values,
 *   after sanitizing the values to make sure we only advertise
 *   what is supported.  Returns < 0 on error, 0 if the PHY's advertisement
 *   hasn't changed, and > 0 if it has changed.
 */
fn genphy_config_advert(
    phydev_addr: u32,
    phydev_advertising: &mut u32,
    phydev_supported: u32,
) -> i32 {
    let mut changed = 0;
    /* Only allow advertising what this PHY supports */
    *phydev_advertising &= phydev_supported;
    let advertise: u32 = *phydev_advertising;

    /* Setup standard advertisement */
    let mut adv = macb_mdio_read(phydev_addr, MII_ADVERTISE) as u32;
    let mut oldadv = adv;
    if adv < 0 {
        return adv as i32;
    }

    adv &= !(ADVERTISE_ALL | ADVERTISE_100BASE4 | ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM);
    if (advertise & ADVERTISED_10baseT_Half) != 0 {
        adv |= ADVERTISE_10HALF;
    }
    if (advertise & ADVERTISED_10baseT_Full) != 0 {
        adv |= ADVERTISE_10FULL;
    }
    if (advertise & ADVERTISED_100baseT_Half) != 0 {
        adv |= ADVERTISE_100HALF;
    }
    if (advertise & ADVERTISED_100baseT_Full) != 0 {
        adv |= ADVERTISE_100FULL;
    }
    if (advertise & ADVERTISED_Pause) != 0 {
        adv |= ADVERTISE_PAUSE_CAP;
    }
    if (advertise & ADVERTISED_Asym_Pause) != 0 {
        adv |= ADVERTISE_PAUSE_ASYM;
    }
    if (advertise & ADVERTISED_1000baseX_Half) != 0 {
        adv |= ADVERTISE_1000XHALF;
    }
    if (advertise & ADVERTISED_1000baseX_Full) != 0 {
        adv |= ADVERTISE_1000XFULL;
    }

    if adv != oldadv {
        macb_mdio_write(phydev_addr, MII_ADVERTISE, adv as u16);
        changed = 1;
    }

    let bmsr: u32 = macb_mdio_read(phydev_addr, MII_BMSR) as u32;
    if bmsr < 0 {
        return bmsr as i32;
    }
    /* Per 802.3-2008, Section 22.2.4.2.16 Extended status all
     * 1000Mbits/sec capable PHYs shall have the BMSR_ESTATEN bit set to a
     * logical 1.
     */
    if (bmsr & BMSR_ESTATEN) == 0 {
        return changed;
    }

    /* Configure gigabit if it's supported */
    adv = macb_mdio_read(phydev_addr, MII_CTRL1000) as u32;
    oldadv = adv;
    if adv < 0 {
        return adv as i32;
    }
    adv &= !(ADVERTISE_1000FULL | ADVERTISE_1000HALF);
    if (phydev_supported & (SUPPORTED_1000baseT_Half | SUPPORTED_1000baseT_Full)) != 0 {
        if (advertise & SUPPORTED_1000baseT_Half) != 0 {
            adv |= ADVERTISE_1000HALF;
        }
        if (advertise & SUPPORTED_1000baseT_Full) != 0 {
            adv |= ADVERTISE_1000FULL;
        }
    }
    if adv != oldadv {
        changed = 1;
    }
    info!("genphy_config_advert Write MII_CTRL1000: {:#x}", adv);
    macb_mdio_write(phydev_addr, MII_CTRL1000, adv as u16);
    changed
}

/**
 * genphy_restart_aneg - Enable and Restart Autonegotiation
 * @phydev: target phy_device struct
 */
fn genphy_restart_aneg(phydev_addr: u32) -> i32 {
    let mut ctl: u32 = macb_mdio_read(phydev_addr, MII_BMCR) as u32;
    ctl |= (BMCR_ANENABLE | BMCR_ANRESTART);

    /* Don't isolate the PHY if we're negotiating */
    ctl &= !(BMCR_ISOLATE);

    info!("genphy_restart_aneg MII_BMCR: {:#x}", ctl);
    macb_mdio_write(phydev_addr, MII_BMCR, ctl as u16);
    0
}
