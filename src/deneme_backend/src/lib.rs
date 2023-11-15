mod types;

use types::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ic_cdk::export::Principal;

thread_local! {
    static SERVICE: RefCell<MainService> = RefCell::default();
}

#[derive(Default)]
pub struct MainService {
    pub patients: HashSet<Patient>,
    pub hospitals: HashSet<Hospital>,
}


impl From<BasicServiceStorage> for MainService {
    fn from(stable: BasicServiceStorage) -> MainService {
        let patiens = stable.patients.into_iter().collect();
        let hospitals = stable.hospitals.into_iter().collect();

        MainService {
            patients: patiens,
            hospitals,
        }
    }
}

//QUERIES
#[ic_cdk::update]
fn add_patient(patient: Patient) -> bool {
    SERVICE.with(|service| {
        service.borrow_mut().patients.insert(patient);
    });
    true
}

#[ic_cdk::query]
fn get_patient_by_ssn(ssn: u32) -> Option<Patient> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_hospital_by_name(hospital_name: String) -> Option<Hospital> {
    let mut hospital = None;
    SERVICE.with(|service| {
        for h in service.borrow().hospitals.iter() {
            if h.hospital_name == hospital_name {
                hospital = Some(h.clone());
            }
        }
    });
    hospital
}

// #[ic_cdk::query]
// fn get_patient_general_info(ssn: u32) -> Option<(String, u32, u32, u32)> {
//     let mut patient = None;
//     SERVICE.with(|service| {
//         for p in service.borrow().patients.iter() {
//             if p.ssn == ssn {
//                 patient = Some((p.name.clone(), p.age, p.weight, p.height));
//             }
//         }
//     });
//     patient
// }

#[ic_cdk::query]
fn get_patient_blood_type(ssn: u32) -> Option<String> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.blood_type.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_patient_illness_records(ssn: u32) -> Option<Vec<IllnessRecords>> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.illness_records.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_patient_prescription_records(ssn: u32) -> Option<Vec<PrescriptionRecords>> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.prescription_records.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_patient_allergy_records(ssn: u32) -> Option<Vec<AllergyRecords>> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.allergy_records.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_patient_past_treatment_records(ssn: u32) -> Option<Vec<PastTreatmentRecords>> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.past_treatment_records.clone());
            }
        }
    });
    patient
}

#[ic_cdk::query]
fn get_patient_xray_records(ssn: u32) -> Option<Vec<XrayRecords>> {
    let mut patient = None;
    SERVICE.with(|service| {
        for p in service.borrow().patients.iter() {
            if p.ssn == ssn {
                patient = Some(p.xray_records.clone());
            }
        }
    });
    patient
}