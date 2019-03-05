pub mod version_0_1_0;

pub trait Migration: Sized {
  const FROM_VERSION: &'static str;
  const TO_VERSION: &'static str;

  type To: From<Self>;
}
