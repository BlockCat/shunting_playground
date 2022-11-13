use crate::Solution;
use shunting_location::ShuntingYard;

pub fn validate_solution(
    _solution: &Solution,
    _yard: &ShuntingYard,
) -> Result<(), Vec<SolutionConflict>> {
    let conflicts = Vec::new();

    // let facility_intervals = facilitiy_intervals(solution, yard);
    // let track_intervals = track_intervals(solution, yard);

    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(conflicts)
    }
}

// fn facilitiy_intervals(
//     solution: &Solution,
//     yard: &ShuntingYard,
// ) -> HashMap<FacilityId, Lapper<usize, ()>> {
//     let mut map: HashMap<FacilityId, Vec<Interval<usize, ()>>> =
//         HashMap::with_capacity(yard.facilities.len());

//     for (index, node) in solution.nodes() {
//         if let NodeAction::Task {
//             kind,
//             location,
//             facilities,
//             arrival_side,
//             arrival_direction,
//             departure_side,
//             train_units,
//         } = node.kind
//         {
//             let interval = Interval {
//                 start: node.start_time,
//                 stop: node.start_time + node.duration,
//                 val: (),
//             };
//             for facility in &facilities {
//                 map.entry(*facility)
//                     .or_insert_with(|| Vec::new())
//                     .push(interval.clone());
//             }
//         }
//     }

//     map.into_iter()
//         .map(|(id, intervals)| (id, Lapper::new(intervals)))
//         .collect()
// }

// fn track_intervals(solution: &Solution, yard: &ShuntingYard) -> HashMap<u32, Lapper<usize, ()>> {
//     let mut map: HashMap<u32, Vec<Interval<usize, ()>>> =
//         HashMap::with_capacity(yard.graph.node_count());

//     for (index, node) in solution.nodes() {
//         if let NodeAction::Movement {
//             path,
//             from_side,
//             to_side,
//             parking_side,
//             order,
//         } = node.kind
//         {
//             let interval = Interval {
//                 start: node.start_time,
//                 stop: node.start_time + node.duration,
//                 val: (),
//             };
//             for track_id in &path {
//                 map.entry(*track_id)
//                     .or_insert_with(|| Vec::new())
//                     .push(interval);
//             }
//         }
//     }

//     map.into_iter()
//         .map(|(id, intervals)| (id, Lapper::new(intervals)))
//         .collect()
// }

pub enum SolutionConflict {
    FacilityOverCapacity,
    TrackOverCapacity,
    LateDeparture,
    MissingService,
    TrackLocked,
    Combination,
    TrainBlockingMovement,
}
