// tools/lunar-lite-reference/scripts/generate-four-pillars-fixtures.mjs
//
// Generates tests/fixtures/four_pillars.json from lunar-lite@0.2.8 (the reference
// for getHeavenlyStemAndEarthlyBranchBySolarDate). Each case carries the solar
// date, 时辰 index, options, and the expected four pillars (stem/branch slugs that
// match the crate's serde `snake_case` encoding).
//
// Run: npm install && npm run generate-four-pillars-fixtures

import { writeFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { getHeavenlyStemAndEarthlyBranchBySolarDate } from "lunar-lite";
import { Solar } from "lunar-typescript";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRoot = resolve(__dirname, "../../..");

const TABLE_MIN_YEAR = 1850;
const TABLE_MAX_YEAR = 2150;

const GAN_SLUG = {
    甲: "jia", 乙: "yi", 丙: "bing", 丁: "ding", 戊: "wu",
    己: "ji", 庚: "geng", 辛: "xin", 壬: "ren", 癸: "gui",
};
const ZHI_SLUG = {
    子: "zi", 丑: "chou", 寅: "yin", 卯: "mao", 辰: "chen", 巳: "si",
    午: "wu", 未: "wei", 申: "shen", 酉: "you", 戌: "xu", 亥: "hai",
};

const JIE = ["小寒", "立春", "惊蛰", "清明", "立夏", "芒种", "小暑", "立秋", "白露", "寒露", "立冬", "大雪"];

const ALL_OPTIONS = [
    { year: "normal", month: "normal" },
    { year: "exact", month: "normal" },
    { year: "normal", month: "exact" },
    { year: "exact", month: "exact" },
];
const EXACT = { year: "exact", month: "exact" };

const pad = (n) => String(n).padStart(2, "0");
const pillar = ([gan, zhi]) => ({ stem: GAN_SLUG[gan], branch: ZHI_SLUG[zhi] });

function lunarYearOf(y, m, d) {
    return Solar.fromYmd(y, m, d).getLunar().getYear();
}

function epochSeconds(y, m, d, secondOfDay) {
    return Date.UTC(y, m - 1, d) / 1000 + secondOfDay;
}

function synthInstant(y, m, d, ti) {
    const hour = Math.max(ti * 2 - 1, 0);
    return epochSeconds(y, m, d, hour * 3600 + 30 * 60);
}

function shiftDay(y, m, d, delta) {
    const dt = new Date(Date.UTC(y, m - 1, d));
    dt.setUTCDate(dt.getUTCDate() + delta);
    return [dt.getUTCFullYear(), dt.getUTCMonth() + 1, dt.getUTCDate()];
}

const cases = [];

function add(y, m, d, ti, options) {
    // Normal options rely on the lunar year; skip what the crate cannot support.
    const usesNormal = options.year === "normal" || options.month === "normal";
    if (usesNormal) {
        const ly = lunarYearOf(y, m, d);
        if (ly < TABLE_MIN_YEAR || ly > TABLE_MAX_YEAR) {
            return;
        }
    }
    const dateStr = `${y}-${pad(m)}-${pad(d)}`;
    const r = getHeavenlyStemAndEarthlyBranchBySolarDate(dateStr, ti, options);
    cases.push({
        solar: { year: y, month: m, day: d },
        time_index: ti,
        options,
        expected: {
            yearly: pillar(r.yearly),
            monthly: pillar(r.monthly),
            daily: pillar(r.daily),
            hourly: pillar(r.hourly),
        },
    });
}

function addAllOptions(y, m, d, ti) {
    for (const opts of ALL_OPTIONS) {
        add(y, m, d, ti, opts);
    }
}

// 1. Ordinary dates across the whole range, all four option combinations.
for (const year of [1850, 1900, 1984, 2000, 2024, 2100, 2150]) {
    addAllOptions(year, 4, 5, 6);
    addAllOptions(year, 10, 20, 3);
}

// 2. Around 立春: before date, on-date straddling the exact second, after date.
//    Locks year:Exact (date granularity) vs month:Exact (second granularity).
for (const year of [1984, 2000, 2024, 2100]) {
    const liChun = Solar.fromYmd(year, 6, 1).getLunar().getJieQiTable()["立春"];
    const lm = liChun.getMonth();
    const ld = liChun.getDay();
    const exactInstant = epochSeconds(
        year, lm, ld,
        liChun.getHour() * 3600 + liChun.getMinute() * 60 + liChun.getSecond(),
    );

    addAllOptions(...shiftDay(year, lm, ld, -1), 6);
    addAllOptions(...shiftDay(year, lm, ld, 1), 6);

    // On the 立春 calendar date, one synth time before and one at/after the exact second.
    let before = null;
    let after = null;
    for (let ti = 0; ti <= 12; ti++) {
        const inst = synthInstant(year, lm, ld, ti);
        if (inst < exactInstant) before = ti;
        if (inst >= exactInstant && after === null) after = ti;
    }
    if (before !== null) {
        add(year, lm, ld, before, EXACT);
    }
    if (after !== null) {
        add(year, lm, ld, after, EXACT);
    }
}

// 3. Around each of the 12 Jie (year 2000), month:Exact: before day, on-date
//    before/after the exact second, after day.
{
    const year = 2000;
    const table = Solar.fromYmd(year, 6, 1).getLunar().getJieQiTable();
    for (const name of JIE) {
        const s = table[name];
        const jm = s.getMonth();
        const jd = s.getDay();
        const exactInstant = epochSeconds(
            year, jm, jd,
            s.getHour() * 3600 + s.getMinute() * 60 + s.getSecond(),
        );

        add(...shiftDay(year, jm, jd, -1), 6, EXACT);
        add(...shiftDay(year, jm, jd, 1), 6, EXACT);

        let before = null;
        let after = null;
        for (let ti = 0; ti <= 12; ti++) {
            const inst = synthInstant(year, jm, jd, ti);
            if (inst < exactInstant) before = ti;
            if (inst >= exactInstant && after === null) after = ti;
        }
        if (before !== null) {
            add(year, jm, jd, before, EXACT);
        }
        if (after !== null) {
            add(year, jm, jd, after, EXACT);
        }
    }
}

// 4. Early 子 vs late 子 on the same date (daily + hourly rollover).
addAllOptions(2000, 8, 16, 0);
addAllOptions(2000, 8, 16, 12);

// 5. All 13 time indices on one known day.
for (let ti = 0; ti <= 12; ti++) {
    add(2000, 8, 16, ti, EXACT);
}

// 6. Boundary years (normal auto-skipped where the lunar year is out of range).
addAllOptions(1850, 1, 1, 6);
addAllOptions(1850, 12, 31, 6);
addAllOptions(2150, 1, 1, 6);
addAllOptions(2150, 12, 31, 6);

const outPath = resolve(repoRoot, "tests/fixtures/four_pillars.json");
await writeFile(outPath, JSON.stringify(cases, null, 2) + "\n", "utf8");
console.log(`Generated ${outPath} (${cases.length} cases)`);
