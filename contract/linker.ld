MEMORY
{
  RAM (rx) : ORIGIN = 0x0, LENGTH = 0x200000 /* 2048k */
  MEMORY(rw): ORIGIN = 0x200000, LENGTH = 4M - LENGTH(RAM)
}

/* The entry point */
ENTRY(start);

EXTERN(HEAP_VECTOR);

SECTIONS
{
  .text :
  {
    *(.text .text.*);
  } > RAM
  .sdata :
  {
    *(.sdata .sdata.*);
  } > RAM
  .riscv :
  {
    *(.riscv .riscv.*);
  } > RAM

  .eh_frame :
  {
    *(.eh_frame .eh_frame.*);
  } > RAM
  .rodata :
  {
    *(.rodata .rodata.*);
  } > RAM
  .heap ORIGIN(MEMORY) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    KEEP(*(.heap.mem));
  } > MEMORY
  .bss :
  {
    *(.bss .bss.*);
  } > MEMORY
  .data :
  {
    *(.data .data.*);
  } > MEMORY
}

