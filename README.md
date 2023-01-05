# Cadence Macb ethernet driver
cadence-macb ethernet driver on SiFive FU740 board.

### How to use

Ethernet initialization
```
let mut macb_device = cadence_macb::eth_macb::open().unwrap();
```
Sending network packets   
```
cadence_macb::eth_macb_ops::macb_send(&mut macb_device, &packet);
```

Receiving network packets
```
cadence_macb::eth_macb_ops::macb_recv(&mut macb_device, &mut rx_buffer);
```


