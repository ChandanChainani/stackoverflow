// use std::fmt::Debug;
use csv::Error;
use ndarray::{array, Array, Array6, Array2, Array1};
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use serde::{de, Deserialize, Deserializer};

// type Record<'a> = Array<&'a str, &'a str>;

// impl Debug for Record<'_> {}

// impl<'de> Deserialize<'de> for Record<'_> { }

fn main() -> Result<(), Error> {
    let csv = "date,value
1959-07-02,0.2930
1959-07-06,0.2910
1959-07-07,0.2820
1959-07-08,0.2846
1959-07-09,0.2760
1959-07-10,0.2757";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    // let records = reader.into_records().collect::<Array<&str, &str>>();
    // let records = reader.deserialize::<Record>();
    // println!("{:?}", records);
    // let mut date = vec![];
    // let mut data = vec![];
    // for record in reader.records() {
    //     let record = record?;
    //     // println!("{:?}", record);
    //     // println!(
    //     //     "{:?}: {:?}",
    //     //     &record[0],
    //     //     &record[1]
    //     // );
    //     date.push(record[0].to_string());
    //     data.push(record[1].to_string());
    // }
    // println!("{:?}", date);
    // println!("{:?}", data);

    println!("{:?}", reader.deserialize::<Array1<String>>());
    // let data: Result<Vec<Array2<String>>, _> = reader.records().map(|row| {
    //     println!("{:?}", row);
    //     row.iter().map(|cell| {
    //         println!("{:?}", cell)
    //     });
    //     array![]
    // }).collect();

    // println!("{}", data.unwrap());
    // let trace = Scatter::new(date, data);

    // let mut plot = Plot::new();
    // plot.add_trace(trace);

    // // let layout = Layout::new()
    // //     .x_axis(Axis::new().range(date))
    // //     .title(Title::new("Manually Set Date Range"));
    // // plot.set_layout(layout);

    // plot.show();

    Ok(())
}
