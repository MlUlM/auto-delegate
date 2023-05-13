use auto_delegate_macros::Delegate;

use crate::common::increment::child::IncrementChild;

#[derive(Default, Delegate)]
pub struct IncrementParent {
    #[by(Addr)]
    child: IncrementChild,
}
