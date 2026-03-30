mod processor;
pub mod data_access;
pub mod gen_helper;


use crate::{AppError, DownloadResult};
//use sqlx::{Pool, Postgres};

//use log::info;


pub async fn obtain_summary_data(base_url: &String, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    
        





    
    let mut file_res = DownloadResult::new();
    Ok(file_res)

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
