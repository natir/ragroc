use crate::utils;

use rand::distributions::Distribution;

#[derive(Debug)]
pub struct FormData {
    pub nb_seq: usize,
    pub length: usize,
    pub format: utils::Format,
    pub weight: [f64; 4],
}

impl FormData {
    pub fn from_document<'a>(doc: &web_sys::Document) -> Result<Self, utils::FormError> {
	let nb_seq = utils::get_value::<usize>("nb_sequences", &doc)?;
	let length = utils::get_value::<usize>("length", &doc)?;

	let format = utils::get_radio_value("format", doc)?;
	
	let weight = [
	    utils::get_value::<f64>("a_weight", &doc)?,
	    utils::get_value::<f64>("c_weight", &doc)?,
	    utils::get_value::<f64>("t_weight", &doc)?,
	    utils::get_value::<f64>("g_weight", &doc)?,
	];

	Ok(Self {
	    nb_seq,
	    length,
	    format,
	    weight,
	})
    }
}

static mut DNA: &'static mut [u8; 4] = &mut [0; 4];
static mut QUAL: &'static mut [u8; 94] = &mut [0; 94];

#[derive(Debug)]
pub struct Generate {
    dna: &'static [u8],
    qual: &'static [u8],
    param: FormData,
}

impl Generate {
    pub fn setup() {
	let dna: &'static mut [u8] = unsafe { DNA };

	dna[0] = b'A';
	dna[1] = b'C';
	dna[2] = b'T';
	dna[3] = b'G';

	let qual: &'static mut [u8]= unsafe { QUAL };
	for i in 0..94 {
	    qual[i] = i as u8;
	}
    }
    
    pub fn from_document(doc: &web_sys::Document) -> Result<Self, utils::FormError> {
	Ok(
	    Generate {
		dna: unsafe { DNA },
		qual: unsafe { QUAL },
		param: FormData::from_document(doc)?,
	    }
	)
    }

    pub fn produce_output(&self) -> String {
	match self.param.format {
	    utils::Format::None => self.create_raw(),
	    utils::Format::Fasta => self.create_fasta(),
	    utils::Format::Fastq => self.create_fastq(),
	}
    }

    fn create_seq(&self) -> Vec<u8> {
	let mut rng = rand::thread_rng();
	let dist = rand::distributions::WeightedIndex::new(&self.param.weight).unwrap(); // value of weight can't be negative

	let mut seq = Vec::with_capacity(self.param.length);
	
	for _ in 0..self.param.length {
	    seq.push(self.dna[dist.sample(&mut rng)]);
	}

	seq
    }
    
    fn create_qual(&self) -> Vec<u8> {
	let mut rng = rand::thread_rng();
	let dist = rand::distributions::Uniform::new(0, 94);
	
	let mut qual = Vec::with_capacity(self.param.length);

	for _ in 0..self.param.length {
	    qual.push(self.qual[dist.sample(&mut rng)] + 33);
	}

	qual
    }
    
    fn create_raw(&self) -> String {
	let mut seq = Vec::with_capacity(self.param.length * self.param.nb_seq + self.param.nb_seq);

	for _ in 0..self.param.nb_seq {
	    seq.append(&mut self.create_seq());
	    seq.push(b'\n');
	}

	unsafe { String::from_utf8_unchecked(seq) }
    }

    fn create_fasta(&self) -> String {
	let mut output = Vec::new();
	
	let mut writer = bio::io::fasta::Writer::new(&mut output);

	for i in 0..self.param.nb_seq {
	    writer.write(
		"random_seq",
		Some(&i.to_string()),
		&self.create_seq(),
	    ).expect("Error durring write of random sequences in fasta");
	}
	drop(writer);
	
	unsafe { String::from_utf8_unchecked(output) }
    }

    fn create_fastq(&self) -> String {
	let mut output = Vec::new();
	
	let mut writer = bio::io::fastq::Writer::new(&mut output);

	for i in 0..self.param.nb_seq {
	    writer.write(
		"random_seq",
		Some(&i.to_string()),
		&self.create_seq(),
		&self.create_qual(),
	    ).expect("Error durring write of random sequences in fastq");
	}
	drop(writer);
	
	unsafe { String::from_utf8_unchecked(output) }
    }
}
