#[doc = "Register `soc_info0` reader"]
pub struct R(crate::R<SOC_INFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_INFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_INFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_INFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `soc_info0` writer"]
pub struct W(crate::W<SOC_INFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_INFO0_SPEC>;
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
impl From<crate::W<SOC_INFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_INFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_26` reader - "]
pub type RESERVED_0_26_R = crate::FieldReader<u32, u32>;
#[doc = "Field `chip_rdy` reader - "]
pub type CHIP_RDY_R = crate::BitReader<bool>;
#[doc = "Field `glb_id` reader - "]
pub type GLB_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn reserved_0_26(&self) -> RESERVED_0_26_R {
        RESERVED_0_26_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> CHIP_RDY_R {
        CHIP_RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GLB_ID_R {
        GLB_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
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
#[doc = "soc_info0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_info0](index.html) module"]
pub struct SOC_INFO0_SPEC;
impl crate::RegisterSpec for SOC_INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_info0::R](R) reader structure"]
impl crate::Readable for SOC_INFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_info0::W](W) writer structure"]
impl crate::Writable for SOC_INFO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets soc_info0 to value 0x6000_0000"]
impl crate::Resettable for SOC_INFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0000;
}
