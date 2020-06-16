/*
The PPU (or Picture processing unit) handles graphics

The PPU’s registers are mostly located in the I/O registers section of CPU 
memory at $2000-$2007. Reading from and writing to PPU memory 
can be done by using the I/O registers $2006 and $2007 in CPU memory.
 - We only really do this during a v-blank, since writing in the middle
   of rendering a frame can casue image corruption. However, we can take
   advantage of this to create split screen effects

The PPU has its own memory, known as VRAM (Video RAM). Like the CPU, the PPU can also address 64 KB of memory although it only has 16 KB of physical RAM. 

Since PPU memory uses 16-bit addresses but I/O registers are only 8-bit, 
two writes to $2006 are required to set the address required. Data can 
then be read from or written to $2007. After each write to $2007, the 
address is incremented by either 1 or 32 as dictated by bit 2 of $2000.

The PPU also has a separate 256 byte area of memory, SPR-RAM (Sprite RAM), 
to store the sprite attributes
*/