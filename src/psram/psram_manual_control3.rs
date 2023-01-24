#[doc = "Register `psram_manual_control3` reader"]
pub struct R(crate::R<PSRAM_MANUAL_CONTROL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_MANUAL_CONTROL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_MANUAL_CONTROL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_MANUAL_CONTROL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_manual_control3` writer"]
pub struct W(crate::W<PSRAM_MANUAL_CONTROL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_MANUAL_CONTROL3_SPEC>;
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
impl From<crate::W<PSRAM_MANUAL_CONTROL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_MANUAL_CONTROL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_adq_rel_val` reader - "]
pub type REG_ADQ_REL_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_adq_rel_val` writer - "]
pub type REG_ADQ_REL_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_MANUAL_CONTROL3_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_wrap2incr_en` reader - "]
pub type REG_WRAP2INCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_wrap2incr_en` writer - "]
pub type REG_WRAP2INCR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_MANUAL_CONTROL3_SPEC, bool, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn reg_adq_rel_val(&self) -> REG_ADQ_REL_VAL_R {
        REG_ADQ_REL_VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&self) -> REG_WRAP2INCR_EN_R {
        REG_WRAP2INCR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_adq_rel_val(&mut self) -> REG_ADQ_REL_VAL_W<0> {
        REG_ADQ_REL_VAL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wrap2incr_en(&mut self) -> REG_WRAP2INCR_EN_W<8> {
        REG_WRAP2INCR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_manual_control3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_manual_control3](index.html) module"]
pub struct PSRAM_MANUAL_CONTROL3_SPEC;
impl crate::RegisterSpec for PSRAM_MANUAL_CONTROL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_manual_control3::R](R) reader structure"]
impl crate::Readable for PSRAM_MANUAL_CONTROL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_manual_control3::W](W) writer structure"]
impl crate::Writable for PSRAM_MANUAL_CONTROL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_manual_control3 to value 0x0120"]
impl crate::Resettable for PSRAM_MANUAL_CONTROL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0120;
}
