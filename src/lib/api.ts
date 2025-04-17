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