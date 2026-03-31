
pub mod cli_reader;
pub mod config_reader;
pub mod log_helper;
pub mod create_tables;

use std::fs;
use std::sync::OnceLock;
use crate::err::AppError;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
use std::time::Duration;
use sqlx::ConnectOptions;
use config_reader::Config;
use cli_reader::CliPars;


pub struct InitParams {
    pub base_url: String,
    pub log_folder_path: PathBuf,
    pub importing: bool,
    pub transforming: bool,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 
    let base_url = config_file.data.base_url;

    let log_folder_path = config_file.folders.log_folder_path;  
    if !folder_exists(&log_folder_path) {
        fs::create_dir_all(&log_folder_path)?;
    }
   
    Ok(InitParams {
        base_url: base_url,
        log_folder_path: log_folder_path,
        importing: cli_pars.importing,
        transforming: cli_pars.transforming,
    })

}

fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


pub async fn get_mon_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_mon_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub async fn get_src_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_src_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_set_up() {  // can be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder_path)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_set_up() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}


// Tests
#[cfg(test)]

mod tests {

    use super::*;
    use std::ffi::OsString;
    #[test]
    fn check_results_with_no_params() {
        let config = r#"
[data]
base_url="https://biolincc.nhlbi.nih.gov/studies/p=1"

[folders]
log_folder_path="/home/steve/Data/MDR logs/biolincc"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="biolincc"

        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        
        assert_eq!(res.base_url, "https://biolincc.nhlbi.nih.gov/studies/p=1");
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/biolincc"));
        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, false);

    }

    #[test]
    fn check_with_all_parameters() {
        let config = r#"
[data]
base_url="https://biolincc.nhlbi.nih.gov/studies/p=1"

[folders]
log_folder_path="/home/steve/Data/MDR logs/biolincc"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="biolincc"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-i", "-t"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.base_url, "https://biolincc.nhlbi.nih.gov/studies/p=1");
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/biolincc"));
        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, true);
    }
}