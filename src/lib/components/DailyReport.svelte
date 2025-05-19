<script>
    'use strict'

    import {datetime} from "$lib/display"

    /**
     * @type {{report: import("$lib/api").DailyReportFull}}
     */
    let {report} = $props();

    function dutyDist() {
        const dist = report.evening_meter - report.morning_meter - report.non_duty_distance
        return dist >= 0 ? dist : 0;
    }
</script>

<table class="table">
    <caption class="text-3xl font-bold text-primary mb-10">日報</caption>
    <colgroup>
        <col style="width: 15%;">
        <col style="width: 15%;">
    </colgroup>
    <thead>
        <tr>
            <th>開始日時</th>
            <th>終了日時</th>
            <th>名前</th>
            <th>車両番号</th>
            <th>開始メーター</th>
            <th>終了メーター</th>
            <th>空走行距離</th>
            <th>走行距離</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>{datetime(report.created_at)}</td>
            <td>{datetime(report.updated_at)}</td>
            <td>{report.name}</td>
            <td>{report.car_number}</td>
            <td>{report.morning_meter}</td>
            <td>{report.evening_meter}</td>
            <td>{report.non_duty_distance}</td>
            <td>{dutyDist()}</td>
        </tr>
        <tr>
            <td>遅延・事故</td>
            <td colspan="7">{report.trouble}</td>
        </tr>
        <tr>
            <td>備考</td>
            <td colspan="7">{report.note}</td>
        </tr>
    </tbody>
</table>
