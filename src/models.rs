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
    pub id: u64,
    pub vatsim_id: String,
    #[serde(rename = "type")]
    pub connection_type: u16,
    pub rating: i8,
    pub callsign: String,
    pub start: String,
    pub end: Option<String>,
    pub server: String,
}

/// A paginated response wrapper. Includes a count of items,
/// potential links to next/previous pages, and a list of results.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtcSessionEntry {
    pub connection_id: u64,
    pub start: String,
    pub end: String,
    pub server: String,
    pub vatsim_id: String,
    #[serde(rename = "type")]
    pub session_type: u16,
    pub rating: i8,
    pub callsign: String,
    pub minutes_on_callsign: String,
    pub total_minutes_on_callsign: f64,
    pub total_aircraft_tracked: u64,
    pub total_aircraft_seen: u64,
    pub total_flights_amended: u64,
    pub total_handoffs_initiated: u64,
    pub total_handoffs_received: u64,
    pub total_handoffs_refused: u64,
    pub total_squawks_assigned: u64,
    pub total_cruisealts_modified: u64,
    pub total_tempalts_modified: u64,
    #[serde(rename = "total_scratchpadmods")]
    pub total_scratchpad_mods: u64,
    #[serde(rename = "aircrafttracked")]
    pub aircraft_tracked: u64,
    #[serde(rename = "aircraftseen")]
    pub aircraft_seen: u64,
    #[serde(rename = "flightsamended")]
    pub flights_amended: u64,
    #[serde(rename = "handoffsinitiated")]
    pub handoffs_initiated: u64,
    #[serde(rename = "handoffsreceived")]
    pub handoffs_received: u64,
    #[serde(rename = "handoffsrefused")]
    pub handoffs_refused: u64,
    #[serde(rename = "squawksassigned")]
    pub squawks_assigned: u64,
    #[serde(rename = "cruisealtsmodified")]
    pub cruise_alts_modified: u64,
    #[serde(rename = "tempaltsmodified")]
    pub temp_alts_modified: u64,
    #[serde(rename = "scratchpadmods")]
    pub scratchpad_mods: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestFlightPlans {
    pub id: u64,
    pub connection_id: u64,
    pub vatsim_id: String,
    pub flight_type: String,
    pub callsign: String,
    pub aircraft: String,
    #[serde(rename = "cruisespeed")]
    pub cruise_speed: String,
    pub dep: String,
    pub arr: String,
    pub alt: String,
    pub altitude: String,
    pub rmks: String,
    pub route: String,
    pub deptime: String,
    #[serde(rename = "hrsenroute")]
    pub hrs_enroute: u64,
    #[serde(rename = "minenroute")]
    pub min_enroute: u64,
    #[serde(rename = "hrsfuel")]
    pub hrs_fuel: u64,
    #[serde(rename = "minsfuel")]
    pub min_sfuel: u8,
    pub filed: String,
    #[serde(rename = "assignedsquawk")]
    pub assigned_squawk: String,
    #[serde(rename = "modifiedbycid")]
    pub modified_by_cid: String,
    #[serde(rename = "modifiedbycallsign")]
    pub modified_by_callsign: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub director: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Facility {
    //
}
