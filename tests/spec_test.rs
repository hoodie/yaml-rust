#![allow(dead_code)]
extern crate yaml_rust;

use yaml_rust::parser::{Parser, EventReceiver, Event};
use yaml_rust::yaml::Yaml;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum TestEvent {
    OnDocumentStart,
    OnDocumentEnd,
    OnSequenceStart,
    OnSequenceEnd,
    OnMapStart,
    OnMapEnd,
    OnScalar,
    OnAlias,
    OnNull,
}

struct YamlChecker {
    pub evs: Vec<TestEvent>
}

impl EventReceiver for YamlChecker {
    fn on_event(&mut self, ev: &Event) {
        let tev = match *ev {
            Event::DocumentStart => TestEvent::OnDocumentStart,
            Event::DocumentEnd => TestEvent::OnDocumentEnd,
            Event::SequenceStart => TestEvent::OnSequenceStart,
            Event::SequenceEnd => TestEvent::OnSequenceEnd,
            Event::MappingStart => TestEvent::OnMapStart,
            Event::MappingEnd => TestEvent::OnMapEnd,
            Event::Scalar(ref v, style) => {
                if v == "~" {
                    TestEvent::OnNull
                } else {
                    TestEvent::OnScalar
                }
            },
            _ => { return } // ignore other events
        };
        self.evs.push(tev);
    }
}

fn str_to_test_events(docs: &str) -> Vec<TestEvent> {
    let mut p = YamlChecker {
        evs: Vec::new()
    };
    let mut parser = Parser::new(docs.chars());
    parser.load(&mut p, true).unwrap();
    p.evs
}

macro_rules! assert_next {
    ($v:expr, $p:pat) => (
        match $v.next().unwrap() {
            $p => {},
            e => { panic!("unexpected event: {:?}", e); }
        }
    )
}

// auto generated from handler_spec_test.cpp
include!("specexamples.rs.inc");
include!("spec_test.rs.inc");
