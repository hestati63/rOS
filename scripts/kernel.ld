ENTRY(main)

SECTIONS
{
  . = 0x0;
  .percpu : {
    *(.percpu .percpu.*)
  }

  . = 0x8004200000;
  .text : AT(0x200000) {
    *(.text .stub .text.* .gnu.linkonce.t.*)
  }
  _etext = .;
  .rodata : {
    *(.rodata .rodata.* .gnu.linkonce.r.*)
  }
  . = ALIGN(0x1000);

  .data : {
    *(.data .data.*)
    *(.got)
  }

  . = ALIGN(0x1000);
  _init_start = .;
  .init : {
    *(.init .init.*)
  }
  . = ALIGN(0x1000);
  _init_end = .;
  _edata = .;

  .bss : {
    *(.bss)
  }

  _end = .;
}
