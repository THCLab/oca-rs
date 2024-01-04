use recursion::{Expandable, PartiallyApplied, MappableFrame, Collapsible};

use super::{NestedAttrType, error::AttributeError, RefValue, AttributeType};

/// This module includes structures for setting up the recursion crate. They
/// enable usage of `expand_frames` and `collapse_frames` functions for
/// NestedAttrType.

pub enum NestedAttrTypeFrame<A> {
    Reference(RefValue),
    Value(AttributeType),
    Array(A),
    Null,
}

impl MappableFrame for NestedAttrTypeFrame<PartiallyApplied> {
    type Frame<X> = NestedAttrTypeFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            NestedAttrTypeFrame::Reference(reference) => NestedAttrTypeFrame::Reference(reference),
            NestedAttrTypeFrame::Value(val) => NestedAttrTypeFrame::Value(val),
            NestedAttrTypeFrame::Array(t) => NestedAttrTypeFrame::Array(f(t)),
            NestedAttrTypeFrame::Null => NestedAttrTypeFrame::Null,
        }
    }
}

impl Expandable for NestedAttrType {
    type FrameToken = NestedAttrTypeFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        match val {
            NestedAttrTypeFrame::Reference(reference) => NestedAttrType::Reference(reference),
            NestedAttrTypeFrame::Value(v) => NestedAttrType::Value(v),
            NestedAttrTypeFrame::Array(arr) => NestedAttrType::Array(Box::new(arr)),
            NestedAttrTypeFrame::Null => NestedAttrType::Null,
        }
    }
}

impl Collapsible for NestedAttrType {
    type FrameToken = NestedAttrTypeFrame<PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            NestedAttrType::Reference(reference) => NestedAttrTypeFrame::Reference(reference),
            NestedAttrType::Value(val) => NestedAttrTypeFrame::Value(val),
            NestedAttrType::Array(arr) => NestedAttrTypeFrame::Array(*arr),
            NestedAttrType::Null => NestedAttrTypeFrame::Null,
        }
    }
}

pub struct AttributeTypeResult(Result<NestedAttrType, AttributeError>);
pub struct AttributeTypeResultFrame<A>(Result<NestedAttrTypeFrame<A>, AttributeError>);

impl MappableFrame for AttributeTypeResultFrame<PartiallyApplied> {
    type Frame<X> = AttributeTypeResultFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input.0 {
            Ok(frame) => AttributeTypeResultFrame(Ok(NestedAttrTypeFrame::map_frame(frame, f))),
            Err(e) => AttributeTypeResultFrame(Err(e)),
        }
    }
}

impl AttributeTypeResult {
    pub fn value(self) -> Result<NestedAttrType, AttributeError> {
        self.0
    } 
}

impl Expandable for AttributeTypeResult {
    type FrameToken = AttributeTypeResultFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        let val = match val.0  {
            Ok(NestedAttrTypeFrame::Value(v)) => Ok(NestedAttrType::Value(v)),
            Ok(NestedAttrTypeFrame::Reference(r)) => Ok(NestedAttrType::Reference(r)),
            Ok(NestedAttrTypeFrame::Array(v)) =>
				match v.0 {
					Ok(ok) => Ok(NestedAttrType::Array(Box::new(ok))),
					Err(er) => Err(er),
				},
            Ok(NestedAttrTypeFrame::Null) => Ok(NestedAttrType::Null),
            Err(er) => Err(er),
        };
        Self(val)
    }
}

impl From<AttributeError> for AttributeTypeResult {
    fn from(value: AttributeError) -> Self {
        AttributeTypeResult(Err(value))
    }
}

impl<A> From<AttributeError> for AttributeTypeResultFrame<A> {
    fn from(value: AttributeError) -> Self {
        AttributeTypeResultFrame(Err(value))
    }
}

impl<A> From<NestedAttrTypeFrame<A>> for AttributeTypeResultFrame<A> {
    fn from(value: NestedAttrTypeFrame<A>) -> Self {
        AttributeTypeResultFrame(Ok(value))
    }
}