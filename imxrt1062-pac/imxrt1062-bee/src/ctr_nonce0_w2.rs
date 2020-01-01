#[doc = "Writer for register CTR_NONCE0_W2"]
pub type W = crate::W<u32, super::CTR_NONCE0_W2>;
#[doc = "Register CTR_NONCE0_W2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR_NONCE0_W2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NONCE02`"]
pub struct NONCE02_W<'a> {
    w: &'a mut W,
}
impl<'a> NONCE02_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Nonce0 from software for CTR, for region0. Nonce0={Nonce03,Nonce02,Nonce01,Nonce00}"]
    #[inline(always)]
    pub fn nonce02(&mut self) -> NONCE02_W {
        NONCE02_W { w: self }
    }
}
