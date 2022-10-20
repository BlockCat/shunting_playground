use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
pub struct LocationCoord {
    #[serde(rename = "X")]
    x: f32,
    #[serde(rename = "Y")]
    y: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LocationReader(HashMap<String, Vec<LocationCoord>>);

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingYard {
    #[serde(rename = "DependencePath")]
    dependencePath: String,
    #[serde(rename = "Value")]
    value: ShuntingYardValue,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingYardValue {
    #[serde(rename = "AlleMogelijkeTaken")]
    possibleTasks: Vec<Task>,

    #[serde(rename = "StandaardTaken")]
    standardTasks: Vec<Task>,

    #[serde(rename = "Sporen")]
    rails: Vec<Rail>,

    #[serde(rename = "Wissels")]
    switches: Vec<Switch>,

    #[serde(rename = "EngelseWissels")]
    englishSwitch: Vec<EnglishSwitch>,

    #[serde(rename = "Stootblokken")]
    #[serde(rename = "Kruisingen")]
    #[serde(rename = "HalfEngelseWissels")]
    #[serde(rename = "Faciliteiten")]
    #[serde(rename = "Eindes")]
    #[serde(rename = "ConstantesABCVoorRangeertijd")]
    #[serde(rename = "Medewerkers")]
    #[serde(rename = "Professies")]
    #[serde(rename = "LoopAfstanden")]
    





}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Task {
    #[serde(rename = "Taaktype")]
    taskType: String,
    #[serde(rename = "Afkorting")]
    taskShort: String,
    #[serde(rename = "Normtijd")]
    normalTime: Duration,
}
