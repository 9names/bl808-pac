#[doc = "Register `psram_timeout_reg` reader"]
pub struct R(crate::R<PSRAM_TIMEOUT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_TIMEOUT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_TIMEOUT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_TIMEOUT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_timeout_reg` writer"]
pub struct W(crate::W<PSRAM_TIMEOUT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_TIMEOUT_REG_SPEC>;
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
impl From<crate::W<PSRAM_TIMEOUT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_TIMEOUT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_timeout_en` reader - "]
pub type REG_TIMEOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_timeout_en` writer - "]
pub type REG_TIMEOUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_TIMEOUT_REG_SPEC, bool, O>;
#[doc = "Field `reg_timeout_clr` reader - "]
pub type REG_TIMEOUT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_timeout_clr` writer - "]
pub type REG_TIMEOUT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_TIMEOUT_REG_SPEC, bool, O>;
#[doc = "Field `sts_timeout` reader - "]
pub type STS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_3_15` reader - "]
pub type RESERVED_3_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_timeout_cnt` reader - "]
pub type REG_TIMEOUT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_timeout_cnt` writer - "]
pub type REG_TIMEOUT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_TIMEOUT_REG_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_timeout_en(&self) -> REG_TIMEOUT_EN_R {
        REG_TIMEOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_clr(&self) -> REG_TIMEOUT_CLR_R {
        REG_TIMEOUT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_timeout(&self) -> STS_TIMEOUT_R {
        STS_TIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn reserved_3_15(&self) -> RESERVED_3_15_R {
        RESERVED_3_15_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_timeout_cnt(&self) -> REG_TIMEOUT_CNT_R {
        REG_TIMEOUT_CNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timeout_en(&mut self) -> REG_TIMEOUT_EN_W<0> {
        REG_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timeout_clr(&mut self) -> REG_TIMEOUT_CLR_W<1> {
        REG_TIMEOUT_CLR_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timeout_cnt(&mut self) -> REG_TIMEOUT_CNT_W<16> {
        REG_TIMEOUT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_timeout_reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_timeout_reg](index.html) module"]
pub struct PSRAM_TIMEOUT_REG_SPEC;
impl crate::RegisterSpec for PSRAM_TIMEOUT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_timeout_reg::R](R) reader structure"]
impl crate::Readable for PSRAM_TIMEOUT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_timeout_reg::W](W) writer structure"]
impl crate::Writable for PSRAM_TIMEOUT_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_timeout_reg to value 0x0100_0000"]
impl crate::Resettable for PSRAM_TIMEOUT_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
