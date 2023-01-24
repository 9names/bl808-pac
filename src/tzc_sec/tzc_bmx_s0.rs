#[doc = "Register `tzc_bmx_s0` reader"]
pub struct R(crate::R<TZC_BMX_S0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BMX_S0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BMX_S0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BMX_S0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_bmx_s0` writer"]
pub struct W(crate::W<TZC_BMX_S0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_BMX_S0_SPEC>;
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
impl From<crate::W<TZC_BMX_S0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_BMX_S0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_bmx_mm_tzsid_en` reader - "]
pub type TZC_BMX_MM_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_mm_tzsid_en` writer - "]
pub type TZC_BMX_MM_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_bmx_dma_tzsid_en` reader - "]
pub type TZC_BMX_DMA_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_dma_tzsid_en` writer - "]
pub type TZC_BMX_DMA_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_bmx_dma2_tzsid_en` reader - "]
pub type TZC_BMX_DMA2_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_dma2_tzsid_en` writer - "]
pub type TZC_BMX_DMA2_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` reader - "]
pub type TZC_BMX_PWR_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` writer - "]
pub type TZC_BMX_PWR_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` reader - "]
pub type TZC_BMX_SDH_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` writer - "]
pub type TZC_BMX_SDH_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_bmx_emac_tzsid_en` reader - "]
pub type TZC_BMX_EMAC_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_emac_tzsid_en` writer - "]
pub type TZC_BMX_EMAC_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_BMX_S0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_bmx_mm_tzsid_lock` reader - "]
pub type TZC_BMX_MM_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_mm_tzsid_lock` writer - "]
pub type TZC_BMX_MM_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S0_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` reader - "]
pub type TZC_BMX_DMA_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` writer - "]
pub type TZC_BMX_DMA_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_en(&self) -> TZC_BMX_MM_TZSID_EN_R {
        TZC_BMX_MM_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_en(&self) -> TZC_BMX_DMA_TZSID_EN_R {
        TZC_BMX_DMA_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_en(&self) -> TZC_BMX_DMA2_TZSID_EN_R {
        TZC_BMX_DMA2_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_en(&self) -> TZC_BMX_PWR_TZSID_EN_R {
        TZC_BMX_PWR_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_en(&self) -> TZC_BMX_SDH_TZSID_EN_R {
        TZC_BMX_SDH_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_en(&self) -> TZC_BMX_EMAC_TZSID_EN_R {
        TZC_BMX_EMAC_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_lock(&self) -> TZC_BMX_MM_TZSID_LOCK_R {
        TZC_BMX_MM_TZSID_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_lock(&self) -> TZC_BMX_DMA_TZSID_LOCK_R {
        TZC_BMX_DMA_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_mm_tzsid_en(&mut self) -> TZC_BMX_MM_TZSID_EN_W<0> {
        TZC_BMX_MM_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_dma_tzsid_en(&mut self) -> TZC_BMX_DMA_TZSID_EN_W<2> {
        TZC_BMX_DMA_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_dma2_tzsid_en(&mut self) -> TZC_BMX_DMA2_TZSID_EN_W<4> {
        TZC_BMX_DMA2_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_pwr_tzsid_en(&mut self) -> TZC_BMX_PWR_TZSID_EN_W<6> {
        TZC_BMX_PWR_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_sdh_tzsid_en(&mut self) -> TZC_BMX_SDH_TZSID_EN_W<8> {
        TZC_BMX_SDH_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_emac_tzsid_en(&mut self) -> TZC_BMX_EMAC_TZSID_EN_W<10> {
        TZC_BMX_EMAC_TZSID_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_mm_tzsid_lock(&mut self) -> TZC_BMX_MM_TZSID_LOCK_W<16> {
        TZC_BMX_MM_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_dma_tzsid_lock(&mut self) -> TZC_BMX_DMA_TZSID_LOCK_W<17> {
        TZC_BMX_DMA_TZSID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_bmx_s0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_bmx_s0](index.html) module"]
pub struct TZC_BMX_S0_SPEC;
impl crate::RegisterSpec for TZC_BMX_S0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_bmx_s0::R](R) reader structure"]
impl crate::Readable for TZC_BMX_S0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_bmx_s0::W](W) writer structure"]
impl crate::Writable for TZC_BMX_S0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_s0 to value 0x0fff"]
impl crate::Resettable for TZC_BMX_S0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}