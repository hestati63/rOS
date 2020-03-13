ENTRY(main)

SECTIONS
{
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
  __per_cpu_start = .;
  .percpu : {
    *(.percpu .percpu.*)
  }
  . = ALIGN(0x1000);
  _edata = .;

  .bss : {
    *(.bss)
  }

  _end = .;
}
