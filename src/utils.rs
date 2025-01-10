use anyhow::{Error, Ok, Result};

use std::{
    fs::File,
    io::{self,prelude::*,BufRead,BufReader},
    path::Path,
};

use log::*;

use crate::error::Xerror;

const MAGIC_MAX_LEN: usize = 6;
const BUFF_SIZE: usize = 1024 * 1024;
const GZ_MAGIC: [u8; 3] = [0x1f, 0x8b, 0x08];

fn magic_num<P>(file_name: P) -> Result<[u8; MAGIC_MAX_LEN], Error>
where
    P: AsRef<Path> + Copy,
{
    let mut buffer: [u8; MAGIC_MAX_LEN] = [0; MAGIC_MAX_LEN];
    let mut fp = File::open(file_name).map_err(Xerror::IoError)?;

    let _ = fp.read(&mut buffer)?;
    Ok(buffer)
}

fn is_gzipped<P>(file_name: P) -> Result<bool>
where
    P: AsRef<Path> + Copy,
{
    let buffer = magic_num(file_name)?;
    let gz_or_not =
        buffer[0] == GZ_MAGIC[0] && buffer[1] == GZ_MAGIC[1] && buffer[2] == GZ_MAGIC[2];
    Ok(gz_or_not
        || file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "gz"))
}

pub fn file_reader<P>(file_in: Option<P>) -> Result<Box<dyn BufRead>>
where
    P: AsRef<Path> + Copy,
{
    if let Some(file_name) = file_in {
        let gz_flag = is_gzipped(file_name)?;
        let fp = File::open(file_name).map_err(Xerror::IoError)?;

        if gz_flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                flate2::read::MultiGzDecoder::new(fp),
            )))
        } else {
            Ok(Box::new(BufReader::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        if atty::is(atty::Stream::Stdin) {
            error!("{}", Xerror::StdinNotDetected);
            // return Err(anyhow::anyhow!("{}", Xerror::StdinNotDetected));
            std::process::exit(1);
        }

        /* 
        let stdout = io::stdout();
        let mut handle = stdout.lock();
    
        if let Err(e) = writeln!(handle, "Stdin Error") {
            if e.kind() == io::ErrorKind::BrokenPipe {
                // 忽略 BrokenPipe 错误
                std::process::exit(0);
            } else {
                eprintln!("Unexpected error: {}", e);
            }
        }
        */

        let fp = BufReader::new(io::stdin());
        Ok(Box::new(fp))
    }
}
