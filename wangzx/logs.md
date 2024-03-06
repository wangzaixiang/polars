
# 2024-03-06
1. compile Polars from source code follows [Build py-polars](../README.md#python-compile-polars-from-source)
2. runs the following code OK, follows [Polars Python API](https://docs.pola.rs/#example)
    ```python3
       import polars as pl
      
       q = (
         pl.scan_csv("docs/data/iris.csv")
         .filter(pl.col("sepal_length") > 5)
         .group_by("species")
         .agg(pl.all().sum())
       )

       df = q.collect()
    ```
3. create a crate example1 in examples folder, and runs the following code
   ```rust
      use polars::prelude::*;

      let q = LazyCsvReader::new("docs/data/iris.csv")
      .has_header(true)
      .finish()?
      .filter(col("sepal_length").gt(lit(5)))
      .group_by(vec![col("species")])
      .agg([col("*").sum()]);

      let df = q.collect()?;
  ```
  - 理解 cargo 的 workspace + member 的概念，以及如何使用。
  - 接下来围绕这个例子，通过 debug 的方式来了解执行流程，主要数据结构。（第一步可以先忽略 lazy）
  - 编写了一个 eager 的版本，接下来先调试这个版本的代码。