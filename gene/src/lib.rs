
/// Change sequence of gene's to binary.
pub fn sequence_to_bin(sequence: &str) -> u32 {
    
    let binary : Vec<u32> = sequence.chars().map(| x | {
        let b: u32 = gene_to_bin(x) as u32;
        b
    }).collect();
    let mut bin_sequence: u32 = 0;
    for i in binary.iter() {
        if bin_sequence == 0 {
            bin_sequence = *i;
        } else {
            bin_sequence = bin_sequence << 2;
            bin_sequence = bin_sequence & i;
        }
    }
    bin_sequence
}

pub fn sequence_from_bin(bin_sequence: u32) -> String {
    let mut sequence = "".to_string();
    for i in 0..16 {
        let gene_bits = ((bin_sequence >> (2 * i)) & 0b11) as u8;
        let gene = gene_from_bin(gene_bits);
        sequence.push(gene);
    }
    sequence
}

pub fn gene_to_bin(gene: char) -> u8 {
    match gene.to_ascii_lowercase() {
        'a' => 0b00,
        't' => 0b11,
        'c' => 0b01,
        'g' => 0b10,
        _ => panic!("No A, T, C, G DNA Sequence found. '{gene}'"),
    }
}

pub fn gene_from_bin(gene_bin: u8) -> char {
    match gene_bin {
        0b00 => 'a',
        0b11 => 't',
        0b01 => 'c',
        0b10 => 'g',
        _ => panic!("No A, T, C, G DNA Sequence found. '{gene_bin}'"),
    }
}

pub fn gene_complement(gene: char) -> char {
    match gene.to_ascii_lowercase() {
        'a' => 't',
        't' => 'a',
        'c' => 'g',
        'g' => 'c',
        _ => panic!("No A, T, C, G DNA Sequence found. '{gene}'"),
    }
}

pub fn sequence_complement(gene: &str) -> String {
    let gene_chars = gene.chars();
    gene_chars.map(| x | gene_complement(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_to_bin() {
        assert_eq!(0b00, gene_to_bin('a'));
        assert_eq!(0b00, gene_to_bin('A'));
        
        assert_eq!(0b11, gene_to_bin('t'));
        assert_eq!(0b11, gene_to_bin('T'));

        assert_eq!(0b01, gene_to_bin('c'));
        assert_eq!(0b01, gene_to_bin('C'));

        assert_eq!(0b10, gene_to_bin('g'));
        assert_eq!(0b10, gene_to_bin('G'));
    }

    #[test]
    fn test_gene_from_bin() {
        assert_eq!('a', gene_from_bin(0b00));
        assert_eq!('t', gene_from_bin(0b11));
        assert_eq!('c', gene_from_bin(0b01));
        assert_eq!('g', gene_from_bin(0b10));
    }

    #[test]
    fn test_sequence_to_bin() {
        let seq = "ACTTG";
        let seq_bin_expected = 0b0001111110u32;
        assert_eq!(seq_bin_expected, sequence_to_bin(seq));
    }

    #[test]
    fn test_sequence_from_bin() {
        let seq_bin = 0b0001111110u32;
        let seq_expected = "ACTTG";
        assert_eq!(seq_expected, sequence_from_bin(seq_bin));
    }
}