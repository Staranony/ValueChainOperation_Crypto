from unicorn import *
from unicorn.x86_const import *

mu = Uc(UC_ARCH_X86, UC_MODE_32)

mu.mem_map(0x1000, 2 * 1024 * 1024)
mu.mem_write(0x1000, b'\xC7\x01\x13\x01\x00\x00')


mu.reg_write(UC_X86_REG_ECX, 0x2000)

mu.emu_start(0x1000, 0x1000 + 6)

result = mu.mem_read(0x2000, 4)
print(list(map(hex, result)))
