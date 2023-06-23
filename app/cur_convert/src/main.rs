use std::{error::Error, fs::File, path::Path, io::BufReader};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Column {
    category: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct BillingPeriod {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CurReportManifest {
    #[serde(rename = "assemblyId")]
    assembly_id: String,
    account: String,
    columns: Vec<Column>,
    charset: String,
    compression: String,
    #[serde(rename = "contentType")]
    content_type: String,
    #[serde(rename = "reportId")]
    report_id: String,
    #[serde(rename = "reportName")]
    report_name: String,
    #[serde(rename = "billingPeriod")]
    billing_period: BillingPeriod,
    bucket: String,
    #[serde(rename = "reportKeys")]
    report_keys: Vec<String>,
}

fn load_book_from_json_file<P: AsRef<Path>>(path: P) -> Result<CurReportManifest, Box<dyn Error>> {
    println!("Hello, world!");
    let file = File::open(path)?; // std::io::Error の可能性
    let reader = BufReader::new(file); // 読み込み時は明示的にバッファリング
    println!("Hello, world!");
    let cur_report_mainfest: CurReportManifest = serde_json::from_reader(reader)?; // serde_json::Error の可能性
    Ok(cur_report_mainfest)
}

fn main() {
    // Jsonファイルを読み込んでスキーマをセットする
    match load_book_from_json_file("/app/dat/cost-report/20160201-20160301/daily-cost-report-Manifest.json") {
        Ok(cur_report) => {
            println!("JSON ファイルの読み込みに成功しました");
            println!("内容: {}, {}", cur_report.assembly_id, cur_report.bucket);
            // println!("内容: {}, {}", cur_report.columns[0].category, cur_report.columns[0].name);
        }
        Err(err) => eprintln!("JSON ファイルの読み込みに失敗しました: {}", err),
    }
    // Jsonファイルを読み込んでスキーマをセットする
    // 最終更新ファイルを取得する
}
