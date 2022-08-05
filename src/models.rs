//! JSON API models for deserializing data from the various HTTP APIs.
//!
//! No docstrings are provided for these models, as they're
//! exclusively intended to be returned by this crate's public
//! functions and the fields match those that come from the APIs,
//! except when underlines are included to improve field
//! readability and adhere to Rust's styling guidelines.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusData {
    pub v3: Vec<String>,
    pub transceivers: Vec<String>,
    pub servers: Vec<String>,
    pub servers_sweatbox: Vec<String>,
    pub servers_all: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub data: StatusData,
    pub user: Vec<String>,
    pub metar: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlightPlan {
    pub flight_rules: String,
    pub aircraft: String,
    pub aircraft_faa: String,
    pub aircraft_short: String,
    pub departure: String,
    pub arrival: String,
    pub alternate: String,
    pub cruise_tas: String,
    pub altitude: String,
    pub deptime: String,
    pub enroute_time: String,
    pub fuel_time: String,
    pub remarks: String,
    pub route: String,
    pub revision_id: i64,
    pub assigned_transponder: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pilot {
    pub cid: i64,
    pub name: String,
    pub callsign: String,
    pub server: String,
    pub pilot_rating: i8,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: i64,
    pub groundspeed: i64,
    pub transponder: String,
    pub heading: i64,
    pub qnh_i_hg: f64,
    pub qnh_mb: i64,
    pub flight_plan: Option<FlightPlan>,
    pub logon_time: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Controller {
    pub cid: i64,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: i64,
    pub rating: i8,
    pub server: String,
    pub visual_range: i64,
    pub text_atis: Option<Vec<String>>,
    pub last_updated: String,
    pub logon_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralData {
    pub version: i64,
    pub reload: i64,
    pub update: String,
    pub update_timestamp: String,
    pub connected_clients: i64,
    pub unique_users: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Atis {
    pub cid: u64,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: u8,
    pub rating: u8,
    pub server: String,
    pub visual_range: u16,
    pub atis_code: String,
    pub text_atis: Vec<String>,
    pub last_updated: String,
    pub logon_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub ident: String,
    pub hostname_or_ip: String,
    pub location: String,
    pub name: String,
    pub clients_connection_allowed: u16,
    pub client_connections_allowed: bool,
    pub is_sweatbox: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReferenceItem {
    pub id: i8,
    pub short: String,
    pub long: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReferenceNameItem {
    pub id: i8,
    pub short_name: String,
    pub long_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct V3ResponseData {
    pub general: GeneralData,
    pub pilots: Vec<Pilot>,
    pub controllers: Vec<Controller>,
    pub atis: Vec<Atis>,
    pub servers: Vec<Server>,
    pub facilities: Vec<ReferenceItem>,
    pub ratings: Vec<ReferenceItem>,
    pub pilot_ratings: Vec<ReferenceNameItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransceiverEntry {
    pub id: u16,
    pub frequency: u64,
    #[serde(rename = "latDeg")]
    pub lat_deg: f64,
    #[serde(rename = "lonDeg")]
    pub lon_deg: f64,
    #[serde(rename = "heightMslM")]
    pub height_msl_m: f64,
    #[serde(rename = "heightAglM")]
    pub height_agl_m: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransceiverResponseEntry {
    pub callsign: String,
    pub transceivers: Vec<TransceiverEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRatingsSimple {
    id: String,
    rating: i8,
    pilot_rating: i8,
    susp_date: Option<String>,
    reg_date: String,
    region: String,
    division: String,
    subdivision: String,
    #[serde(rename = "lastratingchange")]
    last_rating_change: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RatingsTimeData {
    pub id: f64,
    pub atc: f64,
    pub pilot: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub i1: f64,
    pub i2: f64,
    pub i3: f64,
    pub sup: f64,
    pub adm: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectionEntry {
    id: u64,
    vatsim_id: String,
    #[serde(rename = "type")]
    connection_type: u16,
    rating: i8,
    callsign: String,
    start: String,
    end: Option<String>,
    server: String,
}

/// A paginated response wrapper. Includes a count of items,
/// potential links to next/previous pages, and a list of results.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    count: u64,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtcSessionEntry {
    connection_id: u64,
    start: String,
    end: String,
    server: String,
    vatsim_id: String,
    #[serde(rename = "type")]
    session_type: u16,
    rating: i8,
    callsign: String,
    minutes_on_callsign: String,
    total_minutes_on_callsign: f64,
    total_aircraft_tracked: u64,
    total_aircraft_seen: u64,
    total_flights_amended: u64,
    total_handoffs_initiated: u64,
    total_handoffs_received: u64,
    total_handoffs_refused: u64,
    total_squawks_assigned: u64,
    total_cruisealts_modified: u64,
    total_tempalts_modified: u64,
    #[serde(rename = "total_scratchpadmods")]
    total_scratchpad_mods: u64,
    #[serde(rename = "aircrafttracked")]
    aircraft_tracked: u64,
    #[serde(rename = "aircraftseen")]
    aircraft_seen: u64,
    #[serde(rename = "flightsamended")]
    flights_amended: u64,
    #[serde(rename = "handoffsinitiated")]
    handoffs_initiated: u64,
    #[serde(rename = "handoffsreceived")]
    handoffs_received: u64,
    #[serde(rename = "handoffsrefused")]
    handoffs_refused: u64,
    #[serde(rename = "squawksassigned")]
    squawks_assigned: u64,
    #[serde(rename = "cruisealtsmodified")]
    cruise_alts_modified: u64,
    #[serde(rename = "tempaltsmodified")]
    temp_alts_modified: u64,
    #[serde(rename = "scratchpadmods")]
    scratchpad_mods: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestFlightPlans {
    //
}
