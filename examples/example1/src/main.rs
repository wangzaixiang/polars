use polars::prelude::*;

fn main() -> PolarsResult<()> {
    eager_demo()
    // lazy_demo()
}

fn lazy_demo() -> PolarsResult<()> {

    use polars::prelude::*;

    let q = LazyCsvReader::new("docs/data/iris.csv")
        .has_header(true)
        .finish()?
        .filter(col("sepal_length").gt(lit(5)))
        .group_by(vec![col("species")])
        .agg([col("*").sum()]);

    let df = q.collect()?;

    println!("Running Lazy code");
    println!("{:?}", df);

    Ok(())

}

fn eager_demo() -> PolarsResult<()> {
    let file = std::fs::File::open("docs/data/iris.csv")?;

    //
    // let file = Box::new(file) as Box<dyn polars::io::mmap::MmapBytesReader>;
    println!("Running Eager code");

    let df : DataFrame = CsvReader::new(file)
        .has_header(true)
        .finish()?;
    println!("df.size = {:?}", df.shape());
    let mask = df.column("sepal_length")?.gt(5.0)?;
    let df2 = df .filter( &mask )?;

    println!("filtered = {:?}", df2);
    let df3 =  df2.group_by( ["species"])?;

    println!("grouped = {:?}", df3);

    let df4 = df3.sum()?;
    println!("summed {:?}", df4);

    Ok(())
}

