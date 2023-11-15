use std::{collections::HashSet};
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
};


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct BasicServiceStorage {
    pub patients: HashSet<Patient>,
    pub hospitals: HashSet<Hospital>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct IllnessRecords {
    pub illness_name: String,
    pub illness_date: String,
    pub illness_description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct PrescriptionRecords {
    pub prescription_name: String,
    pub prescription_date: String,
    pub prescription_description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct AllergyRecords {
    pub allergy_name: String,
    pub allergy_date: String,
    pub allergy_description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct PastTreatmentRecords {
    pub past_treatment_name: String,
    pub past_treatment_date: String,
    pub past_treatment_description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct XrayRecords {
    pub xray_name: String,
    pub xray_date: String,
    pub xray_description: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct Patient {
    pub ssn: u32,
    pub name: String,
    pub age: String,
    pub weight: String,
    pub height: String,
    pub blood_type: String,
    pub illness_records: Vec<IllnessRecords>,
    pub prescription_records: Vec<PrescriptionRecords>,
    pub allergy_records: Vec<AllergyRecords>,
    pub past_treatment_records: Vec<PastTreatmentRecords>,
    pub xray_records: Vec<XrayRecords>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct Hospital {
    pub hospital_name: String,
    pub hospital_address: String,
}


/*
    boy kilo yaş kan grubu isim vb.
    Hastalık kayıtları
    Reçete kayıtları
    Alerji kayıtları
    Geçmiş tedavi kayıtları
    Röntgen kayıtları
    Tahlil kayıtları
    Ameliyat kayıtları
    İlaç kayıtları
    Aşı kayıtları
*/