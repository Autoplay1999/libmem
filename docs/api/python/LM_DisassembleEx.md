# LM_DisassembleEx

```python
def LM_DisassembleEx(code : int, bits : int, size : int, count : int, runtime_addr : int)
```

# Description

Disassembles one or more instructions into `lm_inst_t`'s.

# Parameters

- code: virtual address of the instructions to be disassembled.
- bits: the bits of the architecture to be disassembled. It can be `32` or `64`.
- size: the maximum size in bytes for the disassembly.
- count: the amount of instructions to be disassembled (0 for as many as possible)
- runtime_addr: the runtime address to resolve the functions (for example, relative jumps will be resolved using this address).

# Return Value

On success, it returns a list of `lm_inst_t`'s containing the disassembled instructions. On failure, it returns `None`.

