// This file is unused.

pub mod log {
    extern crate time;

    use std::path::Path;
    use std::fs::OpenOptions;
    use	std::error::Error;
    use std::io::Write;
    use std::fs;

    pub struct Log<'a> {
        output_addr: &'a Path, // log file address
    }

    impl<'a> Log<'a> {
        pub fn new(addr: &'a str) -> Log<'a> {
            let log = Log { output_addr: Path::new(addr) };
            log.clear(); // clear the old log.
            log
        }

        // shutdown if any error occurs
        pub fn print(&self, msg: &'a str) {
            // add time log
            let msg = format!("[{}] {}\n", time::now().ctime(), msg);

            let mut fout = match OpenOptions::new().append(true).create(true)
                                    .open(&self.output_addr) {
                Err(why) => panic!("could not create {}: {}",
                                   self.output_addr.display(),
                                   why.description()),
                Ok(file) => file,
            };

            if let Err(why) = fout.write(msg.as_bytes()) {
                panic!("could not write to {}: {}",
                       self.output_addr.display(),
                       why.description());
            }
        }
        
        // remove the log file
        fn clear(&self) {
            fs::remove_file(self.output_addr).expect(
                &format!("can not remove file {}.",
                        self.output_addr.display())
            );
        }
    }
}