#[doc = "Register `cgen_cfg0` reader"]
pub struct R(crate::R<CGEN_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg0` writer"]
pub struct W(crate::W<CGEN_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG0_SPEC>;
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
impl From<crate::W<CGEN_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_m_cpu` reader - "]
pub type CGEN_M_CPU_R = crate::BitReader<bool>;
#[doc = "Field `cgen_m_cpu` writer - "]
pub type CGEN_M_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG0_SPEC, bool, O>;
#[doc = "Field `cgen_m_sdu` reader - "]
pub type CGEN_M_SDU_R = crate::BitReader<bool>;
#[doc = "Field `cgen_m_sdu` writer - "]
pub type CGEN_M_SDU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG0_SPEC, bool, O>;
#[doc = "Field `cgen_m_sec` reader - "]
pub type CGEN_M_SEC_R = crate::BitReader<bool>;
#[doc = "Field `cgen_m_sec` writer - "]
pub type CGEN_M_SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG0_SPEC, bool, O>;
#[doc = "Field `cgen_m_dma` reader - "]
pub type CGEN_M_DMA_R = crate::BitReader<bool>;
#[doc = "Field `cgen_m_dma` writer - "]
pub type CGEN_M_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG0_SPEC, bool, O>;
#[doc = "Field `cgen_m_cci` reader - "]
pub type CGEN_M_CCI_R = crate::BitReader<bool>;
#[doc = "Field `cgen_m_cci` writer - "]
pub type CGEN_M_CCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_5_31` reader - "]
pub type RESERVED_5_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_m_cpu(&self) -> CGEN_M_CPU_R {
        CGEN_M_CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_m_sdu(&self) -> CGEN_M_SDU_R {
        CGEN_M_SDU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_m_sec(&self) -> CGEN_M_SEC_R {
        CGEN_M_SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_m_dma(&self) -> CGEN_M_DMA_R {
        CGEN_M_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_m_cci(&self) -> CGEN_M_CCI_R {
        CGEN_M_CCI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn reserved_5_31(&self) -> RESERVED_5_31_R {
        RESERVED_5_31_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_cpu(&mut self) -> CGEN_M_CPU_W<0> {
        CGEN_M_CPU_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_sdu(&mut self) -> CGEN_M_SDU_W<1> {
        CGEN_M_SDU_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_sec(&mut self) -> CGEN_M_SEC_W<2> {
        CGEN_M_SEC_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_dma(&mut self) -> CGEN_M_DMA_W<3> {
        CGEN_M_DMA_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_cci(&mut self) -> CGEN_M_CCI_W<4> {
        CGEN_M_CCI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_m\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg0](index.html) module"]
pub struct CGEN_CFG0_SPEC;
impl crate::RegisterSpec for CGEN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg0::R](R) reader structure"]
impl crate::Readable for CGEN_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg0::W](W) writer structure"]
impl crate::Writable for CGEN_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg0 to value 0x1f"]
impl crate::Resettable for CGEN_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
