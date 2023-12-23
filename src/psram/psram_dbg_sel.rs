#[doc = "Register `psram_dbg_sel` reader"]
pub struct R(crate::R<PSRAM_DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_dbg_sel` writer"]
pub struct W(crate::W<PSRAM_DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_DBG_SEL_SPEC>;
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
impl From<crate::W<PSRAM_DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_psram_dbg_en` reader - "]
pub type REG_PSRAM_DBG_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_psram_dbg_en` writer - "]
pub type REG_PSRAM_DBG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_DBG_SEL_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_psram_dbg_sel` reader - "]
pub type REG_PSRAM_DBG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_psram_dbg_sel` writer - "]
pub type REG_PSRAM_DBG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_DBG_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_8_31` reader - "]
pub type RESERVED_8_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_psram_dbg_en(&self) -> REG_PSRAM_DBG_EN_R {
        REG_PSRAM_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_psram_dbg_sel(&self) -> REG_PSRAM_DBG_SEL_R {
        REG_PSRAM_DBG_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved_8_31(&self) -> RESERVED_8_31_R {
        RESERVED_8_31_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_dbg_en(&mut self) -> REG_PSRAM_DBG_EN_W<0> {
        REG_PSRAM_DBG_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_dbg_sel(&mut self) -> REG_PSRAM_DBG_SEL_W<4> {
        REG_PSRAM_DBG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_dbg_sel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_dbg_sel](index.html) module"]
pub struct PSRAM_DBG_SEL_SPEC;
impl crate::RegisterSpec for PSRAM_DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_dbg_sel::R](R) reader structure"]
impl crate::Readable for PSRAM_DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_dbg_sel::W](W) writer structure"]
impl crate::Writable for PSRAM_DBG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_dbg_sel to value 0"]
impl crate::Resettable for PSRAM_DBG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
