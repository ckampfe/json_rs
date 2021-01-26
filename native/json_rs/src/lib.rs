use num_traits::cast::ToPrimitive;
use rustler::{Atom, Error, ListIterator, MapIterator, Term};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{ser::Error as SerdeError, serde_if_integer128};
use std::slice::from_raw_parts;

const ETF_VERSION_NUMBER_BYTE: u8 = 131;
const ETF_SMALL_BIG_TAG_BYTE: u8 = 110;
const ETF_LARGE_BIG_TAG_BYTE: u8 = 111;

mod atoms {
    #[inline]
    rustler::atoms! {
        ok,
        error,
    }
}

rustler::init!("Elixir.JsonRs", [encode, decode]);

#[rustler::nif]
fn encode(t: Term, pretty: bool) -> Result<(Atom, String), Error> {
    let serde_term: SerdeTerm = t.into();
    let s = if pretty {
        serde_json::to_string_pretty(&serde_term).map_err(|e| {
            let r = e.to_string();
            rustler::Error::RaiseTerm(Box::new(r))
        })?
    } else {
        serde_json::to_string(&serde_term).map_err(|e| {
            let r = e.to_string();
            rustler::Error::RaiseTerm(Box::new(r))
        })?
    };

    Ok((crate::atoms::ok(), s))
}

#[rustler::nif]
fn decode(s: &str) -> Result<Term, Error> {
    todo!()
}

struct SerdeTerm<'t>(Term<'t>);

impl<'t> From<Term<'t>> for SerdeTerm<'t> {
    #[inline]
    fn from(term: Term<'t>) -> Self {
        SerdeTerm(term)
    }
}

impl<'t> serde::Serialize for SerdeTerm<'t> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0.get_type() {
            rustler::TermType::Binary => {
                let s = self.0.decode().unwrap();
                serializer.serialize_str(s)
            }
            rustler::TermType::Atom => {
                if self.0.as_c_arg() == rustler::types::atom::nil().as_c_arg() {
                    return serializer.serialize_unit();
                }
                if self.0.as_c_arg() == rustler::types::atom::true_().as_c_arg() {
                    return serializer.serialize_bool(true);
                }
                if self.0.as_c_arg() == rustler::types::atom::false_().as_c_arg() {
                    return serializer.serialize_bool(false);
                }

                let s = self.0.atom_to_string().unwrap();
                serializer.serialize_str(&s)
            }
            rustler::TermType::Map => {
                let s: MapIterator = self.0.decode().unwrap();
                let size = Term::map_size(self.0).unwrap();
                let mut map = serializer.serialize_map(Some(size))?;
                for (k, v) in s {
                    let k: SerdeTerm = k.into();
                    let v: SerdeTerm = v.into();
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            rustler::TermType::List => {
                let s: ListIterator = self.0.decode().unwrap();
                let size = self.0.list_length().unwrap();
                let mut seq = serializer.serialize_seq(Some(size))?;
                for el in s {
                    let el: SerdeTerm = el.into();
                    seq.serialize_element(&el)?;
                }
                seq.end()
            }
            rustler::TermType::Number => {
                // note:
                // http://erlang.2086793.n4.nabble.com/Convert-bigint-to-float-in-erl-nif-td4673925.html
                #[cfg(target_pointer_width = "64")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_i64(s);
                }
                #[cfg(target_pointer_width = "64")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_f64(s);
                }
                #[cfg(target_pointer_width = "64")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_u64(s);
                }

                #[cfg(target_pointer_width = "32")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_i32(s);
                }
                #[cfg(target_pointer_width = "32")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_f32(s);
                }
                #[cfg(target_pointer_width = "32")]
                if let Ok(s) = self.0.decode() {
                    return serializer.serialize_u32(s);
                }

                serde_if_integer128! {
                    let binary = unsafe {
                        rustler::wrapper::env::term_to_binary(
                            self.0.get_env().as_c_arg(),
                            self.0.as_c_arg(),
                        )
                        .unwrap()
                    };
                    let binary_bytes = unsafe { from_raw_parts(binary.data, binary.size) };

                    match binary_bytes {
                        [ETF_VERSION_NUMBER_BYTE, ETF_SMALL_BIG_TAG_BYTE, _length_byte, sign_byte, digits @ ..] =>
                        {
                            let sign = if sign_byte == &1u8 {
                                num_bigint::Sign::Minus
                            } else {
                                num_bigint::Sign::Plus
                            };

                            let sb = num_bigint::BigInt::from_radix_le(sign, &digits, 256).unwrap();

                            if let Some(i) = sb.to_u128() {
                                return serializer.serialize_u128(i);
                            }
                            if let Some(i) = sb.to_i128() {
                                return serializer.serialize_i128(i);
                            }
                        }
                        [ETF_VERSION_NUMBER_BYTE, ETF_LARGE_BIG_TAG_BYTE, _length_byte_1, _length_byte_2, _length_byte_3, _length_byte_4, sign_byte, digits @ ..] =>
                        {
                            let sign = if sign_byte == &1u8 {
                                num_bigint::Sign::Minus
                            } else {
                                num_bigint::Sign::Plus
                            };

                            let sb = num_bigint::BigInt::from_radix_le(sign, &digits, 256).unwrap();

                            if let Some(i) = sb.to_u128() {
                                return serializer.serialize_u128(i);
                            }
                            if let Some(i) = sb.to_i128() {
                                return serializer.serialize_i128(i);
                            }
                        }
                        _ => ()
                    }
                }

                Err(S::Error::custom(format!(
                    "unable to serialize {:?} as number",
                    self.0
                )))
            }
            rustler::TermType::EmptyList => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            // rustler::TermType::Exception => {}
            // rustler::TermType::Fun => {}
            // rustler::TermType::Pid => {}
            // rustler::TermType::Port => {}
            // rustler::TermType::Ref => {}
            // rustler::TermType::Tuple => {},
            // rustler::TermType::Unknown => {}
            _ => Err(S::Error::custom(format!(
                "unable to serialize {:?}",
                self.0
            ))),
        }
    }
}
