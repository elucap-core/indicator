//! Abstractions for stream aggregation, which we call "Indicator" s.
//!
//! This crate provides abstractions of different levels to build indicators:
//!
//! - Indicators are the combinations of [`Operator`]s. We can chain them by [`OperatorExt::then`]
//! and apply them to the same input "simultaneously" by [`OperatorExt::facet`].
//!
//! - To handle [Time series] better, we introduced [`TickedOperatorExt`] operators,
//! of which the input and output are both [`Tickable`] (the items in time series)
//! and sharing the same [`Tick`]. We provide many ticked version utils. For example,
//! we can apply different [`TickedOperatorExt`]s (using [`TickedOperatorExt::facet_t`] or [`ticked::FacetMap`])
//! to the same [`Tickable`] stream to get a "synced" result stream of the combined outputs of
//! those operators sharing the same [`Tick`] of theirs input.
//! Each item of a "synced" stream will have the form of `TickValue<(O1, O2)>`.
//!
//! - [`TumblingOperator`]s are defined by some operations on events of non-overlapping time windows ([`TumblingWindow`]).
//! [Moving average] is one of the famous examples, which is defined by the average of the numbers
//! from those events occur in the same tumbling window. We call those operations [`TumblingOperation`]s.
//!
//! - [`TumblingOperation`]s can also be chained together or be applied "simultaneously" to the same input
//! by using [`TumblingOperation::then`] or [`TumblingOperation::facet`]. They are considered to be
//! sharing the same [`TumblingWindow`] in the result operator.
//!
//! - Finally, we can apply the indicators to [`Iterator`]s or [`Stream`](futures::stream::Stream)s
//! by using [`IndicatorIteratorExt::indicator`] or [`IndicatorStreamExt::indicator`] accordingly.
//!
//!
//! [Time series]: https://en.wikipedia.org/wiki/Time_series
//! [Moving average]: https://en.wikipedia.org/wiki/Moving_average

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Operator.
pub mod operator;

/// Time window.
pub mod window;

/// Ticked operators.
pub mod ticked;

pub use operator::{IntoOperator, Operator, OperatorExt};
pub use ticked::{
    tumbling::{QueueCapAtLeast, TumblingOperation, TumblingOperator, TumblingQueue},
    TickedOperatorExt,
};
pub use window::{Period, Tick, TickValue, Tickable, TumblingWindow};