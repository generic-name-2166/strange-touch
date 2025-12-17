use std::{
    fs::{File, OpenOptions},
    os::windows::io::AsRawHandle as _,
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use clap::Parser;
use eros::Context;
use windows::Win32::{
    Foundation::{FILETIME, HANDLE},
    Storage::FileSystem::SetFileTime,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    path: PathBuf,
    #[arg(short, long)]
    timestamp: Option<i64>,
    #[arg(short, long)]
    datetime: Option<String>,
}

/// Seconds between 1601-01-01 and 1970-01-01
const EPOCH_DIFFERENCE: i64 = 11_644_473_600;
const NANOSECONDS_PER_SECOND: i64 = 10_000_000;

fn set_filetime(handle: HANDLE, datetime: DateTime<Utc>) {
    let filetime: FILETIME = datetime_to_filetime(datetime);
    // SAFETY:
    // it's a windows API, god knows what it's doing
    unsafe {
        SetFileTime(
            handle,
            Some(&raw const filetime),
            None,
            Some(&raw const filetime),
        )
    }
    .unwrap();
}

fn parse_date(date_string: &str) -> eros::Result<DateTime<Utc>> {
    let datetime: DateTime<Utc> = DateTime::parse_from_rfc3339(date_string)
        .map_err(|err: chrono::ParseError| eros::traced!("{}", err))?
        .to_utc();
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

fn main() -> eros::Result<()> {
    let args: CliArgs = CliArgs::parse();

    let datetime = match (args.timestamp, args.datetime) {
        (Some(secs), _) => {
            DateTime::from_timestamp(secs, 0).context("provided timestamp is invalid")
        }
        (None, Some(date_string)) => parse_date(&date_string),
        _ => Err(eros::traced!("provide either --timestamp or --datetime")),
    }?;

    let file: File = OpenOptions::new().write(true).open(args.path)?;
    let handle: HANDLE = HANDLE(file.as_raw_handle());

    set_filetime(handle, datetime);

    Ok(())
}
