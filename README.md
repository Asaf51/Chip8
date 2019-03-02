# Chip8
Chip8 emulator written in Rust

### Opcodes:
* [x] Implement basic opcodes
* [x] Implement display opcodes
* [ ] Implement keypad opcodes

### Display:
* [x] Use SDL to make the display
* [x] Implement the draw function

### Sound:
* [ ] Make the sound work using SDL


## Bugs:
* [ ] When trying to run the game BLINKY, the game is not showing anything on the screen, and not even getting into the draw opcode. So, probably a bug in one of the opcodes that causes a infinite loop or something like that. Needs to go opcode, opcode and find the buggy one. Also debug using the `print_everythin` function.
