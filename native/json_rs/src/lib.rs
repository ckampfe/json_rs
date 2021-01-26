use rustler::{Atom, Error, ListIterator, MapIterator, Term};
use serde::ser::Error as SerdeError;
use serde::ser::{SerializeMap, SerializeSeq};

mod atoms {
    #[inline]
    rustler::atoms! {
        ok,
        error,
    }
}

rustler::init!("Elixir.JsonRs", [encode, decode]);

#[rustler::nif]
fn encode(t: Term) -> Result<(Atom, String), Error> {
    let jt: SerdeTerm = t.into();
    let s = serde_json::to_string(&jt).unwrap();
    Ok((crate::atoms::ok(), s))
}

#[rustler::nif]
fn decode(s: &str) -> Result<Term, Error> {
    todo!()
}

struct SerdeTerm<'j>(Term<'j>);

impl<'j> From<Term<'j>> for SerdeTerm<'j> {
    #[inline]
    fn from(term: Term<'j>) -> Self {
        SerdeTerm(term)
    }
}

impl<'j> serde::Serialize for SerdeTerm<'j> {
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
                let s = self.0.atom_to_string().unwrap();
                if s == "nil" {
                    serializer.serialize_none()
                } else {
                    serializer.serialize_str(&s)
                }
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
                let mut seq = serializer.serialize_seq(Some(size)).unwrap();
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
