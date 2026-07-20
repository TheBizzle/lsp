use std::sync::Arc;

use crate::analyzer::function::Function as Func;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrganicType<'a> {
  AudioEffect,
  AudioSource,
  Boolean,
  Function(Arc<Func<'a>>),
  List(&'a Self),
  Number,
  RandomArg,
  RoundArg,
  SequenceArg,
  String,
  Unknown,
}
