# lunar-lite

[![Crates.io Version](https://img.shields.io/crates/v/lunar-lite.svg)](https://crates.io/crates/lunar-lite)
[![Crates.io Downloads](https://img.shields.io/crates/d/lunar-lite.svg)](https://crates.io/crates/lunar-lite)
[![Docs.rs](https://img.shields.io/docsrs/lunar-lite.svg)](https://docs.rs/lunar-lite)
[![CI](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/lunar-lite-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/lunar-lite-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/lunar-lite-rs)
[![License](https://img.shields.io/crates/l/lunar-lite.svg)](https://crates.io/crates/lunar-lite)

一个精巧的 Rust 库，用于中国农历（农历）日期转换与干支计算。

## 功能

`lunar-lite` 支持公历（阳历）与中国农历（阴阳历）之间的相互转换，包括闰月处理、十二时辰索引计算、干支（六十甲子）周期定位，以及四柱（八字）年柱/月柱/日柱/时柱计算。

**支持转换范围：** 公历年份 `1..=9999`。农历月历信息（`leap_month`、
`lunar_month_days`）接受农历年份 `-1..=9999`；完整的农历转公历还要求结果公历日期落在
`1..=9999` 之内，因此最早的农历年份（约 `-1`）会返回 `YearOutOfRange`。
`1582-10-15` 之前的日期按儒略历语义处理，公历改革缺失日期
`1582-10-05..=1582-10-14` 视为无效，与 tyme4rs 保持一致。

## 不支持的功能

参见[非目标](#非目标)。

## 设计理念

`lunar-lite` 力求小巧、确定性强、符合 Rust 惯用风格。它使用一个小型内部天文后端进行朔日与节气计算，日历行为与 tyme4rs 兼容。天文计算内核的部分内容改编自 MIT 许可的 `6tail/tyme4rs`，详见 [`THIRD_PARTY_LICENSES.md`](THIRD_PARTY_LICENSES.md)。

- **农历/公历转换在内部计算天文朔日。** 它不存储逐日公农历映射表，也不暴露 tyme4rs 类型。
- **干支 Exact 月柱在内部计算节气边界。** 在 `Exact` 模式下，月支由最近的节气边界确定，月天干则通过五虎遁从相应岁/年干推算。
- **运行时保持纯 Rust 且轻量。** 运行时用户无需 Node.js、`lunar-typescript` 或 `tyme4rs`。

## 安装

```sh
cargo add lunar-lite
```

## 用法

### 公历 → 农历

```rust
use lunar_lite::{SolarDate, solar_to_lunar};

let solar = SolarDate { year: 2023, month: 1, day: 22 };
let lunar = solar_to_lunar(solar).unwrap();
// LunarDate { year: 2023, month: 1, day: 1, is_leap_month: false }
```

### 农历 → 公历

```rust
use lunar_lite::{LunarDate, lunar_to_solar};

let lunar = LunarDate { year: 2023, month: 2, day: 1, is_leap_month: true };
let solar = lunar_to_solar(lunar).unwrap();
// SolarDate { year: 2023, month: 3, day: 22 }
```

### 闰月规整

```rust
use lunar_lite::{LunarDate, normalize_lunar_date};

// 2024 年没有闰一月——闰月标志会被静默清除。
let date = LunarDate { year: 2024, month: 1, day: 1, is_leap_month: true };
let normalized = normalize_lunar_date(date).unwrap();
// LunarDate { year: 2024, month: 1, day: 1, is_leap_month: false }
```

### 时辰索引

```rust
use lunar_lite::time_index;

assert_eq!(time_index(0, 30).unwrap(), 0);   // 早子时  00:00–00:59
assert_eq!(time_index(1, 0).unwrap(), 1);    // 丑时    01:00–02:59
assert_eq!(time_index(23, 0).unwrap(), 12);  // 晚子时  23:00–23:59
```

时辰对照表：

| 索引 | 地支          | 时间段      |
| ---- | ------------- | ----------- |
| 0    | 子（早子时）  | 00:00–00:59 |
| 1    | 丑            | 01:00–02:59 |
| 2    | 寅            | 03:00–04:59 |
| 3    | 卯            | 05:00–06:59 |
| 4    | 辰            | 07:00–08:59 |
| 5    | 巳            | 09:00–10:59 |
| 6    | 午            | 11:00–12:59 |
| 7    | 未            | 13:00–14:59 |
| 8    | 申            | 15:00–16:59 |
| 9    | 酉            | 17:00–18:59 |
| 10   | 戌            | 19:00–20:59 |
| 11   | 亥            | 21:00–22:59 |
| 12   | 子（晚子时）  | 23:00–23:59 |

子时分为两段：早子时（索引 0）属于当日之始，晚子时（索引 12）属于当日之末。

### 干支纪年

干支纪年将十天干与十二地支两两相配，形成六十个组合（六十甲子）。年柱以 `1984 年 = 甲子` 为基准锚点。

```rust
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};

// 由农历年推算年柱。
let pillar = StemBranch::from_lunar_year(2024);
assert_eq!(pillar.stem(), HeavenlyStem::Jia);    // 甲
assert_eq!(pillar.branch(), EarthlyBranch::Chen); // 辰 -> 甲辰

// 在六十甲子中的位置（0 = 甲子，59 = 癸亥）。
assert_eq!(pillar.cycle_index(), 40);
assert_eq!(StemBranch::from_cycle_index(0).stem(), HeavenlyStem::Jia);

// 有效构造：只接受奇偶性匹配的六十对。
assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).is_ok());
assert!(StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chou).is_err());
```

`HeavenlyStem` 与 `EarthlyBranch` 各自提供 `index`、`from_index` 及循环 `offset` 方法；`HEAVENLY_STEMS` 和 `EARTHLY_BRANCHES` 常量给出标准的循环排列顺序。

### 四柱（八字）

四柱干支 API 根据公历日期和时辰索引，计算年柱、月柱、日柱、时柱（`FourPillars`）。该实现忠实移植自 TypeScript [`lunar-lite`](https://github.com/SylarLong/lunar-lite) 的 `getHeavenlyStemAndEarthlyBranchBySolarDate` 函数，并经过输出验证。

Rust 原生入口为 `four_pillars_from_solar_date`（默认选项）和 `four_pillars_from_solar_date_with_options`。较长的 `get_heavenly_stem_and_earthly_branch_by_solar_date[_with_options]` 名称保留，以说明与 TypeScript 参考实现的对等关系；`HeavenlyStemAndEarthlyBranchDate` 作为 `FourPillars` 的类型别名同样保留。

```rust
use lunar_lite::{
    four_pillars_from_solar_date, four_pillars_from_solar_date_with_options,
    EarthlyBranch, FourPillars, HeavenlyStem, MonthDivide, SolarDate, StemBranchOptions,
    YearDivide,
};

let solar = SolarDate { year: 2000, month: 8, day: 16 };

// 最简调用：默认选项（Exact, Exact，与 TypeScript 参考实现一致）。
// time_index 2 == 寅时（03:00–04:59）。
let pillars: FourPillars = four_pillars_from_solar_date(solar, 2).unwrap();

assert_eq!(pillars.yearly.stem(), HeavenlyStem::Geng);     // 庚辰
assert_eq!(pillars.monthly.branch(), EarthlyBranch::Shen); // 甲申

// 显式指定节气边界方式：
let options = StemBranchOptions { year: YearDivide::Exact, month: MonthDivide::Exact };
let _ = four_pillars_from_solar_date_with_options(solar, 2, options);
```

时辰由 `time_index` 合成（`hour = max(time_index * 2 - 1, 0), minute = 30`），与参考实现保持一致。`time_index` 范围为 `0..=12`，其中 `0`（早子时）和 `12`（晚子时）均映射到子支；`12` 还会将日柱推进到次日（晚子时）。

**年柱 — `YearDivide`：**

- `Normal`：使用农历年，年柱在春节（农历正月初一）更换。
- `Exact`：以立春为边界，按**日期**粒度比较——立春当日及之后视为新年。

**月柱 — `MonthDivide`：**

- `Normal`：通过**农历月份**经五虎遁推算，不使用节气。
- `Exact`：依据 12 节（节气）的**精确秒级**边界推算。

> 月柱在 `Exact` 模式下使用节气，而非农历月份。两种模式有意设计为非对称：`year:Exact` 按日期粒度处理，`month:Exact` 按秒级粒度处理，与参考实现保持一致。

**支持范围：** 四柱计算遵循转换 API 的公历范围：年份 `1..=9999`，不包含 `1582-10-05..=1582-10-14`。

## 闰月

中国农历大约每三年插入一个闰月。`LunarDate` 携带 `is_leap_month: bool` 字段，用于区分闰月和普通月份。

`normalize_lunar_date` 是处理外部传入日期的安全入口：

- 若 `is_leap_month = true` **且**当年该位置确有闰月，则保留原值。
- 若 `is_leap_month = true` **但**当年该位置无闰月，则清除该标志，按普通月处理。
- 规整后会验证实际日数；日期超出范围则返回 `LunarError::InvalidLunarDate`。

`lunar_to_solar` 内部调用 `normalize_lunar_date`，因此传入错误的闰月标志是安全的。

## 错误处理

日期/时间转换函数返回 `Result<_, LunarError>`。
干支有效性验证返回 `Result<_, StemBranchError>`。

| 变体                                     | 含义                                          |
| ---------------------------------------- | --------------------------------------------- |
| `LunarError::InvalidSolarDate`           | 公历日期不合法                                |
| `LunarError::InvalidLunarDate`           | 农历日期结构不合法或日数超出当月天数          |
| `LunarError::YearOutOfRange`             | 年份超出支持的公历或农历转换范围              |
| `LunarError::InvalidTime`                | 小时 > 23 或分钟 > 59                         |
| `LunarError::InvalidTimeIndex`           | 时辰索引超出 0..=12 范围                      |
| `LunarError::SolarTermOutOfRange`        | 公历年份超出支持的节气范围                    |
| `StemBranchError::InvalidStemBranchPair` | 天干与地支不构成有效的六十甲子组合            |

## 参考 fixture

四柱兼容性 fixture 由 `tools/lunar-lite-reference/scripts/` 下的 Node.js 脚本生成：

| 脚本                                   | 生成内容                                                            |
| -------------------------------------- | ------------------------------------------------------------------- |
| `generate-four-pillars-fixtures.mjs`   | `tests/fixtures/four_pillars.json`（四柱兼容性测试用例）            |

四柱 fixture 使用 [`lunar-lite`](https://github.com/SylarLong/lunar-lite)。转换测试使用经 [`tyme4rs`](https://github.com/6tail/tyme4rs) 校验的稳定字面量。

**运行时用户无需安装 Node.js、`lunar-typescript` 或 `tyme4rs`。**

重新生成方法：

```sh
cd tools/lunar-lite-reference
npm install
npm run generate-four-pillars-fixtures
```

## 与 tyme4rs 的兼容性

转换结果目标是在支持范围内匹配 tyme4rs 的日历政策。内部天文内核的部分内容改编自 MIT 许可的 `6tail/tyme4rs`，详见 [`THIRD_PARTY_LICENSES.md`](THIRD_PARTY_LICENSES.md)。这是独立的改编实现，并不暗示获得 `6tail` 或 `tyme4rs` 项目的任何认可或关联。

### 精确立春时刻

`lunar-lite` 将立春的精确天文时刻作为稳定的公开原语暴露：

```rust
use lunar_lite::{li_chun_datetime, LunarError};

let dt = li_chun_datetime(2000).unwrap();
assert_eq!((dt.date.year, dt.date.month, dt.date.day), (2000, 2, 4));
assert_eq!((dt.hour, dt.minute, dt.second), (20, 40, 24));

// 超出支持范围的年份返回 LunarError::SolarTermOutOfRange。
assert_eq!(
    li_chun_datetime(0).unwrap_err(),
    LunarError::SolarTermOutOfRange { year: 0 },
);
```

此接口**不会**改变四柱 API 中 [`YearDivide::Exact`] 的语义——后者仍按**日期**粒度处理，保持向后兼容。`li_chun_datetime` 是为需要秒级立春精度的下游 crate 提供的独立公开原语。

## 非目标

- **完整节气 API** — 仅暴露 `li_chun_datetime`；其余节气边界支撑四柱月柱计算，但不单独暴露。
- **真太阳时修正** — 不根据经度应用时区偏移；四柱时间由 `time_index` 合成。
- **紫微斗数排盘** — 不在支持范围内。
- **运行时 JavaScript 依赖** — 库在运行时为纯 Rust 实现。

## 发布流程

版本发布由 [release-plz](https://release-plz.dev/) 管理。

变更合并到 `main` 分支后，release-plz 会自动创建或更新 Release PR。
请在该 PR 中确认版本号升级和变更日志，合并后工作流会将 crate 发布到 crates.io 并创建 GitHub release/tag。

所需仓库密钥：

- `CARGO_REGISTRY_TOKEN`：具有发布 `lunar-lite` 权限的 crates.io API token。

## 许可证

MIT — 详见 [LICENSE](LICENSE)。
