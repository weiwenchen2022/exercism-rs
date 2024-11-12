pub fn encode(source: &str) -> String {
    use std::borrow::Cow;

    let mut s = String::with_capacity(source.len());
    let mut cn = None::<(u8, usize)>;

    for b in source.bytes() {
        if let Some((c, n)) = &mut cn {
            if *c != b {
                s.push_str(&format!(
                    "{}{}",
                    if *n > 1 {
                        Cow::Owned(format!("{n}"))
                    } else {
                        Cow::Borrowed("")
                    },
                    *c as char
                ));

                cn = Some((b, 1));
            } else {
                *n += 1;
            }
        } else {
            cn = Some((b, 1));
        }
    }

    if let Some((c, n)) = cn {
        s.push_str(&format!(
            "{}{}",
            if n > 1 {
                Cow::Owned(format!("{n}"))
            } else {
                Cow::Borrowed("")
            },
            c as char
        ));
    }

    s.shrink_to_fit();
    s
}

pub fn decode(source: &str) -> String {
    // todo!("Return the run-length decoding of {source}.");
    let mut s = String::new();

    let mut n = None::<u32>;
    for b in source.bytes() {
        if (b as char).is_ascii_digit() {
            n = Some(n.unwrap_or_default() * 10 + (b as char).to_digit(10).unwrap());
        } else {
            s.push_str(
                &(b as char)
                    .to_string()
                    .repeat(n.take().unwrap_or(1) as usize),
            );
        }
    }

    s
}
