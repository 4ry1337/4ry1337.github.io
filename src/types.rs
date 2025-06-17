use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Personal {
    pub name: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub address: Address,
    pub citizenship: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Edcucation {
    pub degree: String,
    pub institution: String,
    pub location: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub gpa: Option<f32>,
    pub gpa_scale: Option<f32>,
    pub status: String,
    pub relevant_coursework: Vec<String>,
    pub honors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub technologies: Vec<String>,
    pub achievements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TechnicalSkill {
    pub name: String,
    pub r#type: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skill {
    pub technical: Vec<TechnicalSkill>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub icon_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub highlights: Vec<String>,
    pub technologies: Vec<String>,
}
