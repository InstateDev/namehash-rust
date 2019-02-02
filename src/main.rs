use tiny_keccak::Keccak;



fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];
    if name.is_empty() {
        return node;
    }
    let mut labels: Vec<&str> = name.split(".").collect();
    labels.reverse();
    for label in labels.iter() {
        let mut labelhash = [0u8; 32];
        Keccak::keccak256(label.as_bytes(), &mut labelhash);
        node.append(&mut labelhash.to_vec());
        labelhash = [0u8; 32];
        Keccak::keccak256(node.as_slice(), &mut labelhash);
        node = labelhash.to_vec();
    }
    node
}

fn main() {
    let mut eth_example = vec![0u8; 32];
    println!("{:x?}", namehash("foo.eth"));
}


#[cfg(test)]
mod test {
    use super::namehash;

    #[test]
    fn test_namehash() {
        let results = vec![
            ("eth", vec![0x93, 0xcd, 0xeb, 0x70, 0x8b, 0x75, 0x45, 0xdc, 0x66, 0x8e, 0xb9, 0x28, 0x1, 0x76, 0x16, 0x9d,0x1c, 0x33, 0xcf, 0xd8, 0xed, 0x6f, 0x4, 0x69, 0x0ca, 0xb, 0xcc, 0x88, 0xa9, 0x3f, 0xc4, 0xae]),
            ("eth", 
            b"93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"),
            ("foo.eth", b"de9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f"),
        ];
        for (name, hash) in results {
            let namehash = namehash(name);
            let h = format!("{:x?}", namehash);
            assert_eq!(namehash, hash);
        }
    }
}

