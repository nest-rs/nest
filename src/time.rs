use std::time::Duration;

/// Trait for things which can be converted into seconds. This should not be used for high-precision accuracy.
pub trait ToSeconds: Sized {
    /// Convert into seconds.
    fn to_secs(self) -> f32;

    /// Convert into miliseconds.
    fn to_msecs(self) -> f32 {
        self.to_secs() * 1000.0
    }

    /// Convert into microseconds.
    fn to_usecs(self) -> f32 {
        self.to_secs() * 1000000.0
    }

    /// Convert into minutes.
    fn to_mins(self) -> f32 {
        self.to_secs() / 60.0
    }

    /// Convert into hours.
    fn to_hours(self) -> f32 {
        self.to_secs() / 60.0 / 60.0
    }

    /// Convert into days.
    fn to_days(self) -> f32 {
        self.to_secs() / 60.0 / 60.0 / 24.0
    }

    /// Convert into years.
    fn to_years(self) -> f32 {
        self.to_secs() * 3.16875358e-8
    }
}

impl ToSeconds for Duration {
    fn to_secs(self) -> f32 {
        let secs = self.as_secs() as f32;
        let subsec_nanos = self.subsec_nanos() as f32;
        secs + subsec_nanos * 1e-9
    }
}
