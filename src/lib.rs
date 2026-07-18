//! Seeded spatial simulation for Grafik living diagrams.
//!
//! The [`simulate`] interface accepts one bounded result plus browser-measured geometry and returns
//! a complete, renderer-neutral [`Trace`]. The implementation performs no I/O, reads no ambient
//! randomness or clock, and has no knowledge of the DOM.

use core::fmt;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::grafik_trace;

const CLEARANCE: f64 = 8.0;
const MINIMUM_GAP: f64 = CLEARANCE * 2.0;
const GEOMETRY_EPSILON: f64 = 0.001;

/// A point in the browser's CSS-pixel coordinate space.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f64,
    /// Vertical coordinate.
    pub y: f64,
}

impl Point {
    /// Creates a point from finite CSS-pixel coordinates.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

/// A rectangular panel and spatial-field exclusion zone.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rect {
    /// Left coordinate.
    pub x: f64,
    /// Top coordinate.
    pub y: f64,
    /// Width in CSS pixels.
    pub width: f64,
    /// Height in CSS pixels.
    pub height: f64,
}

impl Rect {
    /// Creates a panel rectangle.
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn right(self) -> f64 {
        self.x + self.width
    }

    fn bottom(self) -> f64 {
        self.y + self.height
    }

    fn is_valid(self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
            && self.right().is_finite()
            && self.bottom().is_finite()
    }
}

/// A selectable edge of a panel.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Edge {
    /// Top horizontal edge.
    Top,
    /// Right vertical edge.
    Right,
    /// Bottom horizontal edge.
    Bottom,
    /// Left vertical edge.
    Left,
}

/// A spatial-field attachment point on a panel edge.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct EdgePort {
    /// Coordinate of the attachment point.
    pub point: Point,
    /// Panel edge containing the point.
    pub edge: Edge,
}

impl EdgePort {
    /// Creates an edge port.
    pub const fn new(point: Point, edge: Edge) -> Self {
        Self { point, edge }
    }
}

/// Provenance of a bounded final result supplied to the simulation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultSource {
    /// A generated result that performed no infrastructure work.
    Simulated,
    /// An intentionally published result captured from an earlier experiment run.
    Recorded,
}

/// Final disposition of one bounded operation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinalDisposition {
    /// Local rejection before any receiver attempt; not a receiver result.
    NotAttempted,
    /// Receiver facts established the defined successful outcome.
    Succeeded,
    /// Receiver facts established the defined failed outcome.
    Failed,
    /// Bounded observation established neither success nor failure.
    Unknown,
}

/// Theme-aware palette role for a decorative uncertainty mark.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionTone {
    /// Primary outcome accent role.
    Accent,
    /// Warning role.
    Warning,
    /// Informational role.
    Info,
}

/// Complete controlled input for one connector simulation.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct SimulationInput {
    /// Nonzero deterministic replay seed.
    pub seed: u64,
    /// Whether the final result is simulated or from an intentionally published recording.
    pub result_source: ResultSource,
    /// Bounded final operation disposition preserved without reinterpretation.
    pub final_disposition: FinalDisposition,
    /// Hero panel exclusion rectangle.
    pub hero: Rect,
    /// Receipt panel exclusion rectangle.
    pub receipt: Rect,
    /// Browser-measured region containing the readable outcome text.
    pub outcome_region: Rect,
    /// Selected attachment on the hero panel.
    pub hero_port: EdgePort,
    /// Selected attachment on the receipt panel.
    pub receipt_port: EdgePort,
}

/// One renderer-neutral evolution event in a spatial trace.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SpatialEvent {
    /// Marks the beginning of one bounded outcome profile.
    OutcomeStarted {
        /// Simulation time in milliseconds.
        at_ms: u32,
    },
    /// Declares the selected connector endpoints.
    ConnectorStarted {
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Hero edge port.
        from: Point,
        /// Receipt edge port.
        to: Point,
    },
    /// Makes one segment visible from its start toward its end.
    SegmentGrew {
        /// Stable zero-based segment index.
        index: u8,
        /// Seeded progress emphasis from 1 through 3.
        weight: u8,
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Growth duration in milliseconds.
        duration_ms: u16,
        /// Segment start.
        from: Point,
        /// Segment end.
        to: Point,
    },
    /// Pulses once through a completely grown successful connector.
    SuccessPulsed {
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Pulse traversal duration in milliseconds.
        duration_ms: u16,
        /// Seeded emphasis from 1 through 3.
        intensity: u8,
    },
    /// Glitches one decorative layer behind failed outcome text.
    FailureGlitched {
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Glitch lifetime in milliseconds.
        duration_ms: u16,
        /// Horizontal decorative displacement in CSS pixels.
        offset_x: i8,
        /// Vertical decorative displacement in CSS pixels.
        offset_y: i8,
        /// Number of decorative glitch strips.
        strips: u8,
    },
    /// Places one decorative uncertainty symbol for a bounded lifetime.
    QuestionMarkAppeared {
        /// Stable zero-based mark index.
        index: u8,
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Mark lifetime in milliseconds.
        lifetime_ms: u16,
        /// Absolute coordinate in the browser spatial field.
        point: Point,
        /// Browser palette role rather than a literal color.
        tone: QuestionTone,
    },
    /// Removes one previously grown segment from the connector leaf.
    SegmentRetracted {
        /// Index of the segment being removed.
        index: u8,
        /// Simulation time in milliseconds.
        at_ms: u32,
        /// Retraction duration in milliseconds.
        duration_ms: u16,
    },
    /// Marks the end of a complete grown-and-retracted connector trace.
    ConnectorFinished {
        /// Final connector simulation time in milliseconds.
        at_ms: u32,
    },
    /// Marks the end of one complete outcome profile.
    OutcomeFinished {
        /// Final profile simulation time in milliseconds.
        at_ms: u32,
    },
}

/// A complete replayable spatial evolution.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Trace {
    /// Seed used for every weighted choice.
    pub seed: u64,
    /// Whether the supplied final result was simulated or recorded.
    pub result_source: ResultSource,
    /// Bounded final disposition preserved from the semantic input.
    pub final_disposition: FinalDisposition,
    /// Ordered renderer-neutral events.
    pub events: Vec<SpatialEvent>,
}

impl Trace {
    /// Serializes this trace for a rendering adapter.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::Serialization`] if JSON serialization fails.
    pub fn to_json(&self) -> Result<String, SimulationError> {
        serde_json::to_string(self)
            .map_err(|error| SimulationError::Serialization(error.to_string()))
    }
}

/// A validation or serialization failure at the simulation interface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimulationError {
    /// Seed zero cannot identify a published replay.
    ZeroSeed,
    /// A named rectangle or port contains non-finite or non-positive geometry.
    InvalidGeometry(&'static str),
    /// The selected port is not on the required panel edge.
    InvalidPort(&'static str),
    /// The tracer requires the hero above the receipt with enough route clearance.
    InsufficientVerticalGap,
    /// A complete trace could not be encoded as JSON.
    Serialization(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSeed => formatter.write_str("seed must be nonzero"),
            Self::InvalidGeometry(name) => write!(formatter, "invalid {name} geometry"),
            Self::InvalidPort(name) => write!(formatter, "invalid {name} edge port"),
            Self::InsufficientVerticalGap => {
                formatter.write_str("hero and receipt need at least 16 CSS pixels of vertical gap")
            }
            Self::Serialization(message) => {
                write!(formatter, "trace serialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for SimulationError {}

/// Evolves one seeded, bounded outcome profile into renderer-neutral events.
///
/// # Errors
///
/// Returns [`SimulationError`] when the seed, rectangles, selected edge ports, or vertical
/// clearance violate the tracer contract.
pub fn simulate(input: SimulationInput) -> Result<Trace, SimulationError> {
    validate(input)?;

    let mut random = SeededRandom::new(input.seed);
    let points = route(input, &mut random);
    let mut events = Vec::with_capacity(points.len() * 2 + 1);
    let mut time_ms = 0_u32;

    events.push(SpatialEvent::OutcomeStarted { at_ms: time_ms });

    let mut segment_count = 0_u8;
    if input.final_disposition == FinalDisposition::Succeeded {
        events.push(SpatialEvent::ConnectorStarted {
            at_ms: time_ms,
            from: input.hero_port.point,
            to: input.receipt_port.point,
        });
        for pair in points.windows(2) {
            let [from, to] = pair else {
                continue;
            };
            let weight = random.progress_weight();
            let duration_ms = random.range_u16(120, 260);
            events.push(SpatialEvent::SegmentGrew {
                index: segment_count,
                weight,
                at_ms: time_ms,
                duration_ms,
                from: *from,
                to: *to,
            });
            time_ms += u32::from(duration_ms);
            segment_count += 1;
        }

        let duration_ms = random.range_u16(360, 720);
        events.push(SpatialEvent::SuccessPulsed {
            at_ms: time_ms,
            duration_ms,
            intensity: random.range_u8(1, 3),
        });
        time_ms += u32::from(duration_ms);

        for index in (0..segment_count).rev() {
            let duration_ms = random.range_u16(90, 180);
            events.push(SpatialEvent::SegmentRetracted {
                index,
                at_ms: time_ms,
                duration_ms,
            });
            time_ms += u32::from(duration_ms);
        }
        events.push(SpatialEvent::ConnectorFinished { at_ms: time_ms });
    } else if input.final_disposition == FinalDisposition::Failed {
        let duration_ms = random.range_u16(140, 260);
        events.push(SpatialEvent::FailureGlitched {
            at_ms: time_ms,
            duration_ms,
            offset_x: random.offset_3(),
            offset_y: random.offset_3(),
            strips: random.range_u8(1, 3),
        });
        time_ms += u32::from(duration_ms);
    } else if input.final_disposition == FinalDisposition::Unknown {
        let rate = random.range_u8(1, 3);
        let normal_count = rate * 2;
        let interval_ms = 1_000 / u32::from(rate);
        let rare_cluster = random.one_in(32);
        let cluster_after = normal_count / 2;
        let mut mark_index = 0_u8;
        for normal_index in 0..normal_count {
            let at_ms = u32::from(normal_index) * interval_ms;
            time_ms = time_ms.max(push_question_mark(
                &mut events,
                &mut random,
                input.outcome_region,
                mark_index,
                at_ms,
            ));
            mark_index += 1;
            if rare_cluster && normal_index == cluster_after {
                for _ in 0..2 {
                    time_ms = time_ms.max(push_question_mark(
                        &mut events,
                        &mut random,
                        input.outcome_region,
                        mark_index,
                        at_ms,
                    ));
                    mark_index += 1;
                }
            }
        }
    }

    events.push(SpatialEvent::OutcomeFinished { at_ms: time_ms });
    Ok(Trace {
        seed: input.seed,
        result_source: input.result_source,
        final_disposition: input.final_disposition,
        events,
    })
}

fn push_question_mark(
    events: &mut Vec<SpatialEvent>,
    random: &mut SeededRandom,
    region: Rect,
    index: u8,
    at_ms: u32,
) -> u32 {
    let lifetime_ms = random.range_u16(450, 900);
    let point = Point::new(
        random.range_f64(region.x, region.right()),
        random.range_f64(region.y, region.bottom()),
    );
    events.push(SpatialEvent::QuestionMarkAppeared {
        index,
        at_ms,
        lifetime_ms,
        point,
        tone: random.question_tone(),
    });
    at_ms + u32::from(lifetime_ms)
}

fn validate(input: SimulationInput) -> Result<(), SimulationError> {
    if input.seed == 0 {
        return Err(SimulationError::ZeroSeed);
    }
    if !input.hero.is_valid() {
        return Err(SimulationError::InvalidGeometry("hero panel"));
    }
    if !input.receipt.is_valid() {
        return Err(SimulationError::InvalidGeometry("receipt panel"));
    }
    if !input.outcome_region.is_valid() || !rect_contains(input.receipt, input.outcome_region) {
        return Err(SimulationError::InvalidGeometry("outcome region"));
    }
    if !input.hero_port.point.is_finite() {
        return Err(SimulationError::InvalidGeometry("hero port"));
    }
    if !input.receipt_port.point.is_finite() {
        return Err(SimulationError::InvalidGeometry("receipt port"));
    }
    if !port_is_on_edge(input.hero, input.hero_port, Edge::Bottom) {
        return Err(SimulationError::InvalidPort("hero"));
    }
    if !port_is_on_edge(input.receipt, input.receipt_port, Edge::Top) {
        return Err(SimulationError::InvalidPort("receipt"));
    }
    if input.hero.bottom() + MINIMUM_GAP > input.receipt.y {
        return Err(SimulationError::InsufficientVerticalGap);
    }
    Ok(())
}

fn rect_contains(outer: Rect, inner: Rect) -> bool {
    inner.x + GEOMETRY_EPSILON >= outer.x
        && inner.y + GEOMETRY_EPSILON >= outer.y
        && inner.right() <= outer.right() + GEOMETRY_EPSILON
        && inner.bottom() <= outer.bottom() + GEOMETRY_EPSILON
}

fn port_is_on_edge(rect: Rect, port: EdgePort, required_edge: Edge) -> bool {
    if port.edge != required_edge {
        return false;
    }

    let on_horizontal_span = port.point.x + GEOMETRY_EPSILON >= rect.x
        && port.point.x <= rect.right() + GEOMETRY_EPSILON;
    let required_y = match required_edge {
        Edge::Top => rect.y,
        Edge::Bottom => rect.bottom(),
        Edge::Right | Edge::Left => return false,
    };
    on_horizontal_span && (port.point.y - required_y).abs() <= GEOMETRY_EPSILON
}

fn route(input: SimulationInput, random: &mut SeededRandom) -> Vec<Point> {
    let hero_out = Point::new(input.hero_port.point.x, input.hero.bottom() + CLEARANCE);
    let receipt_out = Point::new(input.receipt_port.point.x, input.receipt.y - CLEARANCE);
    let midpoint = input.hero_port.point.x.midpoint(input.receipt_port.point.x);
    let offsets = [-48.0, -24.0, 24.0, 48.0];
    let lane_x = midpoint + offsets[random.lane_index()];
    let candidates = [
        input.hero_port.point,
        hero_out,
        Point::new(lane_x, hero_out.y),
        Point::new(lane_x, receipt_out.y),
        receipt_out,
        input.receipt_port.point,
    ];
    let mut points = Vec::with_capacity(candidates.len());
    for point in candidates {
        if points
            .last()
            .is_none_or(|previous| !same_point(*previous, point))
        {
            points.push(point);
        }
    }
    points
}

fn same_point(left: Point, right: Point) -> bool {
    (left.x - right.x).abs() <= GEOMETRY_EPSILON && (left.y - right.y).abs() <= GEOMETRY_EPSILON
}

struct SeededRandom {
    state: u64,
}

impl SeededRandom {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn lane_index(&mut self) -> usize {
        usize::from(self.next_u64().to_le_bytes()[0] & 0b11)
    }

    fn progress_weight(&mut self) -> u8 {
        match self.next_u64().to_le_bytes()[0] % 6 {
            0 | 1 => 1,
            2..=4 => 2,
            _ => 3,
        }
    }

    fn one_in(&mut self, denominator: u8) -> bool {
        self.next_u64().to_le_bytes()[0].is_multiple_of(denominator)
    }

    fn question_tone(&mut self) -> QuestionTone {
        match self.next_u64().to_le_bytes()[0] % 3 {
            0 => QuestionTone::Accent,
            1 => QuestionTone::Warning,
            _ => QuestionTone::Info,
        }
    }

    fn range_f64(&mut self, minimum: f64, maximum: f64) -> f64 {
        let bytes = self.next_u64().to_le_bytes();
        let sample = f64::from(u16::from_le_bytes([bytes[0], bytes[1]]));
        minimum + (maximum - minimum) * sample / f64::from(u16::MAX)
    }

    fn offset_3(&mut self) -> i8 {
        match self.next_u64().to_le_bytes()[0] % 7 {
            0 => -3,
            1 => -2,
            2 => -1,
            3 => 0,
            4 => 1,
            5 => 2,
            _ => 3,
        }
    }

    fn range_u8(&mut self, minimum: u8, maximum: u8) -> u8 {
        let width = maximum - minimum + 1;
        minimum + self.next_u64().to_le_bytes()[0] % width
    }

    fn range_u16(&mut self, minimum: u16, maximum: u16) -> u16 {
        let width = maximum - minimum + 1;
        let sample = u16::from(self.next_u64().to_le_bytes()[0]);
        minimum + sample % width
    }
}

#[cfg(test)]
mod tests {
    use super::{
        simulate, Edge, EdgePort, FinalDisposition, Point, QuestionTone, Rect, ResultSource,
        SimulationError, SimulationInput, SpatialEvent,
    };

    #[test]
    fn same_seed_and_geometry_replay_the_same_trace() -> Result<(), Box<dyn std::error::Error>> {
        let input = SimulationInput {
            seed: 42,
            result_source: ResultSource::Simulated,
            final_disposition: FinalDisposition::Succeeded,
            hero: Rect::new(80.0, 40.0, 320.0, 160.0),
            receipt: Rect::new(520.0, 420.0, 360.0, 220.0),
            outcome_region: Rect::new(560.0, 460.0, 160.0, 32.0),
            hero_port: EdgePort::new(Point::new(240.0, 200.0), Edge::Bottom),
            receipt_port: EdgePort::new(Point::new(700.0, 420.0), Edge::Top),
        };

        let first = simulate(input)?;
        let second = simulate(input)?;

        assert_eq!(first, second);
        assert_eq!(first.to_json()?, second.to_json()?);
        Ok(())
    }

    #[test]
    fn final_dispositions_remain_distinct_in_the_public_trace() -> Result<(), SimulationError> {
        let dispositions = [
            FinalDisposition::NotAttempted,
            FinalDisposition::Succeeded,
            FinalDisposition::Failed,
            FinalDisposition::Unknown,
        ];

        for source in [ResultSource::Simulated, ResultSource::Recorded] {
            for disposition in dispositions {
                let mut input = valid_input(73);
                input.result_source = source;
                input.final_disposition = disposition;
                let trace = simulate(input)?;
                let source_json = match source {
                    ResultSource::Simulated => "\"result_source\":\"simulated\"",
                    ResultSource::Recorded => "\"result_source\":\"recorded\"",
                };
                let disposition_json = match disposition {
                    FinalDisposition::NotAttempted => "NOT_ATTEMPTED",
                    FinalDisposition::Succeeded => "SUCCEEDED",
                    FinalDisposition::Failed => "FAILED",
                    FinalDisposition::Unknown => "UNKNOWN",
                };

                assert_eq!(trace.result_source, source);
                assert_eq!(trace.final_disposition, disposition);
                assert!(trace.to_json()?.contains(source_json));
                assert!(trace.to_json()?.contains(disposition_json));
            }
        }
        Ok(())
    }

    #[test]
    fn succeeded_uses_weighted_progress_while_other_profiles_remain_empty(
    ) -> Result<(), SimulationError> {
        let succeeded = simulate(valid_input(1_337))?;
        let weights = succeeded.events.iter().filter_map(|event| match event {
            SpatialEvent::SegmentGrew { weight, .. } => Some(*weight),
            _ => None,
        });
        let weights = weights.collect::<Vec<_>>();

        assert!(!weights.is_empty());
        assert!(weights.iter().all(|weight| (1..=3).contains(weight)));
        let pulses = succeeded.events.iter().filter_map(|event| match event {
            SpatialEvent::SuccessPulsed {
                duration_ms,
                intensity,
                ..
            } => Some((*duration_ms, *intensity)),
            _ => None,
        });
        let pulses = pulses.collect::<Vec<_>>();
        assert_eq!(pulses.len(), 1);
        assert!((360..=720).contains(&pulses[0].0));
        assert!((1..=3).contains(&pulses[0].1));

        for disposition in [
            FinalDisposition::NotAttempted,
            FinalDisposition::Failed,
            FinalDisposition::Unknown,
        ] {
            let mut input = valid_input(1_337);
            input.final_disposition = disposition;
            let trace = simulate(input)?;
            assert!(trace.events.iter().all(|event| !matches!(
                event,
                SpatialEvent::SegmentGrew { .. } | SpatialEvent::SegmentRetracted { .. }
            )));
        }
        Ok(())
    }

    #[test]
    fn failed_emits_one_bounded_text_local_glitch() -> Result<(), SimulationError> {
        let mut input = valid_input(9_001);
        input.final_disposition = FinalDisposition::Failed;
        let trace = simulate(input)?;
        let glitches = trace.events.iter().filter_map(|event| match event {
            SpatialEvent::FailureGlitched {
                duration_ms,
                offset_x,
                offset_y,
                strips,
                ..
            } => Some((*duration_ms, *offset_x, *offset_y, *strips)),
            _ => None,
        });
        let glitches = glitches.collect::<Vec<_>>();

        assert_eq!(glitches.len(), 1);
        assert!((140..=260).contains(&glitches[0].0));
        assert!((-3..=3).contains(&glitches[0].1));
        assert!((-3..=3).contains(&glitches[0].2));
        assert!((1..=3).contains(&glitches[0].3));
        Ok(())
    }

    #[test]
    fn unknown_emits_a_bounded_seeded_question_mark_burst() -> Result<(), SimulationError> {
        let mut input = valid_input(4_242);
        input.final_disposition = FinalDisposition::Unknown;
        let trace = simulate(input)?;
        let marks = question_marks(&trace);

        assert!((2..=6).contains(&marks.len()));
        assert!(marks.iter().all(|(_, at_ms, lifetime_ms, point)| {
            *at_ms < 2_000
                && (450..=900).contains(lifetime_ms)
                && point.x >= input.outcome_region.x
                && point.x <= input.outcome_region.right()
                && point.y >= input.outcome_region.y
                && point.y <= input.outcome_region.bottom()
        }));
        assert!(maximum_live_marks(&marks) <= 3);

        let mut rare_marks = None;
        for seed in 1..=4_096 {
            let mut candidate = valid_input(seed);
            candidate.final_disposition = FinalDisposition::Unknown;
            let candidate_trace = simulate(candidate)?;
            let candidate_marks = question_marks(&candidate_trace);
            if candidate_marks.len() > 6 {
                rare_marks = Some(candidate_marks);
                break;
            }
        }
        assert!(rare_marks
            .as_ref()
            .is_some_and(|marks| { marks.len() <= 8 && maximum_live_marks(marks) <= 5 }));
        Ok(())
    }

    fn question_marks(trace: &super::Trace) -> Vec<(u8, u32, u16, Point)> {
        trace
            .events
            .iter()
            .filter_map(|event| match event {
                SpatialEvent::QuestionMarkAppeared {
                    index,
                    at_ms,
                    lifetime_ms,
                    point,
                    ..
                } => Some((*index, *at_ms, *lifetime_ms, *point)),
                _ => None,
            })
            .collect()
    }

    fn maximum_live_marks(marks: &[(u8, u32, u16, Point)]) -> usize {
        marks
            .iter()
            .map(|(_, at_ms, _, _)| {
                marks
                    .iter()
                    .filter(|(_, other_at_ms, lifetime_ms, _)| {
                        other_at_ms <= at_ms && *at_ms < *other_at_ms + u32::from(*lifetime_ms)
                    })
                    .count()
            })
            .max()
            .unwrap_or_default()
    }

    #[test]
    fn seeds_vary_profile_parameters_and_question_palette_roles() -> Result<(), SimulationError> {
        let mut pulses = Vec::new();
        let mut glitches = Vec::new();
        let mut tones = [false; 3];

        for seed in 1..=64 {
            let success = simulate(valid_input(seed))?;
            for event in success.events {
                if let SpatialEvent::SuccessPulsed {
                    duration_ms,
                    intensity,
                    ..
                } = event
                {
                    let profile = (duration_ms, intensity);
                    if !pulses.contains(&profile) {
                        pulses.push(profile);
                    }
                }
            }

            let mut failed_input = valid_input(seed);
            failed_input.final_disposition = FinalDisposition::Failed;
            let failed = simulate(failed_input)?;
            for event in failed.events {
                if let SpatialEvent::FailureGlitched {
                    duration_ms,
                    offset_x,
                    offset_y,
                    strips,
                    ..
                } = event
                {
                    let profile = (duration_ms, offset_x, offset_y, strips);
                    if !glitches.contains(&profile) {
                        glitches.push(profile);
                    }
                }
            }

            let mut unknown_input = valid_input(seed);
            unknown_input.final_disposition = FinalDisposition::Unknown;
            let unknown = simulate(unknown_input)?;
            for event in unknown.events {
                if let SpatialEvent::QuestionMarkAppeared { tone, .. } = event {
                    tones[match tone {
                        QuestionTone::Accent => 0,
                        QuestionTone::Warning => 1,
                        QuestionTone::Info => 2,
                    }] = true;
                }
            }
        }

        assert!(pulses.len() > 1);
        assert!(glitches.len() > 1);
        assert!(tones.into_iter().all(|seen| seen));
        Ok(())
    }

    #[test]
    fn every_profile_has_generic_boundaries_without_false_connector_events(
    ) -> Result<(), SimulationError> {
        for disposition in [
            FinalDisposition::NotAttempted,
            FinalDisposition::Succeeded,
            FinalDisposition::Failed,
            FinalDisposition::Unknown,
        ] {
            let mut input = valid_input(808);
            input.final_disposition = disposition;
            let trace = simulate(input)?;
            assert!(matches!(
                trace.events.first(),
                Some(SpatialEvent::OutcomeStarted { at_ms: 0 })
            ));
            assert!(matches!(
                trace.events.last(),
                Some(SpatialEvent::OutcomeFinished { .. })
            ));
            let has_connector = trace
                .events
                .iter()
                .any(|event| matches!(event, SpatialEvent::ConnectorStarted { .. }));
            assert_eq!(has_connector, disposition == FinalDisposition::Succeeded);
            if disposition == FinalDisposition::NotAttempted {
                assert_eq!(trace.events.len(), 2);
            }
        }
        Ok(())
    }

    #[test]
    fn connector_retracts_from_the_leaf() -> Result<(), Box<dyn std::error::Error>> {
        let trace = simulate(valid_input(81))?;
        let grown = trace.events.iter().filter_map(|event| match event {
            SpatialEvent::SegmentGrew { index, .. } => Some(*index),
            _ => None,
        });
        let retracted = trace.events.iter().filter_map(|event| match event {
            SpatialEvent::SegmentRetracted { index, .. } => Some(*index),
            _ => None,
        });
        let mut expected = grown.collect::<Vec<_>>();
        expected.reverse();

        assert_eq!(retracted.collect::<Vec<_>>(), expected);
        Ok(())
    }

    #[test]
    fn connector_events_stay_within_tracer_budgets() -> Result<(), Box<dyn std::error::Error>> {
        let trace = simulate(valid_input(229))?;
        let grown = trace.events.iter().filter_map(|event| match event {
            SpatialEvent::SegmentGrew { duration_ms, .. } => Some(*duration_ms),
            _ => None,
        });
        let durations = grown.collect::<Vec<_>>();

        assert!((1..=5).contains(&durations.len()));
        assert!(durations
            .iter()
            .all(|duration| (120..=260).contains(duration)));
        assert!(trace.events.iter().all(|event| match event {
            SpatialEvent::SegmentRetracted { duration_ms, .. } => {
                (90..=180).contains(duration_ms)
            }
            _ => true,
        }));
        Ok(())
    }

    #[test]
    fn invalid_geometry_is_rejected_before_events_exist() {
        let mut zero_seed = valid_input(1);
        zero_seed.seed = 0;
        let mut overlapping = valid_input(1);
        overlapping.receipt.y = 210.0;
        overlapping.receipt_port.point.y = 210.0;
        overlapping.outcome_region.y = 230.0;
        let mut misplaced_port = valid_input(1);
        misplaced_port.hero_port.point.y -= 1.0;

        assert_eq!(simulate(zero_seed), Err(SimulationError::ZeroSeed));
        assert_eq!(
            simulate(overlapping),
            Err(SimulationError::InsufficientVerticalGap)
        );
        assert_eq!(
            simulate(misplaced_port),
            Err(SimulationError::InvalidPort("hero"))
        );
    }

    #[test]
    fn derived_panel_edges_must_remain_finite() {
        let mut input = valid_input(1);
        input.hero.x = f64::MAX;
        input.hero.width = f64::MAX;
        input.hero_port.point.x = f64::MAX;

        assert_eq!(
            simulate(input),
            Err(SimulationError::InvalidGeometry("hero panel"))
        );
    }

    #[test]
    fn grown_segments_preserve_panel_interiors() -> Result<(), Box<dyn std::error::Error>> {
        let input = valid_input(817);
        let trace = simulate(input)?;

        for event in &trace.events {
            if let SpatialEvent::SegmentGrew { from, to, .. } = event {
                assert!(!segment_enters(*from, *to, input.hero));
                assert!(!segment_enters(*from, *to, input.receipt));
            }
        }
        Ok(())
    }

    fn segment_enters(from: Point, to: Point, rect: Rect) -> bool {
        let minimum_x = from.x.min(to.x);
        let maximum_x = from.x.max(to.x);
        let minimum_y = from.y.min(to.y);
        let maximum_y = from.y.max(to.y);
        let horizontal = (from.y - to.y).abs() <= 0.001
            && from.y > rect.y
            && from.y < rect.y + rect.height
            && maximum_x > rect.x
            && minimum_x < rect.x + rect.width;
        let vertical = (from.x - to.x).abs() <= 0.001
            && from.x > rect.x
            && from.x < rect.x + rect.width
            && maximum_y > rect.y
            && minimum_y < rect.y + rect.height;
        horizontal || vertical
    }

    fn valid_input(seed: u64) -> SimulationInput {
        SimulationInput {
            seed,
            result_source: ResultSource::Simulated,
            final_disposition: FinalDisposition::Succeeded,
            hero: Rect::new(80.0, 40.0, 320.0, 160.0),
            receipt: Rect::new(520.0, 420.0, 360.0, 220.0),
            outcome_region: Rect::new(560.0, 460.0, 160.0, 32.0),
            hero_port: EdgePort::new(Point::new(240.0, 200.0), Edge::Bottom),
            receipt_port: EdgePort::new(Point::new(700.0, 420.0), Edge::Top),
        }
    }
}
