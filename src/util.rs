use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_comma_delim<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut result: Vec<i64> = Vec::new();
    for lines in br.lines() {
        for value in lines.unwrap().split(',') {
            let value: i64 = value.parse::<i64>().unwrap();
            result.push(value);
        }
    }

    Ok(result)
}

pub fn read_comma_delim_str<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let br = BufReader::new(io);
    let mut result: Vec<Vec<String>> = Vec::new();
    for lines in br.lines() {
        let mut line_vec: Vec<String> = Vec::new();
        for value in lines.unwrap().split(',') {
            line_vec.push(value.to_string());
        }
        result.push(line_vec);
    }

    Ok(result)
}
