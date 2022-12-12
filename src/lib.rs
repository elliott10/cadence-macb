#![no_std]
mod mii_const;
mod macb_const;
pub mod eth_macb;
pub mod eth_macb_ops;
pub mod phy_mscc;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
