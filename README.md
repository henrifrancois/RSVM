# RSVM: a simple virtual machine, in Rust

### Inspired by the LLJS 16 bit VM Series.

Current issues: 
    - Hardcoded endianness of the system. Future versions will function independently of the system's endianness. 
    - Registers are 16 bit wide. The commitment on that is not strong. Looking into adopting a 32 bit register convention.
    - ISA is somewhat arbitrary. Would look into using a more standardized ISA in the future.

Goals:
    - Capture the major ideas in VM design, and incorporate them into this project.
    - Use this as an educational tool for learning about Computer Architecture fundamentals. 
    - Have a graphical interface which can be used to interact with the VM, possibly using WebAssembly. 