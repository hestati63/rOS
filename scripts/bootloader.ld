SECTIONS
{
  . = 0x7c00;
  boot_start = .;
  .bootloader : {
    *(.bootloader)
  }
  .boot : {
    *(.bootstraping)
    *(.text .text.*)
    *(.rodata .rodata.*)
    *(.data .data.*)
    *(.got)
  }
  . = ALIGN(512);
  boot_end = .;
}
