
pub struct StudySumm {
    pub name: String,
    pub acronym: String,
    pub acc_num: Option<String>,
    pub biolincc_url: String,
    pub ctrial_ids: Option<String>,
    pub study_period: Option<String>,
    pub study_type: String,
    pub cohort_type: Option<String>,
    pub collection_type: String,
    pub background: Option<String>,
    pub design: Option<String>,
    pub participants: Option<String>,
    pub condition: Option<String>,
    pub addit_feature: Option<String>,
    pub website: Option<String>,
    pub related: Option<String>,
    pub resources_available: String,
    pub has_data: Option<bool>,     // false, true
    pub is_public_use: Option<bool>,  // Yes, No
    pub has_specimens: Option<bool>,         // false, true          
    pub material: Option<String>,
    pub data_comm_rest: Option<bool>,  // No, Yes, Not Applicable
    pub data_area_rest: Option<bool>,   // No, Yes, Not Applicable
    pub irb_app_reqd: Option<bool>,      // false, true
    pub spec_comm_rest: Option<bool>,     // No, Yes, Not Applicable
    pub spec_gen_rest: Option<bool>,     // No, Yes, Not Applicable
    pub spec_gen_allowed: Option<String>,   // No, Yes, Yes, For Some specimens, Not Applicable
    pub spec_nongen_rest: Option<bool>,  // No, Yes, Not Applicable
    pub specific_rest: Option<String>,   // Includes None, None.
}