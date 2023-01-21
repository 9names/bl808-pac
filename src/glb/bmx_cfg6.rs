#[doc = "Register `bmx_cfg6` reader"]
pub struct R(crate::R<BMX_CFG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg6` writer"]
pub struct W(crate::W<BMX_CFG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG6_SPEC>;
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
impl From<crate::W<BMX_CFG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_mcu_berr_addr` reader - "]
pub type STS_MCU_BERR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_addr(&self) -> STS_MCU_BERR_ADDR_R {
        STS_MCU_BERR_ADDR_R::new(self.bits)
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
#[doc = "bmx_cfg6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg6](index.html) module"]
pub struct BMX_CFG6_SPEC;
impl crate::RegisterSpec for BMX_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg6::R](R) reader structure"]
impl crate::Readable for BMX_CFG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg6::W](W) writer structure"]
impl crate::Writable for BMX_CFG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg6 to value 0"]
impl crate::Resettable for BMX_CFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
