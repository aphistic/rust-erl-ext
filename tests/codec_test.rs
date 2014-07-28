extern crate erl_ext;
extern crate glob;

use erl_ext::{Encoder,Decoder};
use std::io;
use glob::glob;

#[test]
fn main() {
    // generate tests/data/*.bin
    match io::Command::new("escript").arg("tests/term_gen.erl").spawn() {
        Ok(_) => (),
        Err(ioerr) => {
            (writeln!(
                io::stderr(),
                "{}:{} [warn] Failed to launch escript - '{}'. Is Erlang installed?",
                file!(), line!(), ioerr)).unwrap();
            return
        }
    };
    // run decode-encode cycle and compare source and resulting binaries
    for path in glob("tests/data/*.bin") {
        let mut in_f = io::File::open(&path).unwrap();
        let src = in_f.read_to_end().unwrap();

        let mut rdr = io::MemReader::new(src);
        let mut wrtr = io::MemWriter::new();
        {
            let mut decoder = Decoder::new(&mut rdr);
            assert!(true == decoder.read_prelude().unwrap(),
                    "{}: bad prelude", path.display());
            let term = decoder.decode_term().unwrap();

            let mut encoder = Encoder::new(&mut wrtr, false, false, true);
            encoder.write_prelude().unwrap();
            encoder.encode_term(term).unwrap();
        }
        assert!(wrtr.get_ref() == rdr.get_ref(),
                "{}: Before and After isn't equal", path.display());
    }
}