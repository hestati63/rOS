.section .bootstraping, "awx"
.global _head64
.intel_syntax noprefix
.code32

_head64:
verify_cpu:
  pushf
  pop eax
  mov eax, ecx
  xor eax, 0x200000
  push eax
  popf
  cmp ebx, eax
  jz no_long_mode        # no cpuid
  xor eax, eax           # cpuid 1 valid?
  cpuid
  cmp eax, 1
  jb no_long_mode        # cpuid 1 not valid.
  mov eax, 0x80000001
  cpuid
  test edx, (1 << 29)    # Check LM bit
  jz no_long_mode

  mov eax, cr4
  or eax, 0x00000020  # CR4_PAE
  mov cr4, eax

# Now, setup the page table
setup_pt:
  lea edi, [boot_pml4e]
  xor eax, eax
  mov ecx, 0x400
  rep stos dword ptr [edi]

  # setup the pdpts
  lea edi, [boot_pml4e]
  lea ebx, [boot_pdpt1]
  or ebx, 0x3            # PTE_P | PTE_W
  mov [edi], ebx
  lea ebx, [boot_pdpt2]
  or ebx, 0x3            # PTE_P | PTE_W
  mov [edi + 0x8], ebx

  # setup the pdpes
  lea edi, [boot_pdpt1]
  lea ebx, [boot_pde1]
  or ebx, 0x3            # PTE_P | PTE_W
  mov [edi], ebx

  lea edi, [boot_pdpt2]
  lea ebx, [boot_pde2]
  or ebx, 0x3            # PTE_P | PTE_W
  mov [edi], ebx

  # setup the pdes with PTE_MBZ
  mov ecx, 128
  lea ebx, [boot_pde1]
  lea edx, [boot_pde2]
  add edx, 256
  mov eax, 0x183         # PTE_P | PTE_W | PTE_MBZ

looping:
  mov [ebx], eax
  mov [edx], eax
  add ebx, 8
  add edx, 8
  add eax, 0x200000
  dec ecx
  cmp ecx, 0
  jne looping

  # load cr3
  lea eax, [boot_pml4e]
  mov cr3, eax

  # Enable the long mode
  mov ecx, 0xC0000080     # EFER_MSR
  rdmsr
  or eax, (1 << 8)        # EFER_LME
  wrmsr

  # Enable the paging
  mov eax, cr0
  or eax, (1 << 31)       # CR0_PE
  mov cr0, eax

  # Jump to the long mode
  lea eax, [gdt_desc64]
  lgdt [eax]
  lea eax, [boot_main]
  push 0x8
  push eax
  retf

no_long_mode:
  jmp no_long_mode

.p2align 2
gdt64:
  .quad 0                   # NULL SEGMENT
  .quad 0x00af9a000000ffff  # CODE SEGMENT64
  .quad 0x00cf92000000ffff  # DATA SEGMENT64
gdt_desc64:
  .word 0x17
  .quad gdt64

.p2align 12
.globl boot_pml4e
.globl boot_pdpt1
.globl boot_pdpt2
.globl boot_pde1
.globl boot_pde2

boot_pml4e:
  .space  0x1000
boot_pdpt1:
  .space  0x1000
boot_pdpt2:
  .space  0x1000
boot_pde1:
  .space  0x1000
boot_pde2:
  .space  0x1000
