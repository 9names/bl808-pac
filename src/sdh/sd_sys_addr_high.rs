#[doc = "Register `sd_sys_addr_high` reader"]
pub struct R(crate::R<SD_SYS_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SYS_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SYS_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SYS_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_sys_addr_high` writer"]
pub struct W(crate::W<SD_SYS_ADDR_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SYS_ADDR_HIGH_SPEC>;
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
impl From<crate::W<SD_SYS_ADDR_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SYS_ADDR_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_addr_h` reader - "]
pub type DMA_ADDR_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dma_addr_h` writer - "]
pub type DMA_ADDR_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_SYS_ADDR_HIGH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_h(&self) -> DMA_ADDR_H_R {
        DMA_ADDR_H_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_addr_h(&mut self) -> DMA_ADDR_H_W<0> {
        DMA_ADDR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_sys_addr_high](index.html) module"]
pub struct SD_SYS_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for SD_SYS_ADDR_HIGH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_sys_addr_high::R](R) reader structure"]
impl crate::Readable for SD_SYS_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_sys_addr_high::W](W) writer structure"]
impl crate::Writable for SD_SYS_ADDR_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_sys_addr_high to value 0"]
impl crate::Resettable for SD_SYS_ADDR_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
