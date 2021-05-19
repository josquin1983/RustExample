pub mod fromfile {

    use std::io::BufReader;

    use std::io::prelude::*;

    use std::fs::File;

    use std::str::FromStr;

    use std::error::Error;

    use std::path::Path;

    type DataResult<T> = std::result::Result<Vec<T>, Box<dyn Error>>;

    pub fn read_from_file<P, T>(file_name: P) -> DataResult<T>
    where P: AsRef<Path>,
         T: FromStr,
         <T as FromStr>::Err: 'static + std::error::Error,
         
    {
        let mut vc = Vec::with_capacity(20);

        let fd = File::open(file_name)?;

        let buffer = BufReader::new(fd);

        for line_result in buffer.lines() {
            let line = match line_result {
                Ok(line) => line,

                Err(e) => return Err(Box::new(e)),
            };

            let data:T = line.parse()?;

            vc.push(data);
        }

        Ok(vc)
    }

    //unit test

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]

                fn has_been_read() {
            let vc = vec![8.0, 8.0, 9., 8., 8.0, 8.0];

            let data: Vec<f32> = read_from_file("test_data.txt").unwrap_or(Vec::with_capacity(1));
                assert_eq!(data, vc);
            }
        }
    }
