MEMORY
{
  MEMORY : ORIGIN = 0x00000000, LENGTH = 1M
}

/* The entry point */
ENTRY(start);

SECTIONS
{
  .text :
  {
    *(.text .text.*);
  } > MEMORY
  .sdata :
  {
    *(.sdata .sdata.*);
  } > MEMORY
  .riscv :
  {
    *(.riscv .riscv.*);
  } > MEMORY
}

