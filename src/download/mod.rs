mod processor;
pub mod data_access;
pub mod gen_helper;

use crate::data_models::data_vecs::*;
use crate::data_models::db_models::*;
use crate::helpers::string_extensions::*;
use crate::{AppError, DownloadResult};
use sqlx::{Pool, Postgres};
use reqwest::Client;
use scraper::{Html, Selector, ElementRef};

//use log::info;

pub async fn setup_sd_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = include_str!("../../sql/sd_tables.sql");

    sqlx::raw_sql(sql).execute(pool)
    .await
    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

pub async fn obtain_summary_data(base_url: &String, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .connect_timeout(std::time::Duration::from_secs(5))
            .build()?;

    let response = client.get(base_url)
        .send().await?;

    let response_text = response.text().await?;
    
    let mut summ_recs = StudySumms::new(400);
    let mut file_res = DownloadResult::new();

    let document = Html::parse_document(&response_text);

    let row_selector = Selector::parse("tr").unwrap();
    
    let name_selector = Selector::parse("td._name>a").unwrap();
    let acronym_selector = Selector::parse("td._acronym>a").unwrap();
    let resources_selector = Selector::parse("td._available_resources>div").unwrap();
    let period_selector = Selector::parse("td._period > div").unwrap();
    let accnum_selector = Selector::parse("td._accession_number > div").unwrap();
    let biolincc_url_selector = Selector::parse("td._biolincc_url > a").unwrap();
    let ctrial_selector = Selector::parse("td._clinical_trial_urls > div").unwrap();
    let cohort_selector = Selector::parse("td._cohort_type>div").unwrap();
    let conds_selector = Selector::parse("td._conditions > div").unwrap();
    let drest_selector = Selector::parse("td._data_restrictions_commuse > div").unwrap();
    let srest_selector = Selector::parse("td._specimen_restrictions_commuse > div").unwrap();
    let darea_selector = Selector::parse("td._data_restrictions_area_of_research > div").unwrap();
    let design_selector = Selector::parse("td._design > div").unwrap();
    let extrasd_selector = Selector::parse("td._extra_study_details_headers > div").unwrap();
    let sgenrest_selector = Selector::parse("td._specimen_restrictions_genetic > div").unwrap();
    let sgenallow_selector = Selector::parse("td._specimen_genetic_allowed > div").unwrap();
    let hasspec_selector = Selector::parse("td._has_specimens > div").unwrap();
    let hasdata_selector = Selector::parse("td._has_data > div").unwrap();
    let irbrest_selector = Selector::parse("td._restrictions_irb_approval > div").unwrap();
    let puds_selector = Selector::parse("td._is_public_use_dataset > div").unwrap();
    let mat_selector = Selector::parse("td._mat_types > div").unwrap();
    let srestnongen_selector = Selector::parse("td._specimen_restrictions_nongenetic > div").unwrap();
    let spcons_selector = Selector::parse("td._specific_consent_restrictions > div").unwrap();
    let website_selector = Selector::parse("td._website > div").unwrap();
    let stper_selector = Selector::parse("td._study_period > div").unwrap();
    let study_type_selector = Selector::parse("td._study_type > div").unwrap();
    let bg_selector = Selector::parse("td._background > div").unwrap();
    let pts_selector = Selector::parse("td._participants > div").unwrap();
    let relsts_selector = Selector::parse("td._related_studies > div").unwrap();

    let mut n = 0;
    let mut added = 0;

    for (row_num, row) in document.select(&row_selector).enumerate() {
        if row_num > 0 {
            let names: Vec<ElementRef>= row.select(&name_selector).collect();
            let acronyms: Vec<ElementRef>= row.select(&acronym_selector).collect();
            let acc_nums: Vec<ElementRef>= row.select(&accnum_selector).collect();
            let urls: Vec<ElementRef>= row.select(&biolincc_url_selector).collect();
            let tids: Vec<ElementRef>= row.select(&ctrial_selector).collect();
            let periods: Vec<ElementRef>= row.select(&stper_selector).collect();
            let study_types: Vec<ElementRef>= row.select(&study_type_selector).collect();
            let cohort_types: Vec<ElementRef>= row.select(&cohort_selector).collect();
            let coll_types: Vec<ElementRef>= row.select(&period_selector).collect();
            let backgrounds: Vec<ElementRef>= row.select(&bg_selector).collect();
            let designs: Vec<ElementRef>= row.select(&design_selector).collect();
            let participants: Vec<ElementRef>= row.select(&pts_selector).collect();
            let conditions: Vec<ElementRef>= row.select(&conds_selector).collect();
            let addit_features: Vec<ElementRef>= row.select(&extrasd_selector).collect();
            let websites: Vec<ElementRef>= row.select(&website_selector).collect();
            let relateds: Vec<ElementRef>= row.select(&relsts_selector).collect();
            let resources: Vec<ElementRef>= row.select(&resources_selector).collect();
            let has_datas: Vec<ElementRef>= row.select(&hasdata_selector).collect();
            let public_uses: Vec<ElementRef>= row.select(&puds_selector).collect();
            let has_specs: Vec<ElementRef>= row.select(&hasspec_selector).collect();
            let materials: Vec<ElementRef>= row.select(&mat_selector).collect();
            let data_comm_rests: Vec<ElementRef>= row.select(&drest_selector).collect();
            let data_area_rests: Vec<ElementRef>= row.select(&darea_selector).collect();
            let irb_app_reqds: Vec<ElementRef>= row.select(&irbrest_selector).collect();
            let spec_comm_rests: Vec<ElementRef>= row.select(&srest_selector).collect();
            let spec_gen_rests: Vec<ElementRef>= row.select(&sgenrest_selector).collect();
            let spec_gen_alloweds: Vec<ElementRef>= row.select(&sgenallow_selector).collect();
            let spec_nongen_rests: Vec<ElementRef>= row.select(&srestnongen_selector).collect();
            let specific_rests: Vec<ElementRef>= row.select(&spcons_selector).collect();

            let mut study_summary = StudySumm {
                name: names[0].inner_html().trim().to_string(),
                acronym: acronyms[0].inner_html().trim().to_string(),
                acc_num: make_string_option(acc_nums[0].inner_html().trim()),
                biolincc_url: urls[0].inner_html().trim().to_string(),
                ctrial_ids: make_string_option(tids[0].inner_html().trim()),
                study_period: make_string_option(periods[0].inner_html().trim()),
                study_type: study_types[0].inner_html().trim().to_string(),
                cohort_type: make_string_option(cohort_types[0].inner_html().trim()),
                collection_type: coll_types[0].inner_html().trim().to_string(),
                background: make_cleaned_string_option(backgrounds[0].inner_html().trim()),
                design: make_cleaned_string_option(designs[0].inner_html().trim()),
                participants: make_cleaned_string_option(participants[0].inner_html().trim()),
                condition: make_string_option(conditions[0].inner_html().trim()),
                addit_feature: make_string_option(addit_features[0].inner_html().trim()),
                website: make_string_option(websites[0].inner_html().trim()),
                related: make_string_option(relateds[0].inner_html().trim()),
                resources_available: resources[0].inner_html().trim().to_string(),
                has_data: make_true_false_bool_option(has_datas[0].inner_html().trim()),  
                is_public_use: make_yes_no_bool_option(public_uses[0].inner_html().trim()), 
                has_specimens: make_true_false_bool_option(has_specs[0].inner_html().trim()),    
                material: make_string_option(materials[0].inner_html().trim()),
                data_comm_rest: make_yes_no_bool_option(data_comm_rests[0].inner_html().trim()),
                data_area_rest: make_yes_no_bool_option(data_area_rests[0].inner_html().trim()), 
                irb_app_reqd: make_true_false_bool_option(irb_app_reqds[0].inner_html().trim()),  
                spec_comm_rest: make_yes_no_bool_option(spec_comm_rests[0].inner_html().trim()), 
                spec_gen_rest: make_yes_no_bool_option(spec_gen_rests[0].inner_html().trim()), 
                spec_gen_allowed: make_string_option(spec_gen_alloweds[0].inner_html().trim()),  
                spec_nongen_rest: make_yes_no_bool_option(spec_nongen_rests[0].inner_html().trim()), 
                specific_rest: make_specific_rest_string_option(specific_rests[0].inner_html().trim()),  
            };

            let mut add = true;
            if study_summary.acronym.ends_with("(non-BioLINCC)") {

                // Ignore 6 of these 8 non-biolinnc resources - 
                // They are either not studies or are duplicated by bio-lincc studies.

                if study_summary.acronym.starts_with("LAM") || study_summary.acronym.starts_with("REDS IV-RESPONSE") {
                    study_summary.acronym= study_summary.acronym.replace("(non-BioLINCC)", "").trim().to_string();
                }
                else {
                    add = false;
                }
            }

            if add {   // Neither really studies, or duplicated by studies
                summ_recs.add(&study_summary);
                added += 1;
                
            }

            n = row_num as i32;

        }
    }

    summ_recs.shrink_to_fit();
    summ_recs.store_data(pool).await?;


    file_res.num_checked = n;
    file_res.num_added = added;
    file_res.num_downloaded = added;
    
    Ok(file_res)

}

fn make_string_option(input: &str) -> Option<String> {
    if input == "" {None} else {Some(input.to_string())}
}

fn make_cleaned_string_option(input: &str) -> Option<String> {
    let opt = if input == "" {None} else {Some(input.to_string())};
    opt.clean_multiline()
}

fn make_yes_no_bool_option(input: &str) -> Option<bool> {
    if input == "" || input == "Not Applicable" 
        {None} 
    else { 
        if input == "Yes"
         {Some(true)}
        else {
            Some(false)
        }
    }
}

fn make_true_false_bool_option(input: &str) -> Option<bool> {
    if input == "" || input == "Not Applicable" 
        {None} 
    else { 
        if input == "True"
         {Some(true)}
        else {
            Some(false)
        }
    }
}

fn make_specific_rest_string_option(input: &str) -> Option<String> {
    if input == "" || input.starts_with("None")
        {None} 
    else { 
        Some(input.to_string())
    }
}



