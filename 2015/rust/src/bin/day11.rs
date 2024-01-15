use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let password: Password = input.trim().parse()?;

    println!("Part 1: {}", part1(&password));
    println!("Part 2: {}", part2(&password));

    Ok(())
}

fn part1(password: &Password) -> Password {
    password.next_valid()
}

fn part2(password: &Password) -> Password {
    password.next_valid().next_valid()
}

#[derive(Debug, Copy, Clone)]
struct Password(u64);

impl Password {
    fn next_valid(&self) -> Password {
        let mut password = Password(self.0 + 1);
        while !password.is_valid() {
            password = Password(password.0 + 1);
        }
        password
    }

    fn is_valid(&self) -> bool {
        let bytes = self.bytes();
        if bytes.iter().any(|b| matches!(b, b'i' | b'o' | b'l')) {
            return false;
        }
        if !bytes
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
        {
            return false;
        }
        let mut first_pair = None;
        for (i, pair) in bytes.windows(2).enumerate() {
            if pair[0] == pair[1] {
                let j = *first_pair.get_or_insert(i);
                if j + 1 < i {
                    return true;
                }
            }
        }
        false
    }

    fn bytes(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        let mut n = self.0;
        for i in (0..8).rev() {
            bytes[i] = (n % 26) as u8 + b'a';
            n /= 26;
        }
        bytes
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes()).unwrap())
    }
}

mod parsing {
    use super::Password;

    impl std::str::FromStr for Password {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() != 8 || s.bytes().any(|b| !b.is_ascii_lowercase()) {
                anyhow::bail!("Invalid password");
            }
            Ok(Password(
                s.bytes().fold(0, |acc, b| acc * 26 + (b - b'a') as u64),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn parse_and_display() {
        let password: Password = "abcdzyxw".parse().unwrap();
        assert_eq!(password.to_string(), "abcdzyxw");
    }

    #[test]
    fn is_valid_1() {
        let password: Password = "hijklmmn".parse().unwrap();
        assert!(!password.is_valid());
    }

    #[test]
    fn is_valid_2() {
        let password: Password = "abbceffg".parse().unwrap();
        assert!(!password.is_valid());
    }

    #[test]
    fn is_valid_3() {
        let password: Password = "abbcegjk".parse().unwrap();
        assert!(!password.is_valid());
    }

    #[test]
    fn is_valid_4() {
        let password: Password = "abcdffaa".parse().unwrap();
        assert!(password.is_valid());
    }

    #[test]
    fn is_valid_5() {
        let password: Password = "ghjaabcc".parse().unwrap();
        assert!(password.is_valid());
    }

    #[test]
    fn next_valid_1() {
        let password: Password = "abcdefgh".parse().unwrap();
        assert_eq!(password.next_valid().to_string(), "abcdffaa");
    }

    #[test]
    fn next_valid_2() {
        let password: Password = "ghijklmn".parse().unwrap();
        assert_eq!(password.next_valid().to_string(), "ghjaabcc");
    }
}
