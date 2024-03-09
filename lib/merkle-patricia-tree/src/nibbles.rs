// 1 nibbles is rapresented as an u8 in wich the 4 most significant bits are set
// to 0
pub fn bytes_to_nibbles(bytes: &[u8]) -> Vec<u8> {
    let mut nibbles = vec![0; bytes.len() * 2];

    let mut j = 0;
    for i in 0..bytes.len() {
        nibbles[j] = (bytes[i] >> 4) & 0x0f;
        nibbles[j + 1] = bytes[i] & 0x0f;
        j += 2;
    }

    nibbles
}

pub fn nibbles_to_bytes(nibbles: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0; nibbles.len() / 2];

    for i in 0..bytes.len() {
        let q = i * 2;
        let k = q + 1;
        bytes[i] = (nibbles[q] << 4) + nibbles[k];
    }

    bytes
}

pub fn add_hex_prefix(nibbles: &[u8], with_terminator: bool) -> Vec<u8> {
    let mut modified_nibbles;

    if nibbles.len() % 2 == 0 {
        let mut head = [0, 0].to_vec();
        head.extend_from_slice(nibbles);
        modified_nibbles = head;
    } else {
        let mut head = [1].to_vec();
        head.extend_from_slice(nibbles);
        modified_nibbles = head;
    }

    if with_terminator == true {
        modified_nibbles[0] += 2;
    }

    modified_nibbles
}

pub fn calculate_prefix_length(nibbles: &[u8], nibbles_target: &[u8]) -> usize {
    let mut prefix = 0;

    let min_length = match nibbles_target.len() {
        len if len > nibbles.len() => nibbles.len(),
        _ => nibbles_target.len(),
    };

    for i in 0..min_length {
        if nibbles[i] != nibbles_target[i] {
            break;
        }

        prefix += 1;
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_bytes_into_nibbles() {
        let bytes = [104, 101, 108, 108, 111]; // "hello"
        let nibbles = bytes_to_nibbles(&bytes);
        assert!(nibbles == vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15]);
    }

    #[test]
    fn should_convert_nibbles_into_bytes() {
        let nibbles = vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15];
        let bytes = nibbles_to_bytes(&nibbles);
        assert!(bytes == vec![104, 101, 108, 108, 111]);
    }

    #[test]
    fn should_add_hex_prefix_with_odd_nibbles_and_with_terminator() {
        // leaf node and odd number of nibbles
        let nibbles = vec![6, 8, 6, 5, 6, 12, 6, 12, 6];
        let prefixed_nibbles = add_hex_prefix(&nibbles, true);
        assert!(prefixed_nibbles == vec![3, 6, 8, 6, 5, 6, 12, 6, 12, 6]);
    }

    #[test]
    fn should_add_hex_prefix_with_even_nibbles_and_with_terminator() {
        // leaf node and even number of nibbles
        let nibbles = vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15];
        let prefixed_nibbles = add_hex_prefix(&nibbles, true);
        assert!(prefixed_nibbles == vec![2, 0, 6, 8, 6, 5, 6, 12, 6, 12, 6, 15]);
    }

    #[test]
    fn should_add_hex_prefix_with_odd_nibbles_and_without_terminator() {
        // extension node and odd number of nibbles
        let nibbles = vec![6, 8, 6, 5, 6, 12, 6, 12, 6];
        let prefixed_nibbles = add_hex_prefix(&nibbles, false);
        assert!(prefixed_nibbles == vec![1, 6, 8, 6, 5, 6, 12, 6, 12, 6]);
    }

    #[test]
    fn should_add_hex_prefix_with_even_nibbles_and_without_terminator() {
        // extension node and even number of nibbles
        let nibbles = vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15];
        let prefixed_nibbles = add_hex_prefix(&nibbles, false);
        assert!(prefixed_nibbles == vec![0, 0, 6, 8, 6, 5, 6, 12, 6, 12, 6, 15]);
    }

    #[test]
    fn should_calculate_the_prefix_length_between_2_array_of_nibbles() {
        let nibbles1 = vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15];
        let nibbles2 = vec![6, 8, 6, 5, 6, 12, 6, 12, 6, 15, 5, 3, 6, 8, 9];
        let prefix = calculate_prefix_length(&nibbles1, &nibbles2);
        assert!(prefix == 10);
    }
}
