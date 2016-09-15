extern crate rand;

use std::net::Ipv4Addr;
use rand::Rng;

const MAX_OCTET_OFFSET: u16 = std::u16::MAX - 255;

/// The `Scruffy` type
pub struct Scruffy {
    octets_offsets: [u16; 4],
    current_ip: Ipv4Addr,
}

impl Scruffy {
    /// Constructs a new `Scruffy` with the given `octets_offsets`.
    pub fn new(octets_offsets: [u16; 4]) -> Scruffy {
        Scruffy {
            octets_offsets: octets_offsets,
            current_ip: Ipv4Addr::new(0, 0, 0, 0),
        }
    }

    /// Constructs a new `Scruffy` with random `octets_offsets`.
    pub fn new_rand() -> Scruffy {
        Scruffy {
            octets_offsets: [Scruffy::get_rand_offset(), Scruffy::get_rand_offset(), Scruffy::get_rand_offset(), Scruffy::get_rand_offset()],
            current_ip: Ipv4Addr::new(0, 0, 0, 0),
        }
    }

    /// Generates a single random octat offset
    ///
    /// A valid offset has to be within `std::u16::MAX` - 255.
    /// A valid offset can not be a factor of 255.
    fn get_rand_offset() -> u16 {
        let mut rand_offset: u16;

        loop {
            rand_offset = rand::thread_rng().gen_range(1, MAX_OCTET_OFFSET);

            // Check if the offset is not a factor of 255
            if 255 % rand_offset != 0 {
                break;
            }
        }

        rand_offset
    }

    /// Getter for octets_offsets.
    ///
    /// Retrieve eventually randomly generated `octets_offsets`.
    pub fn get_octets_offsets(&self) -> [u16; 4] {
        self.octets_offsets
    }
}

impl Iterator for Scruffy {
    type Item = Ipv4Addr;

    /// Impl `next`
    ///
    /// Creates a new IP Address based on the previoius.
    /// For each part of the IP Address a specific offset gets applied.
    /// The offset addition goes from right to left. If the newly created offset completes the
    /// circle of 255 the next part will be modified as well.
    fn next(&mut self) -> Option<Ipv4Addr> {
        let mut octets = self.current_ip.octets();

        for i in (0..4).rev() {
            let new_octet: u16 = octets[i] as u16 + self.octets_offsets[i];
            octets[i] = (new_octet % 255) as u8;

            if new_octet <= 255 {
                break;
            }
        }

        self.current_ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);

        Some(self.current_ip)
    }
}

#[cfg(test)]
mod tests {
    use Scruffy;
    use std::net::Ipv4Addr;

    #[test]
    fn is_deterministic() {
        let rand_scruffy = Scruffy::new_rand();
        let octets_offsets = rand_scruffy.get_octets_offsets();
        let rebuilt_scruffy = Scruffy::new(octets_offsets);

        let a_set = rand_scruffy.take(10000).collect::<Vec<Ipv4Addr>>();
        let b_set = rebuilt_scruffy.take(10000).collect::<Vec<Ipv4Addr>>();

        assert_eq!(a_set, b_set);
    }
}
