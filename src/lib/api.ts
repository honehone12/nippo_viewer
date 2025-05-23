"use strict";

export interface User {
    id:         string;
    name:       string;
}

export interface UserInfo {
    user:       User;
    invitable:  boolean;
    promotable: boolean;
}

export interface Users {
    admin:     boolean;
    users:     UserInfo[];
}

export interface MorningCall {
    id:                    string;
    created_at:            string;
    caller:                string;
    name:                  string;
    license_expiration:    string;
    car_number:            string;
    inspection_expiration: string;
    method:                number;
    using_alc_checker:    boolean;
    alc_check:            boolean;
    alc_photo:             string;
    health_check:         boolean;
    car_check:            boolean;
    note:                  string;
}

export interface EveningCall {
    id:                 string;
    created_at:         string;
    caller:             string;
    name:               string;
    car_number:         string;
    method:             number;
    using_alc_checker: boolean;
    alc_check:         boolean;
    alc_photo:          string;
    note:               string;
}

export interface Calls {
    morning_calls: MorningCall[];
    evening_calls: EveningCall[];
}

export interface DailyReportMini {
    id:         string;
    created_at: string;
    updated_at: string;
}

export interface DailyReportFull {
    id:                  string;
    created_at:          string;
    updated_at:          string;
    name:                string;
    car_number:          string;
    morning_meter:       number;
    morning_meter_photo: string;
    evening_meter:       number;
    evening_meter_photo: string;
    non_duty_distance:   number;
    trouble:             string;
    note:                string;
}

export interface Site {
    id:         string;
    created_at: string;
    name:       string;
    short_note: string;
}

export interface Location {
    id:         string;
    created_at: string;
    label:      string;
    address:    string;
    latitude:   number;
    longitude:  number;
    short_note: string;
}

export interface Waiting {
    id:         string;
    created_at: string;
    updated_at: string;
    label:      string;
    address:    string;
    note:       string;
}

export interface Loading {
    id:              string;
    created_at:      string;
    updated_at:      string;
    label:           string;
    address:         string;
    shipping_check: boolean;
    note:            string;
}

export interface Resting {
    id:         string;
    created_at: string;
    label:      string;
    address:    string;
    short_note: string;
}

export interface DailyReportPrint {
    daily_report: DailyReportFull | null;
    morning_call:     MorningCall | null;
    evening_call:     EveningCall | null;
    sites:                        Site[];
    locations:                Location[];
    waitings:                  Waiting[];
    loadings:                  Loading[];
    restings:                  Resting[];
}

export interface Photos {
    morning_alc: string;
    evening_alc: string;
    morning_mtr: string;
    evening_mtr: string;
}
