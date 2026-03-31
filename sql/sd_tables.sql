SET client_min_messages TO WARNING; 
create schema if not exists sd;

drop table if exists sd.study_summaries;
create table sd.study_summaries (
    name                     varchar
  , acronym                  varchar    primary key
  , accession_number         varchar
  , biolincc_url             varchar
  , ctrial_ids               varchar
  , study_period             varchar
  , study_type               varchar
  , cohort_type              varchar
  , collection_type          varchar
  , background               varchar
  , design                   varchar
  , participants             varchar
  , condition                varchar
  , addit_feature            varchar
  , website                  varchar
  , related                  varchar
  , resources_available      varchar
  , has_data                 boolean
  , is_public_use            boolean
  , has_specimens            boolean
  , material                 varchar
  , data_comm_rest           boolean
  , data_area_rest           boolean
  , irb_app_reqd             boolean
  , spec_comm_rest           boolean
  , spec_gen_rest            boolean
  , spec_gen_allowed         varchar
  , spec_nongen_rest         boolean
  , specific_rest            varchar
);
create index ss_acro on sd.study_summaries(acronym);


drop table if exists sd.studies;
create table sd.studies (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , submit_date              varchar
  , approval_date            varchar
  , study_title              varchar
  , scientific_title         varchar
  , utn                      varchar
  , trial_acronym            varchar
  , linked_study             varchar
  , study_type               varchar
  , patient_registry         varchar
  , registry_followup        varchar
  , registry_followup_type   varchar
  , primary_sponsor_type     varchar
  , primary_sponsor_name     varchar
  , primary_sponsor_country  varchar
  , ethics_status            varchar
  , brief_summary            varchar
  , trial_website            varchar
  , publication              varchar
  , public_notes             varchar
);
create index tid on sd.studies(trial_id);
create index sd_sid on sd.studies(actrn_id);


drop table if exists sd.study_lifecycles;
create table sd.study_lifecycles (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , antic_start_date         varchar
  , actual_start_date        varchar
  , antic_end_date           varchar
  , actual_end_date          varchar
  , antic_last_visit_date    varchar
  , actual_last_visit_date   varchar
  , recruitment_status       varchar
  , data_analysis            varchar
  , withdrawn_reason         varchar
  , withdrawn_reason_other   varchar
  , recruitment_country      varchar
  , recruitment_state        varchar
);
create index lc_tid on sd.study_lifecycles(trial_id);
create index lc_sid on sd.study_lifecycles(actrn_id);


drop table if exists sd.study_features;
create table sd.study_features (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , interventions            varchar
  , comparator               varchar
  , control                  varchar
  , purpose                  varchar
  , allocation               varchar
  , concealment              varchar
  , sequencing               varchar
  , masking                  varchar
  , assignment               varchar
  , other_design_features    varchar
  , endpoint                 varchar
  , phase                    varchar
  , stat_methods             varchar
  , masking_participants     varchar
  , masking_clinicians       varchar
  , masking_assessors        varchar
  , masking_analysts         varchar
  , obs_purpose              varchar
  , obs_duration             varchar
  , obs_selection            varchar
  , obs_timing               varchar
);
create index sf_tid on sd.study_features(trial_id);
create index sf_sid on sd.study_features(actrn_id);


drop table if exists sd.study_participants;
create table sd.study_participants (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , inclusion_criteria       varchar
  , min_age                  varchar
  , min_age_type             varchar
  , max_age                  varchar
  , max_age_type             varchar
  , gender                   varchar
  , healthy_volunteers       varchar
  , exclusion_criteria       varchar
  , target_sample_size       varchar
  , final_sample_size        varchar
  , current_sample_size      varchar
);
create index sp_tid on sd.study_participants(trial_id);
create index sp_sid on sd.study_participants(actrn_id);


drop table if exists sd.secondary_ids;
create table sd.secondary_ids (
    trial_id                 int
  , sec_id                   varchar
);
create index sec_ids_id on sd.secondary_ids(trial_id);


drop table if exists sd.health_conditions;
create table sd.health_conditions (
    trial_id                 int
  , health_condition         varchar
);
create index health_conditions_id on sd.health_conditions(trial_id);


drop table if exists sd.condition_codes;
create table sd.condition_codes (
    trial_id                 int
  , condition_category       varchar
  , condition_code           varchar
);
create index condition_codes_id on sd.condition_codes(trial_id);


drop table if exists sd.intervention_codes;
create table sd.intervention_codes (
    trial_id                 int
  , intervention_code        varchar
);
create index intervention_codes_id on sd.intervention_codes(trial_id);


drop table if exists sd.primary_outcomes;
create table sd.primary_outcomes (
    trial_id                 int
  , outcome                  varchar
  , outcome_assessment       varchar
  , timepoint                varchar
);
create index primary_outcomes_id on sd.primary_outcomes(trial_id);


drop table if exists sd.secondary_outcomes;
create table sd.secondary_outcomes (
    trial_id                 int
  , outcome                  varchar
  , outcome_assessment       varchar
  , timepoint                varchar
);
create index secondary_outcomes_id on sd.secondary_outcomes(trial_id);


drop table if exists sd.hospitals;
create table sd.hospitals (
    trial_id                 int
  , hospital                 varchar
  , location                 varchar
);
create index hospitals_id on sd.hospitals(trial_id);


drop table if exists sd.other_countries;
create table sd.other_countries (
    trial_id                 int
  , country                  varchar
  , state                    varchar
);
create index other_countries_id on sd.other_countries(trial_id);


drop table if exists sd.funding_sources;
create table sd.funding_sources (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index funding_sources_id on sd.funding_sources(trial_id);


drop table if exists sd.secondary_sponsors;
create table sd.secondary_sponsors (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index secondary_sponsors_id on sd.secondary_sponsors(trial_id);


drop table if exists sd.other_collaborators;
create table sd.other_collaborators (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index other_collaborators_id on sd.other_collaborators(trial_id);


drop table if exists sd.ethics_committees;
create table sd.ethics_committees (
    trial_id                 int
  , name                     varchar
  , address                  varchar
  , country                  varchar
  , submit_date              varchar
  , approval_date            varchar
  , hrec_approval_id         varchar
);
create index ethics_committees_id on sd.ethics_committees(trial_id);


drop table if exists sd.contacts;
create table sd.contacts (
    trial_id                 int
  , type                     varchar
  , title                    varchar
  , name                     varchar
  , address                  varchar
  , country                  varchar
  , phone                    varchar
  , fax                      varchar
  , email                    varchar
);
create index contacts_id on sd.contacts(trial_id);


drop table if exists sd.data_sharing_statements;
create table sd.data_sharing_statements (
    trial_id                 int
  , ipd_availability         varchar
  , available_to_whom        varchar
  , availability_conditions  varchar
  , data_to_be_shared        varchar
  , for_what_analyses_types  varchar
  , timeframe_from           varchar
  , timeframe_to             varchar
  , mechanism                varchar
  , extra_considerations     varchar
);
create index data_sharing_statements_id on sd.data_sharing_statements(trial_id);


drop table if exists sd.supporting_documents;
create table sd.supporting_documents (
    trial_id                 int
  , type                     varchar
  , citation                 varchar
  , link                     varchar
  , email                    varchar
  , details                  varchar
  , attachment               varchar
);
create index supporting_documents_id on sd.supporting_documents(trial_id);


drop table if exists sd.study_results;
create table sd.study_results (
    trial_id                 int
  , type                     varchar
  , is_peer_reviewed         varchar
  , doi      	               varchar
  , citations_or_details     varchar
  , attachment               varchar
);
create index study_results_id on sd.study_results(trial_id);


drop table if exists sd.external_publications;
create table sd.external_publications (
    trial_id                 int
  , source                   varchar
  , doi                      varchar
  , title                    varchar
  , year_of_publication      varchar
);
create index external_publications_id on sd.external_publications(trial_id);


SET client_min_messages TO NOTICE; 

