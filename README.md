# rust-iot-workshop 🦀
In this workshop we will create a "smart" humidity and temperature sensor. The goal is to become familiar with some embedded programming in rust on ESP controllers. 

## Project
todo!

<details>
<summary> Setup esp on your computer 💻 </summary>
To be able to work with our esp controller we need to setup our development environment. Esp has a [official book](https://docs.esp-rs.org/book/introduction.html) which explains how to work with esp controllers with rust 

### Prerequisites
To run application with standard library(std) we need ldproxy.
```
cargo install ldproxy
```

### Setup tooling for RISC-V and Xtensa Targets
This setup is also described in the book [here](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html). So feel free to check it out for a more detail description of the tooling. Setting up the tooling is a three step process:
1. Install espup
```
cargo install espup
```
2. Install dependencies
```
espup install
```
3. Setup environment variable
ESP uses some specific environment variables when building the project, these values need to be exported via the export script downloaded by espup. To avoid having to run this command 
```
. $HOME/export-esp.sh
```
each time we need change project I recommend adding a alias to your rc file. By adding this line to our rc file
```
alias get_idf='. $HOME/esp/esp-idf/export.sh'
```
we can run `get_idf` befor building a different esp project

</details>
