# PNML Parser
Simple Rust parser for the PNML format (http://www.pnml.org/).

The parser currently supports only P/T petri nets and ignores graphical elements.

### To Do: 

 - Support for multiple nets in one file
 - Support for different net types (symmetric, high-level, restricted)
 - When the net is invalid, return an error instead of panicking.
