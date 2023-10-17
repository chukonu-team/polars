#include "polars_c.h"
#include <cstdio>

int main(int argc, char* argv[]) {
    printf("%s\n", argv[1]);
    auto df = dataframe_read_parquet(argv[1]);
    auto df1 = dataframe_limit(df, 100);
    dataframe_display(df1);

    return 0;
}

