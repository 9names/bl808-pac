#[doc = "Register `winbond_psram_configure2` reader"]
pub struct R(crate::R<WINBOND_PSRAM_CONFIGURE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINBOND_PSRAM_CONFIGURE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINBOND_PSRAM_CONFIGURE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINBOND_PSRAM_CONFIGURE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `winbond_psram_configure2` writer"]
pub struct W(crate::W<WINBOND_PSRAM_CONFIGURE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINBOND_PSRAM_CONFIGURE2_SPEC>;
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
impl From<crate::W<WINBOND_PSRAM_CONFIGURE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINBOND_PSRAM_CONFIGURE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_wb_zq_code` reader - "]
pub type REG_WB_ZQ_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_zq_code` writer - "]
pub type REG_WB_ZQ_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINBOND_PSRAM_CONFIGURE2_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_4_31` reader - "]
pub type RESERVED_4_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_wb_zq_code(&self) -> REG_WB_ZQ_CODE_R {
        REG_WB_ZQ_CODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn reserved_4_31(&self) -> RESERVED_4_31_R {
        RESERVED_4_31_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_zq_code(&mut self) -> REG_WB_ZQ_CODE_W<0> {
        REG_WB_ZQ_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "winbond_psram_configure2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winbond_psram_configure2](index.html) module"]
pub struct WINBOND_PSRAM_CONFIGURE2_SPEC;
impl crate::RegisterSpec for WINBOND_PSRAM_CONFIGURE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winbond_psram_configure2::R](R) reader structure"]
impl crate::Readable for WINBOND_PSRAM_CONFIGURE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winbond_psram_configure2::W](W) writer structure"]
impl crate::Writable for WINBOND_PSRAM_CONFIGURE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets winbond_psram_configure2 to value 0"]
impl crate::Resettable for WINBOND_PSRAM_CONFIGURE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
