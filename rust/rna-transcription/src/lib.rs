#[derive(Debug)]
pub struct RibonucleicAcid {
  sequence: String
}

impl RibonucleicAcid {
  pub fn new(sequence: &str) -> RibonucleicAcid {
    RibonucleicAcid { sequence: sequence.to_string() }
  }
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
  sequence: String
}

impl DeoxyribonucleicAcid {
  pub fn new(sequence: &str) -> DeoxyribonucleicAcid {
    DeoxyribonucleicAcid { sequence: sequence.to_string() }
  }
  pub fn to_rna(&self) -> RibonucleicAcid {
    let mut new_sequence = String::new();
    for c in self.sequence.chars() {
      match c {
        'A' => new_sequence.push('U'),
        'T' => new_sequence.push('A'),
        'C' => new_sequence.push('G'),
        'G' => new_sequence.push('C'),
        _ => new_sequence.push(' ')
      }
    }

    RibonucleicAcid { sequence: new_sequence }
  }
}

impl PartialEq for RibonucleicAcid {
  fn eq(&self, other: &Self) -> bool {
    return self.sequence == other.sequence;
  }
}

impl PartialEq for DeoxyribonucleicAcid {
  fn eq(&self, other: &Self) -> bool {
    return self.sequence == other.sequence;
  }
}