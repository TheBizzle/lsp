use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use crate::analyzer::address::{NamedVarAddress, ScopeAddress};
use crate::analyzer::function::{Function, ParamInfo as PI};
use crate::analyzer::organic_type::OrganicType;
use OrganicType as OT;

struct Constant {
  name: &'static str,
  typ: OrganicType<'static>,
}

struct StdLibFn {
  name: &'static str,
  func: Function<'static>,
}

#[must_use]
pub fn initial_state() -> (HashMap<String, NamedVarAddress>, HashMap<NamedVarAddress, OrganicType<'static>>) {
  let (bindings, vars) = &*INITIAL_STATE;
  (bindings.clone(), vars.clone())
}

#[allow(clippy::type_complexity)]
static INITIAL_STATE: LazyLock<(
  HashMap<String, NamedVarAddress>,
  HashMap<NamedVarAddress, OrganicType<'static>>,
)> = LazyLock::new(|| {
  let scope_addr = INITIAL_SCOPE_ADDRESS;

  let constants: HashMap<_, _> = CONSTANTS
    .iter()
    .map(|Constant { name, typ }| {
      (NamedVarAddress { name: name.to_string(), scope_addr: scope_addr.clone() }, typ.clone())
    })
    .collect();

  let functions: HashMap<_, _> = FUNCTIONS
    .iter()
    .map(|StdLibFn { name, func }| {
      (
        NamedVarAddress { name: name.to_string(), scope_addr: scope_addr.clone() },
        OrganicType::Function(Arc::new(func.clone())),
      )
    })
    .collect();

  let notes: HashMap<_, _> = NOTES
    .iter()
    .map(|name| (NamedVarAddress { name: name.clone(), scope_addr: scope_addr.clone() }, OrganicType::Number))
    .collect();

  let vars: HashMap<_, _> = constants.into_iter().chain(functions).chain(notes).collect();
  let bindings: HashMap<_, _> = vars.keys().map(|addr| (addr.name.clone(), addr.clone())).collect();

  (bindings, vars)
});

pub static INITIAL_SCOPE_ADDRESS: &ScopeAddress = &ScopeAddress { n: 0 };

static FUNCTIONS: LazyLock<[StdLibFn; 32]> = LazyLock::new(|| {
  [
    StdLibFn {
      name: "absolute",
      func: Function { params: vec![PI(cb("value"), OT::Number, false)], return_type: OT::Number },
    },
    StdLibFn {
      name: "all",
      func: Function {
        params: vec![PI(cb("values"), OT::List(&OT::Boolean), false)],
        return_type: OT::Boolean,
      },
    },
    StdLibFn {
      name: "all-pass",
      func: Function {
        params: vec![
          PI(cb("feedback"), OT::Number, false),
          PI(cb("delay"), OT::Number, false),
          PI(cb("mix"), OT::Number, true),
        ],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "any",
      func: Function {
        params: vec![PI(cb("values"), OT::List(&OT::Boolean), false)],
        return_type: OT::Boolean,
      },
    },
    StdLibFn {
      name: "comb",
      func: Function {
        params: vec![
          PI(cb("feedback"), OT::Number, false),
          PI(cb("delay"), OT::Number, false),
          PI(cb("mix"), OT::Number, true),
        ],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "delay",
      func: Function {
        params: vec![
          PI(cb("feedback"), OT::Number, false),
          PI(cb("delay"), OT::Number, false),
          PI(cb("mix"), OT::Number, true),
        ],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "effect-group",
      func: Function {
        params: vec![PI(cb("effects"), OT::List(&OT::AudioEffect), false), PI(cb("mix"), OT::Number, true)],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "granulate",
      func: Function {
        params: vec![
          PI(
            cb("shape"),
            OT::Function(Arc::new(Function {
              params: vec![PI(cb("value"), OT::Number, false)],
              return_type: OT::Number,
            })),
            true,
          ),
          PI(cb("length"), OT::Number, true),
          PI(cb("grains"), OT::Number, true),
          PI(cb("sample"), OT::String, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "group",
      func: Function {
        params: vec![
          PI(cb("sources"), OT::List(&OT::AudioSource), false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "hold",
      func: Function {
        params: vec![PI(cb("length"), OT::Number, false), PI(cb("value"), OT::Number, false)],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "if",
      func: Function {
        params: vec![
          PI(cb("is-false"), OT::Number, false),
          PI(cb("is-true"), OT::Number, false),
          PI(cb("condition"), OT::Boolean, false),
        ],
        return_type: OT::Number,
      },
    },
    StdLibFn {
      name: "lfo",
      func: Function {
        params: vec![
          PI(cb("length"), OT::Number, false),
          PI(cb("to"), OT::Number, false),
          PI(cb("from"), OT::Number, false),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "limit",
      func: Function {
        params: vec![
          PI(cb("max"), OT::Number, false),
          PI(cb("min"), OT::Number, false),
          PI(cb("value"), OT::Number, false),
        ],
        return_type: OT::Number,
      },
    },
    StdLibFn {
      name: "low-pass",
      func: Function { params: vec![PI(cb("threshold"), OT::Number, false)], return_type: OT::Number },
    },
    StdLibFn {
      name: "max",
      func: Function {
        params: vec![PI(cb("values"), OT::List(&OT::Number), false)],
        return_type: OT::Number,
      },
    },
    StdLibFn {
      name: "min",
      func: Function {
        params: vec![PI(cb("values"), OT::List(&OT::Number), false)],
        return_type: OT::Number,
      },
    },
    StdLibFn {
      name: "noise",
      func: Function {
        params: vec![
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "none",
      func: Function {
        params: vec![PI(cb("values"), OT::List(&OT::Boolean), false)],
        return_type: OT::Boolean,
      },
    },
    StdLibFn {
      name: "oscillator",
      func: Function {
        params: vec![
          PI(
            cb("waveform"),
            OT::Function(Arc::new(Function {
              params: vec![PI(cb("phase"), OT::Number, false)],
              return_type: OT::Number,
            })),
            false,
          ),
          PI(cb("frequency"), OT::Number, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "random",
      func: Function {
        params: vec![
          PI(cb("type"), OT::RandomArg, true),
          PI(cb("length"), OT::Number, false),
          PI(cb("to"), OT::Number, false),
          PI(cb("from"), OT::Number, false),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "repeat",
      func: Function {
        params: vec![PI(cb("repeats"), OT::Number, true), PI(cb("value"), OT::Number, false)],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "reverb",
      func: Function {
        params: vec![PI(cb("length"), OT::Number, false), PI(cb("mix"), OT::Number, true)],
        return_type: OT::AudioEffect,
      },
    },
    StdLibFn {
      name: "round",
      func: Function {
        params: vec![
          PI(cb("direction"), OT::RoundArg, true),
          PI(cb("step"), OT::Number, true),
          PI(cb("value"), OT::Number, false),
        ],
        return_type: OT::Number,
      },
    },
    StdLibFn {
      name: "sample",
      func: Function {
        params: vec![
          PI(cb("file"), OT::String, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "saw",
      func: Function {
        params: vec![
          PI(cb("frequency"), OT::Number, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "sequence",
      func: Function {
        params: vec![PI(cb("order"), OT::SequenceArg, true), PI(cb("values"), OT::List(&OT::Number), false)],
        return_type: OT::List(&OT::Number),
      },
    },
    StdLibFn {
      name: "sine",
      func: Function {
        params: vec![
          PI(cb("frequency"), OT::Number, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "square",
      func: Function {
        params: vec![
          PI(cb("frequency"), OT::Number, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "sweep",
      func: Function {
        params: vec![
          PI(cb("length"), OT::Number, false),
          PI(cb("to"), OT::Number, false),
          PI(cb("from"), OT::Number, false),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn { name: "time", func: Function { params: vec![], return_type: OT::Number } },
    StdLibFn {
      name: "triangle",
      func: Function {
        params: vec![
          PI(cb("frequency"), OT::Number, false),
          PI(cb("effects"), OT::List(&OT::AudioEffect), true),
          PI(cb("pan"), OT::Number, true),
          PI(cb("volume"), OT::Number, true),
        ],
        return_type: OT::AudioSource,
      },
    },
    StdLibFn {
      name: "trigger",
      func: Function {
        params: vec![PI(cb("value"), OT::Number, false), PI(cb("condition"), OT::Boolean, false)],
        return_type: OT::Number,
      },
    },
  ]
});

static CONSTANTS: &[Constant] = &[
  Constant { name: "backward", typ: OT::SequenceArg },
  Constant { name: "down", typ: OT::RoundArg },
  Constant { name: "e", typ: OT::Number },
  Constant { name: "false", typ: OT::Boolean },
  Constant { name: "forward", typ: OT::SequenceArg },
  Constant { name: "linear", typ: OT::RandomArg },
  Constant { name: "nearest", typ: OT::RoundArg },
  Constant { name: "pi", typ: OT::Number },
  Constant { name: "ping-pong", typ: OT::SequenceArg },
  Constant { name: "shuffle", typ: OT::SequenceArg },
  Constant { name: "step", typ: OT::RandomArg },
  Constant { name: "tau", typ: OT::Number },
  Constant { name: "true", typ: OT::Boolean },
  Constant { name: "up", typ: OT::RoundArg },
];

// e.g. `as5` is an A# on the 5th octave
static NOTES: LazyLock<Vec<String>> = LazyLock::new(|| {
  ('a'..='g')
    .flat_map(|note| ["f", "", "s"].into_iter().map(move |accidental| (note, accidental)))
    .flat_map(|(note, accidental)| (0..=9).map(move |octave| format!("{note}{accidental}{octave}")))
    .collect()
});

const fn cb<T: ?Sized + ToOwned>(value: &T) -> Cow<'_, T> {
  Cow::Borrowed(value)
}
