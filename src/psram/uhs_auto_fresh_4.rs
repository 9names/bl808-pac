#[doc = "Register `uhs_auto_fresh_4` reader"]
pub struct R(crate::R<UHS_AUTO_FRESH_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_AUTO_FRESH_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_AUTO_FRESH_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_AUTO_FRESH_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_auto_fresh_4` writer"]
pub struct W(crate::W<UHS_AUTO_FRESH_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_AUTO_FRESH_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UHS_AUTO_FRESH_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_AUTO_FRESH_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bust_cycle` reader - "]
pub type REG_BUST_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_bust_cycle` writer - "]
pub type REG_BUST_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_AUTO_FRESH_4_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_7_31` reader - "]
pub type RESERVED_7_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn reg_bust_cycle(&self) -> REG_BUST_CYCLE_R {
        REG_BUST_CYCLE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn reserved_7_31(&self) -> RESERVED_7_31_R {
        RESERVED_7_31_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bust_cycle(&mut self) -> REG_BUST_CYCLE_W<0> {
        REG_BUST_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_auto_fresh_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_auto_fresh_4](index.html) module"]
pub struct UHS_AUTO_FRESH_4_SPEC;
impl crate::RegisterSpec for UHS_AUTO_FRESH_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_auto_fresh_4::R](R) reader structure"]
impl crate::Readable for UHS_AUTO_FRESH_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_auto_fresh_4::W](W) writer structure"]
impl crate::Writable for UHS_AUTO_FRESH_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_auto_fresh_4 to value 0x1d"]
impl crate::Resettable for UHS_AUTO_FRESH_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x1d;
}
