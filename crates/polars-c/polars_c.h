#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

struct DataFrame;

DataFrame *dataframe_read_parquet(const char *path_ptr);

DataFrame *dataframe_limit(DataFrame *df_ptr, uintptr_t limit);

void dataframe_display(DataFrame *df_ptr);

} // extern "C"
