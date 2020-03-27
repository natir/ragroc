use crate::utils;

#[derive(Debug)]
pub struct FormData {
    pub input: Vec<u8>,
    pub format: utils::Format,
}

impl FormData {
    pub fn from_document<'a>(doc: &web_sys::Document) -> Result<Self, utils::FormError> {
	let input = utils::get_text_area("rc_in_text", doc)?.value();

	if input.len() == 0 {
	    Ok(Self {
		input: input.into_bytes(),
		format: utils::Format::None,
	    })
	} else if input.starts_with(">") {
	    Ok(Self {
		input: input.into_bytes(),
		format: utils::Format::Fasta,
	    })
	} else if input.starts_with("@") {
	    Ok(Self {
		input: input.into_bytes(),
		format: utils::Format::Fastq,
	    })
	} else {
	    Ok(Self {
		input: input.into_bytes(),
		format: utils::Format::None,
	    })
	}
    }
}

#[derive(Debug)]
pub struct Revcomp {
    comp: &'static [u8],
    param: FormData,
}

static mut COMP: &'static mut [u8; 255] = &mut [0; 255];

impl Revcomp {
    pub fn setup() {
	let comp: &'static mut [u8] = unsafe { COMP };

	for i in 0..255 {
	    comp[i] = i as u8;
	}
	
	comp[b'A' as usize] = b'T';
	comp[b'C' as usize] = b'G';
	comp[b'T' as usize] = b'A';
	comp[b'G' as usize] = b'C';
	comp[b'a' as usize] = b't';
	comp[b'c' as usize] = b'g';
	comp[b't' as usize] = b'a';
	comp[b'g' as usize] = b'c';
    }

    pub fn from_document(doc: &web_sys::Document) -> Result<Self, utils::FormError> {
	
	Ok(
	    Revcomp {
		comp: unsafe { COMP },
		param: FormData::from_document(doc)?,
	    }
	)
    }    

    pub fn produce_output(&self) -> String {
	match self.param.format {
	    utils::Format::None => {
		unsafe { String::from_utf8_unchecked(self.revcomp_seq(self.param.input.clone())) }
	    },
	    utils::Format::Fasta => {
		unsafe { String::from_utf8_unchecked(self.manage_fasta()) }
	    },
	    utils::Format::Fastq => {
		unsafe { String::from_utf8_unchecked(self.manage_fastq()) }
	    }
	}
    }

    fn manage_fasta(&self) -> Vec<u8> {
	let mut output = Vec::new();
	
	let mut reader = bio::io::fasta::Reader::new(&self.param.input[..]).records();
	let mut writer = bio::io::fasta::Writer::new(&mut output);

	while let Some(Ok(record)) = reader.next() {
	    writer.write(
		record.id(),
		record.desc(),
		&self.revcomp_seq(record.seq().to_vec())
	    ).expect("Error durring write of reverse complement fasta");
	}

	drop(writer);
	
	output
    }
    
    fn manage_fastq(&self) -> Vec<u8> {
	let mut output = Vec::new();
	
	let mut reader = bio::io::fastq::Reader::new(&self.param.input[..]).records();
	let mut writer = bio::io::fastq::Writer::new(&mut output);
	
	while let Some(Ok(record)) = reader.next() {
	    writer.write(
		record.id(),
		record.desc(),
		&self.revcomp_seq(record.seq().to_vec()),
		&record.qual().iter().rev().cloned().collect::<Vec<u8>>()
	    ).expect("Error durring write of reverse complement fasta");
	}

	drop(writer);
	
	output
    }

    fn revcomp_seq(&self, mut input: Vec<u8>) -> Vec<u8> {
	if input.len() == 0 {
	    return input;
	}

	if input.len() == 1 {
	    input[0] = self.comp[input[0] as usize];
	    return input;
	}
	
	let mut first = 0;
	let mut last = input.len() - 1;
	
	while first < last {
	    input[first] = self.comp[input[first] as usize];
	    input[last] = self.comp[input[last] as usize];

	    input.swap(first, last);

	    first += 1;
	    last -= 1;
	}

	if input.len() % 2 == 1 {
	    let middle = input.len() / 2;
	    input[middle] = self.comp[input[middle] as usize];
	}
	
	input
    }
}
