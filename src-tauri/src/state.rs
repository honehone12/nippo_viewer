use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct CachedCode {
    pub(crate) code: String
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Token {
    pub(crate) access_token: String,
    pub(crate) expires_in: u64,
    pub(crate) id_token: String,
    pub(crate) refresh_token: String,
    pub(crate) token_type: String,
}

impl Token {
    pub(crate) fn refresh(&mut self, refresh: TokenRefresh) {
        self.access_token = refresh.access_token;
        self.id_token = refresh.id_token;
        self.token_type = refresh.token_type;
        self.expires_in = refresh.expires_in;
    }
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct TokenRefresh {
    pub(crate) access_token: String,
    pub(crate) expires_in: u64,
    pub(crate) id_token: String,
    pub(crate) token_type: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct CachedViewer {
    pub(crate) org_id: Uuid,
    pub(crate) tkn: Token,
    pub(crate) exp: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) name: String
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Users {
    pub(crate) admin: bool,
    pub(crate) users: Vec<User>,
    pub(crate) invitables: Vec<User>
}

#[derive(Default)]
pub(crate) struct CachedUsers {
    pub(crate) org_id: Uuid,
    pub(crate) users: Users,
}

impl CachedUsers {
    pub(crate) fn has(&self, org_id: &Uuid) -> bool {
        return &self.org_id == org_id && !self.users.users.is_empty();
    }
}

#[derive(Default)]
pub(crate) struct CachedQuery {
    pub(crate) user: String,
    pub(crate) y: String,
    pub(crate) m: String,
    pub(crate) report: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct MorningCall {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) caller: String,
    pub(crate) name: String,
    pub(crate) car_number: String,
    pub(crate) method: u32,
    pub(crate) using_alc_checker: bool,
    pub(crate) alc_check: bool,
    pub(crate) alc_photo: String,
    pub(crate) health_check: bool,
    pub(crate) car_check: bool,
    pub(crate) note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct EveningCall {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) caller: String,
    pub(crate) name: String,
    pub(crate) car_number: String,
    pub(crate) method: u32,
    pub(crate) using_alc_checker: bool,
    pub(crate) alc_check: bool,
    pub(crate) alc_photo: String,
    pub(crate) note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Calls {
    pub(crate) morning_calls: Vec<MorningCall>,
    pub(crate) evening_calls: Vec<EveningCall>,
}

#[derive(Default)]
pub(crate) struct CachedCalls {
    pub(crate) user: String,
    pub(crate) y: String,
    pub(crate) m: String,
    pub(crate) calls: Calls,
}

impl CachedCalls {
    pub(crate) fn has(&self, user: &str, y: &str, m: &str) -> bool {
        if self.user != user || self.y != y || self.m != m {
            return false;
        }

        !self.calls.morning_calls.is_empty() || !self.calls.evening_calls.is_empty()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct DailyReportMini {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct DailyReportFull {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) updated_at: DateTime<FixedOffset>,
    pub(crate) name: String,
    pub(crate) car_number: String,
    pub(crate) meter_photo: String,
    pub(crate) trouble: String,
    pub(crate) note: String,
}

#[derive(Default)]
pub(crate) struct CachedDailyReports {
    pub(crate) user: String,
    pub(crate) y: String,
    pub(crate) m: String,
    pub(crate) reports: Vec<DailyReportMini>,
}

impl CachedDailyReports {
    pub(crate) fn has(&self, user: &str, y: &str, m: &str) -> bool {
        if self.user != user || self.y != y || self.m != m {
            return false;
        }

        !self.reports.is_empty()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Location {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) label: String,
    pub(crate) address: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) short_note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Waiting {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) updated_at: DateTime<FixedOffset>,
    pub(crate) label: String,
    pub(crate) address: String,
    pub(crate) note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Loading {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) updated_at: DateTime<FixedOffset>,
    pub(crate) label: String,
    pub(crate) address: String,
    pub(crate) shipping_check: bool,
    pub(crate) note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Resting {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    pub(crate) updated_at: DateTime<FixedOffset>,
    pub(crate) label: String,
    pub(crate) address: String,
    pub(crate) short_note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct DailyReportPrint {
    pub(crate) daily_report: Option<DailyReportFull>,
    pub(crate) morning_call: Option<MorningCall>,
    pub(crate) evening_call: Option<EveningCall>,
    pub(crate) locations: Vec<Location>,
    pub(crate) waitings: Vec<Waiting>,
    pub(crate) loadings: Vec<Loading>,
    pub(crate) restings: Vec<Resting>,
}

#[derive(Default)]
pub(crate) struct CachedDailyReportPrint {
    pub(crate) report: String,
    pub(crate) print: DailyReportPrint,
}

impl CachedDailyReportPrint {
    pub(crate) fn has(&self, report: &str) -> bool {
        &self.report == report && self.print.daily_report.is_some()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Photos {
    pub(crate) morning_alc: String,
    pub(crate) evening_alc: String,
    pub(crate) meter: String,
}

#[derive(Default)]
pub(crate) struct CachedPhotos {
    pub(crate) report: String,
    pub(crate) photos: Photos,
}

impl CachedPhotos {
    pub(crate) fn has(&self, report: &str) -> bool {
        if &self.report != report {
            return false;
        }

        return !self.photos.morning_alc.is_empty()
            || !self.photos.evening_alc.is_empty()
            || !self.photos.meter.is_empty();
    }
}
