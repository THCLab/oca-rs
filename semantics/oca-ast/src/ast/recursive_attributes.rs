use std::error::Error;

use recursion::{Collapsible, Expandable, MappableFrame, PartiallyApplied};

use super::{AttributeType, NestedAttrType, RefValue};

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

pub struct AttributeTypeResult<E: Error>(Result<NestedAttrType, E>);
pub struct AttributeTypeResultFrame<A, E: Error>(Result<NestedAttrTypeFrame<A>, E>);

impl<E: Error> MappableFrame for AttributeTypeResultFrame<PartiallyApplied, E> {
    type Frame<X> = AttributeTypeResultFrame<X, E>;

    fn map_frame<A, B>(input: Self::Frame<A>, f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input.0 {
            Ok(frame) => AttributeTypeResultFrame(Ok(NestedAttrTypeFrame::map_frame(frame, f))),
            Err(e) => AttributeTypeResultFrame(Err(e)),
        }
    }
}

impl<E: Error> AttributeTypeResult<E> {
    pub fn value(self) -> Result<NestedAttrType, E> {
        self.0
    }
}

impl<E: Error> Expandable for AttributeTypeResult<E> {
    type FrameToken = AttributeTypeResultFrame<PartiallyApplied, E>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        let val = match val.0 {
            Ok(NestedAttrTypeFrame::Value(v)) => Ok(NestedAttrType::Value(v)),
            Ok(NestedAttrTypeFrame::Reference(r)) => Ok(NestedAttrType::Reference(r)),
            Ok(NestedAttrTypeFrame::Array(v)) => match v.0 {
                Ok(ok) => Ok(NestedAttrType::Array(Box::new(ok))),
                Err(er) => Err(er),
            },
            Ok(NestedAttrTypeFrame::Null) => Ok(NestedAttrType::Null),
            Err(er) => Err(er),
        };
        Self(val)
    }
}

impl<E: Error> From<E> for AttributeTypeResult<E> {
    fn from(value: E) -> Self {
        AttributeTypeResult(Err(value))
    }
}

impl<A, E: Error> From<E> for AttributeTypeResultFrame<A, E> {
    fn from(value: E) -> Self {
        AttributeTypeResultFrame(Err(value))
    }
}

impl<A, E: Error> From<NestedAttrTypeFrame<A>> for AttributeTypeResultFrame<A, E> {
    fn from(value: NestedAttrTypeFrame<A>) -> Self {
        AttributeTypeResultFrame(Ok(value))
    }
}
