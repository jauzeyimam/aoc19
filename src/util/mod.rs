use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_comma_delim<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    let mut result: Vec<u64> = Vec::new();
    for lines in br.lines() {
        for value in lines.unwrap().split(',') {
            let value: u64 = value.parse::<u64>().unwrap();
            result.push(value);
        }
    }

    Ok(result)
}
