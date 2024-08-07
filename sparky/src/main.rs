use crate::dataframe::DataFrame;

mod dataframe;

fn main() {
    println!("Hello world!");
    let rows = Some(vec![vec![10109876543.01234, 4.0, 5.0], vec![0.0, 1.0, 2.0]]);
    let columns = Some(vec![
        String::from("A"),
        String::from("B"),
        String::from("C"),
    ]);
    let mut df1: DataFrame<f64> = DataFrame::new(rows, columns);
    println!("{}\n\n", df1);

    let rows = Some(vec![vec![3.1, 4.1, 5.1], vec![0.1, 1.1, 2.1]]);
    let columns = Some(vec![
        String::from("D"),
        String::from("E"),
        String::from("F"),
    ]);
    let mut df2: DataFrame<f64> = DataFrame::new(rows, columns);
    println!("{}\n\n", df2);

    let df_union_all = df1.union_all(&mut df2);
    match df_union_all {
        Ok(df) => println!("{}\n\n", df),
        Err(e) => println!("Error occurred: {e}"),
    }
}
