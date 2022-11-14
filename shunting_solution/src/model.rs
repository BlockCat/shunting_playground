use daggy::Dag;
use easy_error::{Error, ResultExt};
use petgraph::stable_graph::DefaultIx;
use serde::{de, Deserialize};
use std::{collections::HashMap, io::Read};

pub fn read_pos_json<R: Read>(reader: R) -> Result<PosJson, Error> {
    serde_json::from_reader(reader).context("Could not")
}

#[derive(Debug, Deserialize)]
pub struct PosJson {
    pub actions: Vec<PosAction>,
    pub matching: Vec<PosMatching>,
    pub graph: Vec<PosEdge>,
    pub feasible: bool,
}

#[derive(Debug, Deserialize)]
pub struct PosAction {
    #[serde(rename = "id", deserialize_with = "parse_string")]
    pub id: usize,

    #[serde(rename = "suggestedStartingTime", deserialize_with = "parse_string")]
    pub suggested_starting_time: usize,

    #[serde(rename = "suggestedFinishingTime", deserialize_with = "parse_string")]
    pub suggested_finishing_time: usize,

    #[serde(rename = "minimumDuration", deserialize_with = "parse_string_f32")]
    pub minimum_duration: f32,

    #[serde(rename = "trainUnitIds", deserialize_with = "parse_string_vec")]
    pub train_unit_ids: Vec<usize>,

    #[serde(rename = "movement")]
    pub movement: Option<PosMovement>,

    #[serde(rename = "task")]
    pub task: Option<PosTask>,

    #[serde(rename = "staffIds")]
    pub staff_ids: Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct PosMovement {
    #[serde(rename = "path", deserialize_with = "parse_string_vec")]
    pub path: Vec<usize>,
    #[serde(rename = "fromSide")]
    pub from_side: PosTrackSide,
    #[serde(rename = "toSide")]
    pub to_side: PosTrackSide,
    #[serde(rename = "order")]
    pub order: usize,
    #[serde(rename = "parkingSide")]
    pub parking_side: PosTrackSide,
}

#[derive(Debug, Deserialize)]
pub struct PosTask {
    #[serde(rename = "type", deserialize_with = "parse_task_type")]
    pub kind: PosTaskType,
    #[serde(rename = "location", deserialize_with = "parse_string")]
    pub location: usize,
    #[serde(rename = "facilities")]
    pub facilities: Vec<PosFacility>,
    #[serde(rename = "arrivalSide")]
    pub arrival_side: PosTrackSide,
    #[serde(rename = "arrivalDirection")]
    pub arrival_direction: PosTrackSide,
    #[serde(rename = "departureSide")]
    pub departure_side: PosTrackSide,
    #[serde(rename = "trainUnitIds", deserialize_with = "parse_string_vec")]
    pub train_unit_ids: Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct PosFacility {
    #[serde(deserialize_with = "parse_string")]
    pub id: usize,
    pub index: usize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum PosTaskType {
    Predefined(String),
    Other(String),
}

#[derive(Debug, Deserialize)]
pub enum PosTrackSide {
    A,
    NoSide,
    B,
}

#[derive(Debug, Deserialize)]
pub struct PosMatching {
    #[serde(rename = "trainUnitId", deserialize_with = "parse_string")]
    pub train_unit_id: usize,
    #[serde(rename = "trainOutId", deserialize_with = "parse_string")]
    pub train_out_id: usize,
    #[serde(rename = "position")]
    pub position: usize,
}

#[derive(Debug, Deserialize)]
pub struct PosEdge {
    #[serde(rename = "preActionId", deserialize_with = "parse_string")]
    pub pre_action_id: usize,
    #[serde(rename = "postActionId", deserialize_with = "parse_string")]
    pub post_action_id: usize,
    #[serde(rename = "minimumTimeLag")]
    pub minimum_time_lag: f32,
}

fn parse_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    value.parse().map_err(|_| {
        de::Error::invalid_value(de::Unexpected::Str(&value), &"String parseable to number")
    })
}

fn parse_string_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    value.parse().map_err(|_| {
        de::Error::invalid_value(de::Unexpected::Str(&value), &"String parseable to number")
    })
}

fn parse_string_vec<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value: Vec<String> = Vec::deserialize(deserializer)?;
    value
        .iter()
        .map(|value| value.parse::<usize>())
        .try_collect::<Vec<_>>()
        .map_err(de::Error::custom)
}

fn parse_task_type<'de, D>(deserializer: D) -> Result<PosTaskType, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PosTaskRawType {
        #[serde(rename = "other")]
        other: Option<String>,
        #[serde(rename = "predefined")]
        predefined: Option<String>,
    }
    let value = PosTaskRawType::deserialize(deserializer)?;

    assert!(
        !(value.predefined.is_some() && value.other.is_some()),
        "Should contain one or the other"
    );

    match (value.other, value.predefined) {
        (None, Some(x)) => Ok(PosTaskType::Predefined(x)),
        (Some(x), None) => Ok(PosTaskType::Other(x)),

        (other, predefined) => panic!(
            "Should contain exactly one. other: {:?}, predefined: {:?}",
            other, predefined
        ),
    }
}

impl PosJson {
    pub fn json_graph(self) -> daggy::Dag<PosAction, f32, DefaultIx> {
        let mut graph: Dag<PosAction, f32, u32> =
            daggy::Dag::with_capacity(self.actions.len(), self.graph.len());
        let nodes = self
            .actions
            .into_iter()
            .map(|x| (x.id, graph.add_node(x)))
            .collect::<HashMap<_, _>>();

        for edge in self.graph.into_iter() {
            let source = nodes[&edge.pre_action_id];
            let target = nodes[&edge.post_action_id];
            graph
                .add_edge(source, target, edge.minimum_time_lag)
                .expect("Could not add edge");
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use shunting_location::ShuntingYard;

    use crate::Solution;

    use super::PosJson;
    use std::io::Cursor;

    #[test]
    fn read_test() {
        let reader = Cursor::new(include_str!("../../data/pos.json"));

        let location = Cursor::new(include_str!("../../data/location.json"));
        let yard = ShuntingYard::read(location);
    
        let pos: PosJson = super::read_pos_json(reader).expect("Could not parse");

        for movement in pos.actions.iter().filter_map(|x| x.movement.as_ref()) {
            let path = &movement.path;

            for a in path.windows(2).filter(|x| x[0] == x[1]) {
                println!("double? turn {:?}", a);
            }
        }

        let sol = Solution::from(pos, &yard);

        // println!("{:?}", Dot::with_config(&graph, &[]));

        // let _solution: Solution = pos.into();
    }
}
