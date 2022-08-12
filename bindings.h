#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


struct A;

struct A *a_create(void);

void a_free(struct A *v);

void a_print(const struct A *v);
