// S-Box and its inverse (RS-Box) mappings for substitution
const S_BOX: [u8; 16] = [0x9, 0x4, 0xa, 0xb, 0xd, 0x1, 0x8, 0x5, 0x6, 0x2, 0x0, 0x3, 0xc, 0xe, 0xf, 0x7];
const RS_BOX: [u8; 16] = [0xa, 0x5, 0x9, 0xb, 0x1, 0x7, 0x8, 0xf, 0x6, 0x0, 0x2, 0x3, 0xc, 0x4, 0xd, 0xe];

// P-Box and its inverse (RP-Box) mappings for permutation
const P_BOX: [u8; 8] = [0, 4, 1, 5, 2, 6, 3, 7];
const RP_BOX: [u8; 8] = [0, 2, 4, 6, 1, 3, 5, 7];

// Substitutes each byte in the input vector with a corresponding value from S-Box
fn substitution(input: Vec<u8>) -> Vec<u8> {
    let mut output = vec![0; input.len()];
    for (i, &byte) in input.iter().enumerate() {
        // extract the high nibble (4 bits)
        let t1 = (byte & 0xf0) >> 4;
        // extract the low nibble (4 bits)
        let t2 = byte & 0x0f;
        // substitute the byte using S-Box
        output[i] = (S_BOX[t1 as usize] << 4) | S_BOX[t2 as usize]

    }
    output
}

// Substitutes each byte in the input vector with a corresponding value from RS-Box (inverse S-Box)
fn substitution_inv(input: Vec<u8>) -> Vec<u8> {
    let mut output = vec![0; input.len()];
    for (i, &byte) in input.iter().enumerate() {
        // extract the high nibble (4 bits)
        let t1 = (byte & 0xf0) >> 4;
        // extract the low nibble (4 bits)
        let t2 = byte & 0x0f;
        // substitute the byte using RS-Box
        output[i] = (RS_BOX[t1 as usize] << 4) | RS_BOX[t2 as usize]
    }
    output
}

// Permutes the bits of each byte in the input vector using P-Box
fn permutation(input: Vec<u8>) -> Vec<u8> {
    let mut output = vec![0; input.len()];
    for (i, &byte) in input.iter().enumerate() {
        let mut temp = 0;
        for j in 0..8 {
            // extract each bit of the byte
            let bit = (byte >> j) & 0x01;
            // permute the bit using P-Box
            temp |= bit << P_BOX[j];
        }
        output[i] = temp;
    }
    output
}

// Permutes the bits of each byte in the input vector using RP-Box (inverse P-Box)
fn permutation_inv(input: Vec<u8>) -> Vec<u8> {
    let mut output = vec![0; input.len()];
    for (i, &byte) in input.iter().enumerate() {
        let mut temp = 0;
        for j in 0..8 {
            // extract each bit of the byte
            let bit = (byte >> j) & 0x01;
            // permute the bit using RP-Box
            temp |= bit << RP_BOX[j];
        }
        output[i] = temp;
    }
    output
}

fn main() {
    let input = "Hello world!";
    // convert the input string to a vector of bytes
    let input_bytes  = input.as_bytes().to_vec();
    println!("Input: {:?}", input_bytes);

    // perform substitution on the input bytes
    let subst = substitution(input_bytes.clone());
    // perform permutation on the substituted bytes
    let perm = permutation(subst.clone());
    println!("Cipher: {:?}", perm);

    // perform inverse permutation on the permuted bytes
    let perm_inv = permutation_inv(perm.clone());
    // perform inverse substitution on the bytes
    let subst_inv = substitution_inv(perm_inv.clone());
    println!("Inversed: {:?}", subst_inv);
}