"use strict";

export interface User {
    id: string,
    name: string
}

export interface MorningCall {
    id: string,
    created_at: string,
    caller: string,
    name: string,
    car_number: string,
    method: number,
    using_alc_checker: boolean,
    alc_check: boolean,
    alc_photo: string,
    health_check: boolean,
    car_check: boolean,
    note: string
}

export interface EveningCall {
    id: string,
    created_at: string,
    caller: string,
    name: string,
    car_number: string,
    method: number,
    using_alc_checker: boolean,
    alc_check: boolean,
    alc_photo: string,
    note: string
}

export interface Calls {
    morning_calls: Array<MorningCall>,
    evening_calls: Array<EveningCall>
}

export interface DailyReportMini {
    id: string,
    created_at: string,
    updated_at: string,
}

export interface DailyReportFull {
    created_at: string,
    updated_at: string,
    id: string,
    name: string,
    car_number: string,
    meter_photo: string,
    trouble: string,
    note: string
}

export interface Location {
    id: string,
    created_at: string,
    lable: string,
    address: string,
    latitude: number,
    longitude: number,
    short_note: string
}

export interface Waiting {
    id: string,
    created_at: string,
    updated_at: string,
    lable: string,
    address: string,
    latitude: number,
    longitude: number,
    note: string
}

export interface Loading {
    id: string,
    created_at: string,
    updated_at: string,
    lable: string,
    address: string,
    latitude: number,
    longitude: number,
    shipping_check: boolean,
    note: string
}

export interface Resting {
    id: string,
    created_at: string,
    updated_at: string,
    lable: string,
    address: string,
    latitude: number,
    longitude: number,
    short_note: string
}
