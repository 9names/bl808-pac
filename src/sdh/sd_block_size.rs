#[doc = "Register `sd_block_size` reader"]
pub struct R(crate::R<SD_BLOCK_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_BLOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_BLOCK_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_BLOCK_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_block_size` writer"]
pub struct W(crate::W<SD_BLOCK_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_BLOCK_SIZE_SPEC>;
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
impl From<crate::W<SD_BLOCK_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_BLOCK_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `block_size` reader - "]
pub type BLOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `block_size` writer - "]
pub type BLOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_BLOCK_SIZE_SPEC, u16, u16, 12, O>;
#[doc = "Field `host_dma_bdry` reader - "]
pub type HOST_DMA_BDRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `host_dma_bdry` writer - "]
pub type HOST_DMA_BDRY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_BLOCK_SIZE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn host_dma_bdry(&self) -> HOST_DMA_BDRY_R {
        HOST_DMA_BDRY_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<0> {
        BLOCK_SIZE_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn host_dma_bdry(&mut self) -> HOST_DMA_BDRY_W<12> {
        HOST_DMA_BDRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_block_size](index.html) module"]
pub struct SD_BLOCK_SIZE_SPEC;
impl crate::RegisterSpec for SD_BLOCK_SIZE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_block_size::R](R) reader structure"]
impl crate::Readable for SD_BLOCK_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_block_size::W](W) writer structure"]
impl crate::Writable for SD_BLOCK_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_block_size to value 0"]
impl crate::Resettable for SD_BLOCK_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
