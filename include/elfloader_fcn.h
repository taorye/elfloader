#include <stdint.h>

/* api */
void *rust_elf_load(const void *elf_buf);
void *rust_elf_sym(const void *handle, const uint8_t *sym_name);
void rust_elf_unload(const void *handle);

/* to be impl */
void rust_console_putbytes(const uint8_t *bs, const size_t len);
uint8_t *rust_aligned_alloc(const size_t alignment, const size_t size);
void rust_free(const uint8_t *ptr);