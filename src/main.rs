use std::{fs::File, os::windows::io::AsRawHandle as _, path::PathBuf};

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use clap::Parser;
use color_eyre::eyre;
use windows::Win32::{
    Foundation::{FILETIME, HANDLE},
    Storage::FileSystem::SetFileTime,
};

#[derive(Parser, Debug)]
struct CliArgs {
    path: PathBuf,
    datetime: String,
}

/// Seconds between 1601-01-01 and 1970-01-01
const EPOCH_DIFFERENCE: i64 = 11_644_473_600;
const NANOSECONDS_PER_SECOND: i64 = 10_000_000;

fn set_filetime(handle: HANDLE, datetime: DateTime<Utc>) {
    let filetime: FILETIME = datetime_to_filetime(datetime);
    // SAFETY:
    // it's a windows API, god knows what it's doing
    unsafe { SetFileTime(handle, Some(&filetime), None, Some(&filetime)) }.unwrap();
}

fn parse_date(date_string: &str) -> eyre::Result<DateTime<Utc>> {
    let naive_date: NaiveDate = match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
        Err(error) => Err(eyre::eyre!("failure to parse {} - {}", date_string, error)),
        Ok(ok) => Ok(ok),
    }?;
    let datetime: DateTime<Utc> = NaiveDateTime::new(naive_date, NaiveTime::MIN).and_utc();
    Ok(datetime)
}

fn datetime_to_filetime(datetime: DateTime<Utc>) -> FILETIME {
    let timestamp: i64 = datetime.timestamp();

    let filetime_value: i64 = (timestamp + EPOCH_DIFFERENCE) * NANOSECONDS_PER_SECOND;

    let low_datetime: u32 = u32::try_from(filetime_value & 0xFFFF_FFFF).unwrap();
    let high_datetime: u32 = u32::try_from(filetime_value >> 32_i32).unwrap();
    FILETIME {
        dwLowDateTime: low_datetime,
        dwHighDateTime: high_datetime,
    }
}

fn main() -> eyre::Result<()> {
    let args: CliArgs = CliArgs::parse();

    let datetime: DateTime<Utc> = parse_date(&args.datetime)?;

    let file: File = File::open(args.path)?;
    let handle: HANDLE = HANDLE(file.as_raw_handle());

    set_filetime(handle, datetime);

    Ok(())
}
