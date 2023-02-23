# Cadence Macb ethernet driver
cadence-macb ethernet Rust driver on SiFive FU740 board.

### Quick Start

Initialize ethernet driver
```
pub struct MEM;
impl MemMapping for MEM {
    fn dma_alloc_coherent(pages: usize) -> usize {
        ...
    }
    fn dma_free_coherent(paddr: usize, pages: usize) {
        ...
    }
}

let mut macb_device = cadence_macb::eth_macb::open::<MEM>(&mac).unwrap();
```

Sending network packets
```
cadence_macb::eth_macb_ops::macb_send(&mut macb_device, &packet);

```

Receiving network packets
```
cadence_macb::eth_macb_ops::macb_recv(&mut macb_device, &mut rx_buffer);

```

## Reference
* Linux and U-Boot C code
