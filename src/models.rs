use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub id: i8,
    
    #[serde(alias = "tenKhoaHoc")]
    pub ten_khoa_hoc: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Subject {
    pub id: i8,
    #[serde(alias = "tenMonHoc")]
    pub ten_mon_hoc: Option<String>,
    #[serde(alias = "moTa")]
    pub mo_ta: Option<String>,
    #[serde(alias = "khoaHocId")]
    pub khoa_hoc_id: i8,
    #[serde(alias = "khoaHoc")]
    pub khoa_hoc: Course,
}