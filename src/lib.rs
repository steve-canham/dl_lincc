
pub mod setup;
pub mod err;
pub mod data_models;
pub mod helpers;
mod download;

use download::data_access::{get_next_download_id, update_dl_event_record};
use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

#[derive(Clone)]
pub struct DownloadResult {
    pub num_checked: i32,
    pub num_downloaded: i32,
    pub num_added: i32,
}

impl DownloadResult {
    pub fn new() -> Self {
        DownloadResult {  
        num_checked: 0,
        num_downloaded: 0,
        num_added: 0,
        }
   }

   pub fn add(&self, other: DownloadResult ) -> Self {
        DownloadResult {  
            num_checked: self.num_checked + other.num_checked,
            num_downloaded: self.num_downloaded + other.num_downloaded,
            num_added: self.num_added + other.num_added,
        }
    }
}

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;

    setup::establish_log(&params)?;
    let mon_pool = setup::get_mon_db_pool().await?;  // pool for the monitoring db
    let src_pool = setup::get_src_db_pool().await?;  // pool for the source specific db

    // Download type is constant - reading data by web scraping
    // First recreate the sd schema tables, get Id of this download,
    // then import the data into the sd tables
    // before updating the download record.

    download::setup_sd_tables(&src_pool).await?;
        
    let dl_id = get_next_download_id("All, scraped from Biolinnc web site", &mon_pool).await?;
    let res = download::obtain_summary_data(&params.base_url, &src_pool).await?;
    
    update_dl_event_record (dl_id, res, &mon_pool).await?;
    
    Ok(())  
}



