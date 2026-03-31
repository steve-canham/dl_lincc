use crate::data_models::db_models::*;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::AppError;

pub struct StudySumms {
    pub names: Vec<String>,
    pub acronyms: Vec<String>,
    pub acc_nums: Vec<Option<String>>,
    pub biolincc_urls: Vec<String>,
    pub ctrial_idss: Vec<Option<String>>,
    pub study_periods: Vec<Option<String>>,
    pub study_types: Vec<String>,
    pub cohort_types: Vec<Option<String>>,
    pub collection_types: Vec<String>,
    pub backgrounds: Vec<Option<String>>,
    pub designs: Vec<Option<String>>,
    pub participantss: Vec<Option<String>>,
    pub conditions: Vec<Option<String>>,
    pub addit_features: Vec<Option<String>>,
    pub websites: Vec<Option<String>>,
    pub relateds: Vec<Option<String>>,
    pub resources_availables: Vec<String>,
    pub has_datas: Vec<Option<bool>>,     // false, true
    pub is_public_uses: Vec<Option<bool>>,  // Yes, No
    pub has_specimenss: Vec<Option<bool>>,         // false, true          
    pub materials: Vec<Option<String>>,
    pub data_comm_rests: Vec<Option<bool>>,  // No, Yes, Not Applicable
    pub data_area_rests: Vec<Option<bool>>,   // No, Yes, Not Applicable
    pub irb_app_reqds: Vec<Option<bool>>,      // false, true
    pub spec_comm_rests: Vec<Option<bool>>,     // No, Yes, Not Applicable
    pub spec_gen_rests: Vec<Option<bool>>,     // No, Yes, Not Applicable
    pub spec_gen_alloweds: Vec<Option<String>>,   // No, Yes, Yes, For Some specimens, Not Applicable
    pub spec_nongen_rests: Vec<Option<bool>>, // No, Yes, Not Applicable
    pub specific_rests: Vec<Option<String>>,   // Includes None, None.
}

impl StudySumms{

    pub fn new(vsize: usize) -> Self {
        StudySumms { 
            names: Vec::with_capacity(vsize),
            acronyms: Vec::with_capacity(vsize),
            acc_nums: Vec::with_capacity(vsize),
            biolincc_urls: Vec::with_capacity(vsize),
            ctrial_idss: Vec::with_capacity(vsize),
            study_periods: Vec::with_capacity(vsize),
            study_types: Vec::with_capacity(vsize),
            cohort_types: Vec::with_capacity(vsize),
            collection_types: Vec::with_capacity(vsize),
            backgrounds: Vec::with_capacity(vsize),
            designs: Vec::with_capacity(vsize),
            participantss: Vec::with_capacity(vsize),
            conditions: Vec::with_capacity(vsize),
            addit_features: Vec::with_capacity(vsize),
            websites: Vec::with_capacity(vsize),
            relateds: Vec::with_capacity(vsize),
            resources_availables: Vec::with_capacity(vsize),
            has_datas: Vec::with_capacity(vsize),     // false, true
            is_public_uses: Vec::with_capacity(vsize),// Yes, No
            has_specimenss: Vec::with_capacity(vsize),        // false, true          
            materials: Vec::with_capacity(vsize),
            data_comm_rests: Vec::with_capacity(vsize),// No, Yes, Not Applicable
            data_area_rests: Vec::with_capacity(vsize),// No, Yes, Not Applicable
            irb_app_reqds: Vec::with_capacity(vsize),     // false, true
            spec_comm_rests: Vec::with_capacity(vsize),    // No, Yes, Not Applicable
            spec_gen_rests: Vec::with_capacity(vsize),   // No, Yes, Not Applicable
            spec_gen_alloweds: Vec::with_capacity(vsize), // No, Yes, Yes, For Some specimens, Not Applicable
            spec_nongen_rests: Vec::with_capacity(vsize), // No, Yes, Not Applicable
            specific_rests: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: &StudySumm) {
        self.names.push(r.name.clone());
        self.acronyms.push(r.acronym.clone());
        self.acc_nums.push(r.acc_num.clone());
        self.biolincc_urls.push(r.biolincc_url.clone());
        self.ctrial_idss.push(r.ctrial_ids.clone());
        self.study_periods.push(r.study_period.clone());
        self.study_types.push(r.study_type.clone());
        self.cohort_types.push(r.cohort_type.clone());
        self.collection_types.push(r.collection_type.clone());
        self.backgrounds.push(r.background.clone());
        self.designs.push(r.design.clone());
        self.participantss.push(r.participants.clone());
        self.conditions.push(r.condition.clone());
        self.addit_features.push(r.addit_feature.clone());
        self.websites.push(r.website.clone());
        self.relateds.push(r.related.clone());
        self.resources_availables.push(r.resources_available.clone());
        self.has_datas.push(r.has_data);     // false, true
        self.is_public_uses.push(r.is_public_use);
        self.has_specimenss.push(r.has_specimens);       // false, true          
        self.materials.push(r.material.clone());
        self.data_comm_rests.push(r.data_comm_rest.clone()); 
        self.data_area_rests.push(r.data_area_rest.clone());
        self.irb_app_reqds.push(r.irb_app_reqd.clone());
        self.spec_comm_rests.push(r.spec_comm_rest.clone());
        self.spec_gen_rests.push(r.spec_gen_rest.clone());
        self.spec_gen_alloweds.push(r.spec_gen_allowed.clone());
        self.spec_nongen_rests.push(r.spec_nongen_rest.clone());
        self.specific_rests.push(r.specific_rest.clone());
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.names.shrink_to_fit();
        self.acronyms.shrink_to_fit();
        self.acc_nums.shrink_to_fit();
        self.biolincc_urls.shrink_to_fit();
        self.ctrial_idss.shrink_to_fit();
        self.study_periods.shrink_to_fit();
        self.study_types.shrink_to_fit();
        self.cohort_types.shrink_to_fit();
        self.collection_types.shrink_to_fit();
        self.backgrounds.shrink_to_fit();
        self.designs.shrink_to_fit();
        self.participantss.shrink_to_fit();
        self.conditions.shrink_to_fit();
        self.addit_features.shrink_to_fit();
        self.websites.shrink_to_fit();
        self.relateds.shrink_to_fit();
        self.resources_availables.shrink_to_fit();
        self.has_datas.shrink_to_fit();  
        self.is_public_uses.shrink_to_fit();
        self.has_specimenss.shrink_to_fit();   
        self.materials.shrink_to_fit();
        self.data_comm_rests.shrink_to_fit();
        self.data_area_rests.shrink_to_fit();
        self.irb_app_reqds.shrink_to_fit();
        self.spec_comm_rests.shrink_to_fit();
        self.spec_gen_rests.shrink_to_fit();
        self.spec_gen_alloweds.shrink_to_fit();
        self.spec_nongen_rests.shrink_to_fit();
        self.specific_rests.shrink_to_fit();
    }
  
    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_summaries(name, acronym, accession_number, biolincc_url, ctrial_ids, study_period, study_type, cohort_type, 
                        collection_type, background, design, participants, condition, addit_feature, website, related, 
                        resources_available, has_data, is_public_use, has_specimens, material, data_comm_rest, data_area_rest,
                        irb_app_reqd, spec_comm_rest, spec_gen_rest, spec_gen_allowed, spec_nongen_rest, specific_rest) 
                        SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                        $7::text[], $8::text[], $9::text[], $10::text[], $11::text[], $12::text[], $13::text[], $14::text[],
                        $15::text[], $16::text[], $17::text[], $18::bool[], $19::bool[], $20::bool[], $21::text[],
                        $22::bool[], $23::bool[], $24::bool[], $25::bool[], $26::bool[], $27::text[], $28::bool[], $29::text[])"#;

        sqlx::query(sql)
        .bind(&self.names)
        .bind(&self.acronyms)
        .bind(&self.acc_nums)
        .bind(&self.biolincc_urls)
        .bind(&self.ctrial_idss)
        .bind(&self.study_periods)
        .bind(&self.study_types)
        .bind(&self.cohort_types)
        .bind(&self.collection_types)
        .bind(&self.backgrounds)
        .bind(&self.designs)
        .bind(&self.participantss)
        .bind(&self.conditions)
        .bind(&self.addit_features)
        .bind(&self.websites)
        .bind(&self.relateds)
        .bind(&self.resources_availables)
        .bind(&self.has_datas)
        .bind(&self.is_public_uses)
        .bind(&self.has_specimenss)    
        .bind(&self.materials)
        .bind(&self.data_comm_rests)
        .bind(&self.data_area_rests)
        .bind(&self.irb_app_reqds)
        .bind(&self.spec_comm_rests)
        .bind(&self.spec_gen_rests)
        .bind(&self.spec_gen_alloweds)
        .bind(&self.spec_nongen_rests)
        .bind(&self.specific_rests)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

