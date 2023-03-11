# set_property PACKAGE_PIN	F4	 [get_ports tck]
# set_property IOSTANDARD LVCMOS33 [get_ports tck]

# set_property PACKAGE_PIN	D2	 [get_ports tms]
# set_property IOSTANDARD LVCMOS33 [get_ports tms]

# set_property PACKAGE_PIN	D4	 [get_ports tdo]
# set_property IOSTANDARD LVCMOS33 [get_ports tdo]
# set_property PULLUP		true	 [get_ports tdo]

# set_property PACKAGE_PIN	E2	 [get_ports tdi]
# set_property IOSTANDARD LVCMOS33 [get_ports tdi]

# set_property PACKAGE_PIN	D3	 [get_ports trst]
# set_property IOSTANDARD LVCMOS33 [get_ports trst]
# set_property	PULLUP	true	 [get_ports trst]

# ## serial:0.tx
# set_property PACKAGE_PIN D10 [get_ports serial_tx]
# set_property IOSTANDARD LVCMOS33 [get_ports serial_tx]

# ## serial:0.rx
# set_property PACKAGE_PIN A9 [get_ports serial_rx]
# set_property IOSTANDARD LVCMOS33 [get_ports serial_rx]

## clk100:0
set_property PACKAGE_PIN E3 [get_ports clk]
set_property IOSTANDARD LVCMOS33 [get_ports clk]

# ## cpu_reset:0
# set_property PACKAGE_PIN C2 [get_ports cpu_reset]
# set_property IOSTANDARD LVCMOS33 [get_ports cpu_reset]

## user_leds:0
set_property PACKAGE_PIN H5 [get_ports user_leds[0]]
set_property IOSTANDARD LVCMOS33 [get_ports user_leds[0]]

## user_leds:1
set_property PACKAGE_PIN J5 [get_ports user_leds[1]]
set_property IOSTANDARD LVCMOS33 [get_ports user_leds[1]]

## user_leds:2
set_property PACKAGE_PIN T9 [get_ports user_leds[2]]
set_property IOSTANDARD LVCMOS33 [get_ports user_leds[2]]

## user_leds:3
set_property PACKAGE_PIN T10 [get_ports user_leds[3]]
set_property IOSTANDARD LVCMOS33 [get_ports user_leds[3]]

# ## user_sw:0
# set_property PACKAGE_PIN A8 [get_ports user_sw0]
# set_property IOSTANDARD LVCMOS33 [get_ports user_sw0]

# ## user_sw:1
# set_property PACKAGE_PIN C11 [get_ports user_sw1]
# set_property IOSTANDARD LVCMOS33 [get_ports user_sw1]

# ## user_sw:2
# set_property PACKAGE_PIN C10 [get_ports user_sw2]
# set_property IOSTANDARD LVCMOS33 [get_ports user_sw2]

# ## user_sw:3
# set_property PACKAGE_PIN A10 [get_ports user_sw3]
# set_property IOSTANDARD LVCMOS33 [get_ports user_sw3]

# ## user_btn:0
# set_property PACKAGE_PIN D9 [get_ports user_btn0]
# set_property IOSTANDARD LVCMOS33 [get_ports user_btn0]

# ## user_btn:1
# set_property PACKAGE_PIN C9 [get_ports user_btn1]
# set_property IOSTANDARD LVCMOS33 [get_ports user_btn1]

# ## user_btn:2
# set_property PACKAGE_PIN B9 [get_ports user_btn2]
# set_property IOSTANDARD LVCMOS33 [get_ports user_btn2]

# ## user_btn:3
# set_property PACKAGE_PIN B8 [get_ports user_btn3]
# set_property IOSTANDARD LVCMOS33 [get_ports user_btn3]

# ## spiflash_1x:0.cs_n
# set_property LOC L13 [get_ports spiflash_1x_cs_n]
# set_property IOSTANDARD LVCMOS33 [get_ports spiflash_1x_cs_n]

# ## spiflash_1x:0.mosi
# set_property LOC K17 [get_ports spiflash_1x_mosi]
# set_property IOSTANDARD LVCMOS33 [get_ports spiflash_1x_mosi]

# ## spiflash_1x:0.miso
# set_property LOC K18 [get_ports spiflash_1x_miso]
# set_property IOSTANDARD LVCMOS33 [get_ports spiflash_1x_miso]

# ## spiflash_1x:0.wp
# set_property LOC L14 [get_ports spiflash_1x_wp]
# set_property IOSTANDARD LVCMOS33 [get_ports spiflash_1x_wp]

# ## spiflash_1x:0.hold
# set_property LOC M14 [get_ports spiflash_1x_hold]
# set_property IOSTANDARD LVCMOS33 [get_ports spiflash_1x_hold]

set_property INTERNAL_VREF 0.75 [get_iobanks 34]
set_property BITSTREAM.CONFIG.SPI_BUSWIDTH 4 [current_design]

create_clock -period 10.000 -name clk [get_nets clk]

# set_clock_groups -group [get_clocks -include_generated_clocks -of [get_nets sys_clk]] -group [get_clocks -include_generated_clocks -of [get_nets eth_rx_clk]] -asynchronous
# set_clock_groups -group [get_clocks -include_generated_clocks -of [get_nets sys_clk]] -group [get_clocks -include_generated_clocks -of [get_nets eth_tx_clk]] -asynchronous
# set_clock_groups -group [get_clocks -include_generated_clocks -of [get_nets eth_rx_clk]] -group [get_clocks -include_generated_clocks -of [get_nets eth_tx_clk]] -asynchronous