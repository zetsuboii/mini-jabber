use std::io::Cursor;

use color_eyre::eyre;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};

use super::serialize::{XmlCustomDeserialize, XmlCustomSerialize};

pub struct StreamHeader {
    pub from: String,
    pub to: String,
    pub version: String,
    pub xml_lang: String,
    pub xmlns: String,
    pub xmlns_stream: String,
}

impl XmlCustomSerialize for StreamHeader {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));

        let mut stream_header = BytesStart::new("stream:stream");
        stream_header.push_attribute(("from", self.from.as_str()));
        stream_header.push_attribute(("to", self.to.as_str()));
        stream_header.push_attribute(("version", self.version.as_str()));
        stream_header.push_attribute(("xml:lang", self.xml_lang.as_str()));
        stream_header.push_attribute(("xmlns", self.xmlns.as_str()));
        stream_header.push_attribute(("xmlns:stream", self.xmlns_stream.as_str()));

        writer.write_event(Event::Start(stream_header)).unwrap();
        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

impl XmlCustomDeserialize for StreamHeader {
    fn from_string(value: &str) -> eyre::Result<Self> {
        let mut reader = Reader::from_str(value);

        let mut from: Option<String> = None;
        let mut to: Option<String> = None;
        let mut version: Option<String> = None;
        let mut xml_lang: Option<String> = None;
        let mut xmlns: Option<String> = None;
        let mut xmlns_stream: Option<String> = None;

        loop {
            if let Ok(event) = reader.read_event() {
                match event {
                    Event::Eof => break,
                    Event::Start(e) => {
                        let name = e.name();
                        if name.as_ref() != b"stream:stream" {
                            eyre::bail!(
                                "expected stream:stream got {:?}",
                                std::str::from_utf8(name.as_ref())
                            );
                        }

                        e.attributes().for_each(|attr| {
                            if let Ok(attr) = attr {
                                let key = attr.key.0;
                                let value = std::str::from_utf8(&attr.value).unwrap().to_string();

                                match key {
                                    b"from" => from = Some(value),
                                    b"to" => to = Some(value),
                                    b"version" => version = Some(value),
                                    b"xml:lang" => xml_lang = Some(value),
                                    b"xmlns" => xmlns = Some(value),
                                    b"xmlns:stream" => xmlns_stream = Some(value),
                                    _ => {}
                                }
                            }
                        })
                    }
                    _ => {}
                }
            }
        }

        return Ok(StreamHeader {
            from: from.ok_or(eyre::eyre!("from"))?,
            to: to.ok_or(eyre::eyre!("to"))?,
            version: version.ok_or(eyre::eyre!("version"))?,
            xml_lang: xml_lang.ok_or(eyre::eyre!("xml:lang"))?,
            xmlns: xmlns.ok_or(eyre::eyre!("xmlns"))?,
            xmlns_stream: xmlns_stream.ok_or(eyre::eyre!("xmlns:stream"))?,
        });
    }
}

impl StreamHeader {
    pub fn into_response(self, id: String) -> StreamHeaderResponse {
        StreamHeaderResponse {
            id,
            from: self.from,
            to: self.to,
            version: self.version,
            xml_lang: self.xml_lang,
            xmlns: self.xmlns,
            xmlns_stream: self.xmlns_stream,
        }
    }
}

pub struct StreamHeaderResponse {
    pub id: String,
    pub from: String,
    pub to: String,
    pub version: String,
    pub xml_lang: String,
    pub xmlns: String,
    pub xmlns_stream: String,
}

impl XmlCustomSerialize for StreamHeaderResponse {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));

        let mut stream_header = BytesStart::new("stream:stream");
        stream_header.push_attribute(("id", self.id.as_str()));
        stream_header.push_attribute(("from", self.from.as_str()));
        stream_header.push_attribute(("to", self.to.as_str()));
        stream_header.push_attribute(("version", self.version.as_str()));
        stream_header.push_attribute(("xml:lang", self.xml_lang.as_str()));
        stream_header.push_attribute(("xmlns", self.xmlns.as_str()));
        stream_header.push_attribute(("xmlns:stream", self.xmlns_stream.as_str()));

        writer.write_event(Event::Start(stream_header)).unwrap();
        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

impl XmlCustomDeserialize for StreamHeaderResponse {
    fn from_string(value: &str) -> eyre::Result<Self> {
        let mut reader = Reader::from_str(value);

        let mut id: Option<String> = None;
        let mut from: Option<String> = None;
        let mut to: Option<String> = None;
        let mut version: Option<String> = None;
        let mut xml_lang: Option<String> = None;
        let mut xmlns: Option<String> = None;
        let mut xmlns_stream: Option<String> = None;

        loop {
            if let Ok(event) = reader.read_event() {
                match event {
                    Event::Eof => break,
                    Event::Start(e) => {
                        let name = e.name();
                        if name.as_ref() != b"stream:stream" {
                            eyre::bail!(
                                "expected stream:stream got {:?}",
                                std::str::from_utf8(name.as_ref())
                            );
                        }

                        e.attributes().for_each(|attr| {
                            if let Ok(attr) = attr {
                                let key = attr.key.0;
                                let value = std::str::from_utf8(&attr.value).unwrap().to_string();

                                match key {
                                    b"id" => id = Some(value),
                                    b"from" => from = Some(value),
                                    b"to" => to = Some(value),
                                    b"version" => version = Some(value),
                                    b"xml:lang" => xml_lang = Some(value),
                                    b"xmlns" => xmlns = Some(value),
                                    b"xmlns:stream" => xmlns_stream = Some(value),
                                    _ => {}
                                }
                            }
                        })
                    }
                    _ => {}
                }
            }
        }

        return Ok(StreamHeaderResponse {
            id: id.ok_or(eyre::eyre!("id"))?,
            from: from.ok_or(eyre::eyre!("from"))?,
            to: to.ok_or(eyre::eyre!("to"))?,
            version: version.ok_or(eyre::eyre!("version"))?,
            xml_lang: xml_lang.ok_or(eyre::eyre!("xml:lang"))?,
            xmlns: xmlns.ok_or(eyre::eyre!("xmlns"))?,
            xmlns_stream: xmlns_stream.ok_or(eyre::eyre!("xmlns:stream"))?,
        });
    }
}

pub struct StreamFeatures {
    pub start_tls: Option<StartTls>,
    pub mechanisms: Option<Mechanisms>,
}

impl StreamFeatures {
    pub fn empty(&self) -> bool {
        self.start_tls.is_none() && self.mechanisms.is_none()
    }
}

impl XmlCustomSerialize for StreamFeatures {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));
        let stream_features_start = BytesStart::new("stream:features");

        if self.empty() {
            // If there are no features, return <stream:features />
            writer
                .write_event(Event::Empty(stream_features_start))
                .unwrap();
            return std::str::from_utf8(writer.into_inner().into_inner().as_slice())
                .unwrap()
                .to_string();
        }

        // <stream:features>
        writer
            .write_event(Event::Start(stream_features_start))
            .unwrap();

        if let Some(start_tls) = &self.start_tls {
            let mut start_tls_start = BytesStart::new("starttls");
            start_tls_start.push_attribute(("xmlns", start_tls.xmlns.as_ref()));

            if start_tls.required {
                // <starttls xmlns>
                writer.write_event(Event::Start(start_tls_start)).unwrap();
                // <required/>
                writer
                    .write_event(Event::Empty(BytesStart::new("required")))
                    .unwrap();
                let start_tls_end = BytesEnd::new("starttls");
                // </starttls>
                writer.write_event(Event::End(start_tls_end)).unwrap();
            } else {
                writer.write_event(Event::Empty(start_tls_start)).unwrap();
            }
        }

        if let Some(mechanisms) = &self.mechanisms {
            let mut mechanisms_start = BytesStart::new("mechanisms");
            mechanisms_start.push_attribute(("xmlns", mechanisms.xmlns.as_ref()));
            // <mechanisms>
            writer.write_event(Event::Start(mechanisms_start)).unwrap();

            for mechanism in &mechanisms.mechanisms {
                // <mechanism>
                writer
                    .write_event(Event::Start(BytesStart::new("mechanism")))
                    .unwrap();
                // Text
                writer
                    .write_event(Event::Text(BytesText::new(mechanism.0.as_ref())))
                    .unwrap();
                // </mechanism>
                writer
                    .write_event(Event::End(BytesEnd::new("mechanism")))
                    .unwrap();
            }
            // </mechanisms>
            writer
                .write_event(Event::End(BytesEnd::new("mechanisms")))
                .unwrap();
        }

        // </stream:features>
        writer
            .write_event(Event::End(BytesEnd::new("stream:features")))
            .unwrap();
        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

impl XmlCustomDeserialize for StreamFeatures {
    fn from_string(value: &str) -> eyre::Result<Self> {
        let mut reader = Reader::from_str(value);

        let mut header_found = false;
        let mut start_tls: Option<StartTls> = None;
        let mut mechanisms: Option<Mechanisms> = None;

        loop {
            if let Ok(event) = reader.read_event() {
                match event {
                    Event::Eof => break,
                    Event::Empty(e) => {
                        let name = e.name();
                        match name.as_ref() {
                            b"starttls" => {
                                if !header_found {
                                    eyre::bail!("header not found")
                                } else if start_tls.is_some() {
                                    eyre::bail!("starttls exists");
                                }

                                let xmlns = std::str::from_utf8(
                                    &e.try_get_attribute("xmlns").unwrap().unwrap().value,
                                )
                                .unwrap()
                                .to_string();

                                start_tls = Some(StartTls {
                                    xmlns,
                                    required: false,
                                });
                            }
                            _ => {}
                        }
                    }
                    Event::Start(e) => {
                        let name = e.name();
                        match name.as_ref() {
                            b"stream:features" => header_found = true,
                            b"starttls" => {
                                if !header_found {
                                    eyre::bail!("header not found")
                                } else if start_tls.is_some() {
                                    eyre::bail!("starttls exists");
                                }

                                let xmlns = std::str::from_utf8(
                                    &e.try_get_attribute("xmlns").unwrap().unwrap().value,
                                )
                                .unwrap()
                                .to_string();

                                let mut required = false;

                                while let Ok(event) = reader.read_event() {
                                    match event {
                                        Event::Empty(e) => {
                                            if e.name().as_ref() == b"required" {
                                                required = true;
                                            }
                                        }
                                        Event::End(_) => {
                                            break;
                                        }
                                        _ => {}
                                    }
                                }

                                start_tls = Some(StartTls { xmlns, required });
                            }
                            b"mechanisms" => {
                                if !header_found {
                                    eyre::bail!("header not found")
                                }

                                let xmlns = std::str::from_utf8(
                                    &e.try_get_attribute("xmlns").unwrap().unwrap().value,
                                )
                                .unwrap()
                                .to_string();

                                let mut values = Vec::new();

                                while let Ok(event) = reader.read_event() {
                                    match event {
                                        Event::Text(text) => {
                                            let text =
                                                std::str::from_utf8(&text).unwrap().to_string();
                                            values.push(Mechanism(text));
                                        }
                                        Event::End(_) => break,
                                        _ => {}
                                    }
                                }

                                mechanisms = Some(Mechanisms {
                                    xmlns,
                                    mechanisms: values,
                                })
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(StreamFeatures {
            start_tls,
            mechanisms,
        })
    }
}

pub struct StartTls {
    pub xmlns: String,
    pub required: bool,
}

impl XmlCustomSerialize for StartTls {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));
        let mut start_tls_start = BytesStart::new("starttls");
        start_tls_start.push_attribute(("xmlns", self.xmlns.as_ref()));

        if self.required {
            // <starttls xmlns>
            writer.write_event(Event::Start(start_tls_start)).unwrap();
            // <required/>
            writer
                .write_event(Event::Empty(BytesStart::new("required")))
                .unwrap();
            let start_tls_end = BytesEnd::new("starttls");
            // </starttls>
            writer.write_event(Event::End(start_tls_end)).unwrap();
        } else {
            writer.write_event(Event::Empty(start_tls_start)).unwrap();
        }

        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

impl XmlCustomDeserialize for StartTls {
    fn from_string(value: &str) -> eyre::Result<Self> {
        let mut reader = Reader::from_str(value);

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Empty(e) => {
                    let xmlns =
                        std::str::from_utf8(&e.try_get_attribute("xmlns").unwrap().unwrap().value)
                            .unwrap()
                            .to_string();

                    if e.name().as_ref() == b"starttls" {
                        return Ok(StartTls {
                            xmlns,
                            required: false,
                        });
                    }
                }
                Event::Start(e) => {
                    let xmlns =
                        std::str::from_utf8(&e.try_get_attribute("xmlns").unwrap().unwrap().value)
                            .unwrap()
                            .to_string();

                    let mut required = false;

                    while let Ok(event) = reader.read_event() {
                        match event {
                            Event::Empty(e) => {
                                if e.name().as_ref() == b"required" {
                                    required = true;
                                }
                            }
                            Event::End(_) => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    return Ok(StartTls { xmlns, required });
                }
                _ => {}
            }
        }

        eyre::bail!("failed to parse")
    }
}

pub enum StartTlsResponse {
    Proceed(StartTlsProceed),
    Failure(StartTlsFailure),
}

impl XmlCustomDeserialize for StartTlsResponse {
    fn from_string(value: &str) -> eyre::Result<Self> {
        let mut reader = Reader::from_str(value);

        if let Ok(event) = reader.read_event() {
            match event {
                Event::Empty(e) => match e.name().as_ref() {
                    b"proceed" => return Ok(StartTlsResponse::Proceed(StartTlsProceed())),
                    b"failure" => return Ok(StartTlsResponse::Failure(StartTlsFailure())),
                    _ => {}
                },
                _ => {}
            }
        }
        eyre::bail!("invalid response")
    }
}

pub struct StartTlsProceed();

impl XmlCustomSerialize for StartTlsProceed {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));
        writer
            .write_event(Event::Empty(BytesStart::new("proceed")))
            .unwrap();

        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

pub struct StartTlsFailure();

impl XmlCustomSerialize for StartTlsFailure {
    fn into_string(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::<u8>::new()));
        writer
            .write_event(Event::Empty(BytesStart::new("failure")))
            .unwrap();

        std::str::from_utf8(writer.into_inner().into_inner().as_slice())
            .unwrap()
            .to_string()
    }
}

pub struct Mechanisms {
    pub xmlns: String,
    pub mechanisms: Vec<Mechanism>,
}

pub struct Mechanism(pub String);
