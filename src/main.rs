#[macro_use]
extern crate tokio;
#[macro_use]
extern crate thiserror;
// #[macro_use]
// extern crate derive_more;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_value_object;

mod app;
mod fetching;
mod prelude;
mod error;
mod parsing;
mod market;
mod repository;
mod serializer;
mod sec_gov;
mod openfigi;
mod yahoo_finance;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use repository::model::file::File;
    use sec_gov::model::edgar_file::EdgarFile;
    use crate::repository::model::relative_path::RelativePath;
    use crate::sec_gov::utils::read_edgar_company_index::read_edgar_company_index;
    use std::path::PathBuf;
    let path = PathBuf::from("/home/svmk/Загрузки/company.idx");
    let file = async_std::fs::File::open(&path).await?;
    let relative_path = RelativePath::from_string("".into());
    let file = EdgarFile::new(relative_path, path, file);
    let index = read_edgar_company_index(file).await?;
    println!("index = {:#?}", index);
    return Ok(());
}
