use actix_web::HttpRequest;
use actix_web::error::ParseError;
use actix_web::http::header::Accept;
use actix_web::http::header::Header as _;
use mime::Mime;

pub enum Conclusion {
    AllAcceptable,
    ErrorParsingHeader(ParseError),
    NotAllAcceptable,
}

fn accepts(choices: &[Mime], mime: Mime) -> bool {
    for choice in choices {
        match choice.type_() {
            mime::STAR => return true,
            type_ => {
                if type_ == mime.type_() {
                    match choice.subtype() {
                        mime::STAR => return true,
                        subtype => {
                            if subtype == mime.subtype() {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

pub fn accepts_all(req: &HttpRequest, mimes: Vec<Mime>) -> Conclusion {
    match Accept::parse(req) {
        Ok(accept) => {
            let ranked = accept.ranked();

            for mime in mimes {
                if !accepts(&ranked, mime) {
                    return Conclusion::NotAllAcceptable;
                }
            }

            Conclusion::AllAcceptable
        }
        Err(err) => Conclusion::ErrorParsingHeader(err),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn accepts_mime() -> Result<()> {
        assert!(accepts(
            &["image/something".parse()?, "text/*".parse()?,],
            "text/event-stream".parse()?
        ));
        assert!(accepts(&["*/*".parse()?,], "text/event-stream".parse()?));

        Ok(())
    }

    #[test]
    fn does_not_accept_mime() -> Result<()> {
        assert!(!accepts(&["text/*".parse()?,], "something/else".parse()?));

        Ok(())
    }
}
