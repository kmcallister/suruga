use chrono::{DateTime, UTC};

// RFC 5280 6. Certification Path Validation

use super::CertResult;
use super::CertErrorKind::{InvalidField, InvalidPeriod};
use super::version::Version;
use super::cert::Certificate;

pub struct ValidationContext {
    pub current_time: DateTime<UTC>,
}

impl ValidationContext {
    pub fn check_cert(&self, c: &Certificate) -> CertResult<()> {
        try!(self.check_field_constraints(c));
        try!(self.check_validity_time(c));

        Ok(())
    }
}

impl ValidationContext {
    fn check_validity_time(&self, c: &Certificate) -> CertResult<()> {
        let v = &c.cert.validity;
        if self.current_time >= v.not_after.time {
            return cert_err!(InvalidPeriod, "certificate expired: {}", v.not_after);
        }
        if v.not_before.time >= self.current_time {
            return cert_err!(InvalidPeriod, "certificate not valid yet: {}", v.not_before);
        }

        Ok(())
    }
}
