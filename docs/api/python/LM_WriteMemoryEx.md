# LM_WriteMemoryEx

```python
def LM_WriteMemoryEx(pproc : lm_process_t, dst : int, src : bytearray)
```

# Description

Writes `src` in the calling process into a virtual address (`dst`).

# Parameters

- pproc: valid process which will be accessed for writing memory.
- dst: the address which will be written the bytes from `src`.
- src: the bytes to write into `dst`.

# Return Value

On success, it returns `true`. On failure, it returns `false`.

