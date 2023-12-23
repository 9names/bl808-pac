#[doc = "Register `tzc_bmx_tzmid_lock` reader"]
pub struct R(crate::R<TZC_BMX_TZMID_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BMX_TZMID_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BMX_TZMID_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BMX_TZMID_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_bmx_tzmid_lock` writer"]
pub struct W(crate::W<TZC_BMX_TZMID_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_BMX_TZMID_LOCK_SPEC>;
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
impl From<crate::W<TZC_BMX_TZMID_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_BMX_TZMID_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_pico_tzmid_lock` reader - "]
pub type TZC_PICO_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_pico_tzmid_lock` writer - "]
pub type TZC_PICO_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_mm_tzmid_lock` reader - "]
pub type TZC_MM_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_tzmid_lock` writer - "]
pub type TZC_MM_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_usb_tzmid_lock` reader - "]
pub type TZC_USB_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_usb_tzmid_lock` writer - "]
pub type TZC_USB_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_wifi_tzmid_lock` reader - "]
pub type TZC_WIFI_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_wifi_tzmid_lock` writer - "]
pub type TZC_WIFI_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_cci_tzmid_lock` reader - "]
pub type TZC_CCI_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cci_tzmid_lock` writer - "]
pub type TZC_CCI_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_sdhm_tzmid_lock` reader - "]
pub type TZC_SDHM_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sdhm_tzmid_lock` writer - "]
pub type TZC_SDHM_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_emaca_tzmid_lock` reader - "]
pub type TZC_EMACA_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_emaca_tzmid_lock` writer - "]
pub type TZC_EMACA_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_cpu_tzmid_lock` reader - "]
pub type TZC_CPU_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cpu_tzmid_lock` writer - "]
pub type TZC_CPU_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_dma_tzmid_lock` reader - "]
pub type TZC_DMA_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma_tzmid_lock` writer - "]
pub type TZC_DMA_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_dma2_tzmid_lock` reader - "]
pub type TZC_DMA2_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma2_tzmid_lock` writer - "]
pub type TZC_DMA2_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_lz4_tzmid_lock` reader - "]
pub type TZC_LZ4_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_lz4_tzmid_lock` writer - "]
pub type TZC_LZ4_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_lock(&self) -> TZC_PICO_TZMID_LOCK_R {
        TZC_PICO_TZMID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_lock(&self) -> TZC_MM_TZMID_LOCK_R {
        TZC_MM_TZMID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_lock(&self) -> TZC_USB_TZMID_LOCK_R {
        TZC_USB_TZMID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_lock(&self) -> TZC_WIFI_TZMID_LOCK_R {
        TZC_WIFI_TZMID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_lock(&self) -> TZC_CCI_TZMID_LOCK_R {
        TZC_CCI_TZMID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_lock(&self) -> TZC_SDHM_TZMID_LOCK_R {
        TZC_SDHM_TZMID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid_lock(&self) -> TZC_EMACA_TZMID_LOCK_R {
        TZC_EMACA_TZMID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_lock(&self) -> TZC_CPU_TZMID_LOCK_R {
        TZC_CPU_TZMID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_lock(&self) -> TZC_DMA_TZMID_LOCK_R {
        TZC_DMA_TZMID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_lock(&self) -> TZC_DMA2_TZMID_LOCK_R {
        TZC_DMA2_TZMID_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_lock(&self) -> TZC_LZ4_TZMID_LOCK_R {
        TZC_LZ4_TZMID_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_pico_tzmid_lock(&mut self) -> TZC_PICO_TZMID_LOCK_W<0> {
        TZC_PICO_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mm_tzmid_lock(&mut self) -> TZC_MM_TZMID_LOCK_W<1> {
        TZC_MM_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_usb_tzmid_lock(&mut self) -> TZC_USB_TZMID_LOCK_W<2> {
        TZC_USB_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_wifi_tzmid_lock(&mut self) -> TZC_WIFI_TZMID_LOCK_W<3> {
        TZC_WIFI_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cci_tzmid_lock(&mut self) -> TZC_CCI_TZMID_LOCK_W<4> {
        TZC_CCI_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sdhm_tzmid_lock(&mut self) -> TZC_SDHM_TZMID_LOCK_W<5> {
        TZC_SDHM_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_emaca_tzmid_lock(&mut self) -> TZC_EMACA_TZMID_LOCK_W<6> {
        TZC_EMACA_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_cpu_tzmid_lock(&mut self) -> TZC_CPU_TZMID_LOCK_W<7> {
        TZC_CPU_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_dma_tzmid_lock(&mut self) -> TZC_DMA_TZMID_LOCK_W<8> {
        TZC_DMA_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_dma2_tzmid_lock(&mut self) -> TZC_DMA2_TZMID_LOCK_W<9> {
        TZC_DMA2_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_lz4_tzmid_lock(&mut self) -> TZC_LZ4_TZMID_LOCK_W<10> {
        TZC_LZ4_TZMID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_bmx_tzmid_lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_bmx_tzmid_lock](index.html) module"]
pub struct TZC_BMX_TZMID_LOCK_SPEC;
impl crate::RegisterSpec for TZC_BMX_TZMID_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_bmx_tzmid_lock::R](R) reader structure"]
impl crate::Readable for TZC_BMX_TZMID_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_bmx_tzmid_lock::W](W) writer structure"]
impl crate::Writable for TZC_BMX_TZMID_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_tzmid_lock to value 0"]
impl crate::Resettable for TZC_BMX_TZMID_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
