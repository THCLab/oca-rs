use recursion::{Expandable, PartiallyApplied, MappableFrame};

use super::{attributes::NestedAttrTypeFrame, NestedAttrType, error::AttributeError};


pub struct NestedResult(pub Result<NestedAttrType, AttributeError>);
pub struct NestedResultFrame<A>(pub Result<NestedAttrTypeFrame<A>, AttributeError>);

impl MappableFrame for NestedResultFrame<PartiallyApplied> {
    type Frame<X> = NestedResultFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input.0 {
            Ok(frame) => NestedResultFrame(Ok(NestedAttrTypeFrame::map_frame(frame, f))),
            Err(e) => NestedResultFrame(Err(e)),
        }
    }
}

impl Expandable for NestedResult {
    type FrameToken = NestedResultFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        let val = match val.0  {
            Ok(NestedAttrTypeFrame::Value(v)) => {NestedAttrType::Value(v)},
            Ok(NestedAttrTypeFrame::Reference(r)) => {NestedAttrType::Reference(r)},
            Ok(NestedAttrTypeFrame::Object(o)) => todo!(),
            Ok(NestedAttrTypeFrame::Array(v)) => {
				match v.0 {
					Ok(ok) => NestedAttrType::Array(Box::new(ok)),
					Err(er) => return NestedResult(Err(er)),
				}
			},
            Ok(NestedAttrTypeFrame::Null) => {NestedAttrType::Null},
            Err(er) => return NestedResult(Err(er)),
        };
        Self(Ok(val))
    }
}
