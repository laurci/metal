# metal

This project was built during the [Bucharest Hackathon 2023](https://thebucharesthackathon.com/) which we (me and my team Deadlock) ended up [winning](https://www.linkedin.com/feed/update/urn:li:activity:7041055436652851201/).

We wanted to empower users without Digital Hardware Design experience to easily build their own hardware (like accelerators, custom peripherals, high-performance signal processing, custom actuator drivers, etc) using the tools they already know.

I believe that this project can have a huge impact on the iteration speed (from software to ASIC), but also it can provide cheaper access to educational products (ex: let's say you buy an Arduino for $30. You play with the basic stuff and it gets pretty interesting. You are now ready to try some advanced projects. You want to connect to the internet. You do some research and find out about the Ethernet Shield, but quickly give up when you see it costs 2-3x more than the Arduino. With Metal, this roadblock would not exist. The shield can be just a library that you can download from GitHub and it comes with both hardware and software to use it. Now the only thing you need is the RJ45 connector that you can buy with a breakout for $5).

### Part 1: From code to hardware

We used Rust’s amazing meta-programming system to extract functions annotated by the user with the **#[metal::teleport]** directive and convert them into hardware implementations by generating Scala code (that is later converted into Verilog using Spinal HDL). The hardware implementations are **APB3** bus slaves and memory-mapped at known locations. We replace the function body with a basic copy from arguments to the locations mapped as the inputs for the APB3 slave and from the location mapped to the return value. Now everywhere the function is called, its inputs and outputs will be correctly transferred to the hardware implementation. 

### Part 2: Executing code

We compile the Rust code to the **riscv32-imc** target and we link it using a known memory map. We combine the generated hardware implementation with a standard RISCV 32 bits IMC softcore and we generate the bitstream. We load the generated binary (from the Rust code) into the generated block RAM and we load the final bitstream into the FPGA. Now your code runs and it interacts with the generated hardware peripherals like it would with normal memory.

## Setup

Currently we only support the `Xillinx X7` series FPGA. We recommend to use an FPGA board like the `Arty A7` (either one of 35T or 100T will work). 

**Prerequisites:**

- `f4pga` for `xc7`
- The GNU RiscV toolchain
- `sbt`

**Setup:**

- `source ./core/arty/init.sh`

