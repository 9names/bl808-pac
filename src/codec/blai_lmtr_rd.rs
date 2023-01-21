#[doc = "Register `blai_lmtr_rd` reader"]
pub struct R(crate::R<BLAI_LMTR_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLAI_LMTR_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLAI_LMTR_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLAI_LMTR_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `blai_lmtr_rd` writer"]
pub struct W(crate::W<BLAI_LMTR_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLAI_LMTR_RD_SPEC>;
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
impl From<crate::W<BLAI_LMTR_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLAI_LMTR_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_blai_rcmd_cnt` reader - "]
pub type REG_BLAI_RCMD_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_blai_rcmd_cnt` writer - "]
pub type REG_BLAI_RCMD_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLAI_LMTR_RD_SPEC, u16, u16, 16, O>;
#[doc = "Field `reserved_16_30` reader - "]
pub type RESERVED_16_30_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_blai_rcmd_mode` reader - "]
pub type REG_BLAI_RCMD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_blai_rcmd_mode` writer - "]
pub type REG_BLAI_RCMD_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BLAI_LMTR_RD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_blai_rcmd_cnt(&self) -> REG_BLAI_RCMD_CNT_R {
        REG_BLAI_RCMD_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn reserved_16_30(&self) -> RESERVED_16_30_R {
        RESERVED_16_30_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_blai_rcmd_mode(&self) -> REG_BLAI_RCMD_MODE_R {
        REG_BLAI_RCMD_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_blai_rcmd_cnt(&mut self) -> REG_BLAI_RCMD_CNT_W<0> {
        REG_BLAI_RCMD_CNT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_blai_rcmd_mode(&mut self) -> REG_BLAI_RCMD_MODE_W<31> {
        REG_BLAI_RCMD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "codec_bus_dec_err\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blai_lmtr_rd](index.html) module"]
pub struct BLAI_LMTR_RD_SPEC;
impl crate::RegisterSpec for BLAI_LMTR_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blai_lmtr_rd::R](R) reader structure"]
impl crate::Readable for BLAI_LMTR_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blai_lmtr_rd::W](W) writer structure"]
impl crate::Writable for BLAI_LMTR_RD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets blai_lmtr_rd to value 0"]
impl crate::Resettable for BLAI_LMTR_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
