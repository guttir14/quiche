// Copyright (C) 2025, Cloudflare, Inc.
// All rights reserved.
//
// SPDX-License-Identifier: BSD-2-Clause

//! Compatibility layer for optional foundations-based telemetry.
//!
//! When the `telemetry` feature is enabled this re-exports foundations
//! structured logging, Prometheus metrics and the telemetry context. When it
//! is disabled it provides zero-cost no-op shims so the crate builds without
//! the `foundations` dependency tree.

// ===================== telemetry ENABLED =====================
#[cfg(feature = "telemetry")]
pub use foundations::telemetry::log;

#[cfg(feature = "telemetry")]
pub use foundations::telemetry::TelemetryContext;

#[cfg(feature = "telemetry")]
pub mod metrics {
    pub use foundations::telemetry::metrics::metrics;
    pub use foundations::telemetry::metrics::Counter;
    pub use foundations::telemetry::metrics::Gauge;
    pub use foundations::telemetry::metrics::Histogram;
    pub use foundations::telemetry::metrics::HistogramBuilder;
    pub use foundations::telemetry::metrics::TimeHistogram;
}

// ===================== telemetry DISABLED ====================
#[cfg(not(feature = "telemetry"))]
pub mod log {
    //! No-op replacements for the foundations structured logging macros.
    //! Any arguments are swallowed without evaluation.
    macro_rules! __noop_log {
        ($($tt:tt)*) => {{}};
    }
    pub(crate) use __noop_log as error;
    pub(crate) use __noop_log as warn;
    pub(crate) use __noop_log as info;
    pub(crate) use __noop_log as debug;
    pub(crate) use __noop_log as trace;
}

#[cfg(not(feature = "telemetry"))]
pub mod metrics {
    //! Zero-sized no-op metric handles mirroring the foundations API surface
    //! used by this crate.

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Counter;
    impl Counter {
        #[inline]
        pub fn inc(&self) {}
        #[inline]
        pub fn inc_by<T>(&self, _v: T) {}
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Gauge;
    impl Gauge {
        #[inline]
        pub fn inc(&self) {}
        #[inline]
        pub fn dec(&self) {}
        #[inline]
        pub fn inc_by<T>(&self, _v: T) {}
        #[inline]
        pub fn dec_by<T>(&self, _v: T) {}
        #[inline]
        pub fn set<T>(&self, _v: T) {}
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Histogram;
    impl Histogram {
        #[inline]
        pub fn observe<T>(&self, _v: T) {}
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct TimeHistogram;
    impl TimeHistogram {
        #[inline]
        pub fn observe<T>(&self, _v: T) {}
        #[inline]
        pub fn start_timer(&self) -> Timer {
            Timer
        }
    }

    /// RAII timer guard returned by [`TimeHistogram::start_timer`]; a no-op.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Timer;
}

#[cfg(not(feature = "telemetry"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct TelemetryContext;

#[cfg(not(feature = "telemetry"))]
impl TelemetryContext {
    #[inline]
    pub fn current() -> Self {
        TelemetryContext
    }

    #[inline]
    pub fn apply<F>(&self, future: F) -> F {
        future
    }
}