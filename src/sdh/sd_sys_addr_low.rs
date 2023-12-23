#[doc = "Register `sd_sys_addr_low` reader"]
pub struct R(crate::R<SD_SYS_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SYS_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SYS_ADDR_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SYS_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_sys_addr_low` writer"]
pub struct W(crate::W<SD_SYS_ADDR_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SYS_ADDR_LOW_SPEC>;
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
impl From<crate::W<SD_SYS_ADDR_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SYS_ADDR_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_addr_l` reader - "]
pub type DMA_ADDR_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dma_addr_l` writer - "]
pub type DMA_ADDR_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_SYS_ADDR_LOW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_l(&self) -> DMA_ADDR_L_R {
        DMA_ADDR_L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_addr_l(&mut self) -> DMA_ADDR_L_W<0> {
        DMA_ADDR_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Address Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_sys_addr_low](index.html) module"]
pub struct SD_SYS_ADDR_LOW_SPEC;
impl crate::RegisterSpec for SD_SYS_ADDR_LOW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_sys_addr_low::R](R) reader structure"]
impl crate::Readable for SD_SYS_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_sys_addr_low::W](W) writer structure"]
impl crate::Writable for SD_SYS_ADDR_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_sys_addr_low to value 0"]
impl crate::Resettable for SD_SYS_ADDR_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}