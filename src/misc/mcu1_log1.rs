#[doc = "Register `mcu1_log1` reader"]
pub struct R(crate::R<MCU1_LOG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU1_LOG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU1_LOG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU1_LOG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu1_log1` writer"]
pub struct W(crate::W<MCU1_LOG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU1_LOG1_SPEC>;
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
impl From<crate::W<MCU1_LOG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU1_LOG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_mcu1_mcause` reader - "]
pub type STS_MCU1_MCAUSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_mcu1_mcause(&self) -> STS_MCU1_MCAUSE_R {
        STS_MCU1_MCAUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu1_log1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu1_log1](index.html) module"]
pub struct MCU1_LOG1_SPEC;
impl crate::RegisterSpec for MCU1_LOG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu1_log1::R](R) reader structure"]
impl crate::Readable for MCU1_LOG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu1_log1::W](W) writer structure"]
impl crate::Writable for MCU1_LOG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log1 to value 0"]
impl crate::Resettable for MCU1_LOG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
