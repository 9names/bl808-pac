#[doc = "Register `pdm_cfg0` reader"]
pub struct R(crate::R<PDM_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pdm_cfg0` writer"]
pub struct W(crate::W<PDM_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CFG0_SPEC>;
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
impl From<crate::W<PDM_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pdm_io_sel` reader - "]
pub type REG_PDM_IO_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_pdm_io_sel` writer - "]
pub type REG_PDM_IO_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDM_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_1_31` reader - "]
pub type RESERVED_1_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pdm_io_sel(&self) -> REG_PDM_IO_SEL_R {
        REG_PDM_IO_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved_1_31(&self) -> RESERVED_1_31_R {
        RESERVED_1_31_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pdm_io_sel(&mut self) -> REG_PDM_IO_SEL_W<0> {
        REG_PDM_IO_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pdm_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_cfg0](index.html) module"]
pub struct PDM_CFG0_SPEC;
impl crate::RegisterSpec for PDM_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_cfg0::R](R) reader structure"]
impl crate::Readable for PDM_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_cfg0::W](W) writer structure"]
impl crate::Writable for PDM_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pdm_cfg0 to value 0"]
impl crate::Resettable for PDM_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
