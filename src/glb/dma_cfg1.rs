#[doc = "Register `dma_cfg1` reader"]
pub struct R(crate::R<DMA_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dma_cfg1` writer"]
pub struct W(crate::W<DMA_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CFG1_SPEC>;
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
impl From<crate::W<DMA_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_23` reader - "]
pub type RESERVED_0_23_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dma2_clk_en` reader - "]
pub type DMA2_CLK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma2_clk_en` writer - "]
pub type DMA2_CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_CFG1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn reserved_0_23(&self) -> RESERVED_0_23_R {
        RESERVED_0_23_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma2_clk_en(&self) -> DMA2_CLK_EN_R {
        DMA2_CLK_EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_clk_en(&mut self) -> DMA2_CLK_EN_W<24> {
        DMA2_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dma_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cfg1](index.html) module"]
pub struct DMA_CFG1_SPEC;
impl crate::RegisterSpec for DMA_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_cfg1::R](R) reader structure"]
impl crate::Readable for DMA_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_cfg1::W](W) writer structure"]
impl crate::Writable for DMA_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_cfg1 to value 0xff00_0000"]
impl crate::Resettable for DMA_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_0000;
}
