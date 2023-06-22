use std::fs::File;
use std::sync::Arc;

use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use parquet::arrow::{ArrowWriter, ArrowReader, ParquetFileArrowReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("identity/LineItemId", DataType::Utf8, false),
        Field::new("identity/TimeInterval", DataType::Utf8, false),
        Field::new("bill/InvoiceId", DataType::Utf8, false),
    ]));
    let input = File::open("/app/dat/20230601T133652Z/daily-cost-report-00001.csv")?;
    let mut csv = csv::ReaderBuilder::new()
        .has_header(true)
        //.with_quote(b'"')
        .with_schema(Arc::clone(&schema))
        .build(input)?;
    
    println!("test");
    let output = File::create("/app/dat/output.parquet")?;
    let mut writer = ArrowWriter::try_new(output, schema,None)?;
    while let Some(batch) = csv.next() {
        let batch = batch?;
        println!("{:?}", batch);
        writer.write(&batch)?;
    }
    writer.close()?;

    Ok(())
}