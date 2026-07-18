//! Seeded spatial simulation for Grafik living diagrams.
//!
//! The [`simulate`] interface accepts browser-measured geometry and returns a complete,
//! renderer-neutral [`Trace`]. The implementation performs no I/O, reads no ambient randomness or
//! clock, and has no knowledge of the DOM.

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

impl FinalDisposition {
    /// Returns the canonical disposition spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "NOT_ATTEMPTED",
            Self::Succeeded => "SUCCEEDED",
            Self::Failed => "FAILED",
            Self::Unknown => "UNKNOWN",
        }
    }
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
    /// Selected attachment on the hero panel.
    pub hero_port: EdgePort,
    /// Selected attachment on the receipt panel.
    pub receipt_port: EdgePort,
}

impl SimulationInput {
    /// Creates a complete simulation input.
    pub const fn new(
        seed: u64,
        result_source: ResultSource,
        final_disposition: FinalDisposition,
        hero: Rect,
        receipt: Rect,
        hero_port: EdgePort,
        receipt_port: EdgePort,
    ) -> Self {
        Self {
            seed,
            result_source,
            final_disposition,
            hero,
            receipt,
            hero_port,
            receipt_port,
        }
    }
}

/// One renderer-neutral evolution event in a spatial trace.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SpatialEvent {
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
        /// Final simulation time in milliseconds.
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

/// Evolves one seeded connector from growth through leaf-first retraction.
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

    events.push(SpatialEvent::ConnectorStarted {
        at_ms: time_ms,
        from: input.hero_port.point,
        to: input.receipt_port.point,
    });

    let mut segment_count = 0_u8;
    if input.final_disposition == FinalDisposition::Succeeded {
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

        for index in (0..segment_count).rev() {
            let duration_ms = random.range_u16(90, 180);
            events.push(SpatialEvent::SegmentRetracted {
                index,
                at_ms: time_ms,
                duration_ms,
            });
            time_ms += u32::from(duration_ms);
        }
    }

    events.push(SpatialEvent::ConnectorFinished { at_ms: time_ms });
    Ok(Trace {
        seed: input.seed,
        result_source: input.result_source,
        final_disposition: input.final_disposition,
        events,
    })
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

    fn range_u16(&mut self, minimum: u16, maximum: u16) -> u16 {
        let width = maximum - minimum + 1;
        let sample = u16::from(self.next_u64().to_le_bytes()[0]);
        minimum + sample % width
    }
}

#[cfg(test)]
mod tests {
    use super::{
        simulate, Edge, EdgePort, FinalDisposition, Point, Rect, ResultSource, SimulationError,
        SimulationInput, SpatialEvent,
    };

    #[test]
    fn same_seed_and_geometry_replay_the_same_trace() -> Result<(), Box<dyn std::error::Error>> {
        let input = SimulationInput::new(
            42,
            ResultSource::Simulated,
            FinalDisposition::Succeeded,
            Rect::new(80.0, 40.0, 320.0, 160.0),
            Rect::new(520.0, 420.0, 360.0, 220.0),
            EdgePort::new(Point::new(240.0, 200.0), Edge::Bottom),
            EdgePort::new(Point::new(700.0, 420.0), Edge::Top),
        );

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

                assert_eq!(trace.result_source, source);
                assert_eq!(trace.final_disposition, disposition);
                assert!(trace.to_json()?.contains(source_json));
                assert!(trace.to_json()?.contains(disposition.as_str()));
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
        SimulationInput::new(
            seed,
            ResultSource::Simulated,
            FinalDisposition::Succeeded,
            Rect::new(80.0, 40.0, 320.0, 160.0),
            Rect::new(520.0, 420.0, 360.0, 220.0),
            EdgePort::new(Point::new(240.0, 200.0), Edge::Bottom),
            EdgePort::new(Point::new(700.0, 420.0), Edge::Top),
        )
    }
}
