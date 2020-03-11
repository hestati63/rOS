.section .bootloader, "awx"
.global _start
.intel_syntax noprefix
.code16

_start:
  cli
  cld

# Setup the ds, es, ss
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov ss, ax

  mov sp, 0x7000

# ENABLE A20
seta20_1:
  in al, 0x64
  test al, 0x2
  jnz seta20_1  # spin until not busy

  mov al, 0xd1
  out 0x64, al

seta20_2:
  in al, 0x64
  test al, 0x2
  jnz seta20_2  # spin until not busy

  mov al, 0xdf
  out 0x60, al

# Get a820 map from bios
get_e820:
  mov eax, 0xe820
  mov edi, 0x7000 + 52 + 4        # E820_map + 4
  xor ebx, ebx
  mov edx, 0x534d4150
  mov ecx, 24
  int 0x15
  jc fail
  cmp edx, eax
  jne fail
  test ebx, ebx
  je fail
  mov ebp, 24

parse_entry:
  mov [edi - 4], ecx
  add edi, 24
  mov eax, 0xe820
  mov ecx, 24
  int 0x15
  jc done
  add ebp, 24
  test ebx, ebx
  jne parse_entry

done:
  mov [edi - 4], ecx
  mov dword ptr [0x7000], 0x40
  mov dword ptr [0x7000 + 44], ebp
  mov dword ptr [0x7000 + 48], 0x7000 + 52  # E820_map
fail:


# Switch to protected mode
  lgdt gdt_desc
  mov eax, cr0
  or  eax, 1           # CR0_PE
  mov cr0, eax

# Jump to the 32bit mode
  lea eax, [_code32]
  push 0x8
  push eax
  retf

.code32
_code32:
  mov ax, 0x10         # PROT_DS
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax


  # Load the remaining boot loaders
  mov edi, 0x7c00 # addr
  xor ecx, ecx    # sector
load_boot_loader:
  inc ecx
  add edi, 0x200
  lea esi, [boot_end]
  cmp edi, esi
  jae end
  push ecx
  push edi
  call read_sector
  pop edi
  pop ecx
  jmp load_boot_loader
end:
  jmp _head64
  hlt

boot_fail:
  mov ax, 0x8A00
  mov dx, 0x8A00
  out dx, al
  mov ax, 0x8E00
  mov dx, 0x8A00
  out dx, al
spin:
  jmp spin

read_sector: # edi: dst, ecx: offset
  call wait_disk

  mov al, 1
  mov edx, 0x1F2
  out dx, al

  mov eax, ecx
  mov edx, 0x1F3
  out dx, al

  mov eax, ecx
  shr eax, 0x8
  mov edx, 0x1F4
  out dx, al

  mov eax, ecx
  shr eax, 0x10
  mov edx, 0x1F5
  out dx, al

  mov eax, ecx
  shr eax, 0x18
  or  ax, 0xE0
  mov edx, 0x1F6
  out dx, al

  mov ax, 0x20
  mov edx, 0x1F7
  out dx, al

  call wait_disk

  mov ecx, 0x80
  mov edx, 0x1F0
  cld
  repnz ins DWORD PTR [edi], dx
  ret


wait_disk:
  mov edx, 0x1F7
  in al, dx
  and al, 0xC0
  cmp al, 0x40
  jne wait_disk
  ret

.p2align 2
gdt:
  .quad 0;                    # NULL SEGMENT
  .quad 0xCF9A000000FFFF;     # CODE SEGMENT
  .quad 0xCF92000000FFFF;     # DATA SEGMENT

gdt_desc:
  .word 0x17 # sizeof(gdt) - 1
  .long gdt  # addrof(gdt)

.org 510
.word 0xaa55
