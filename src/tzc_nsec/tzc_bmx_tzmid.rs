#[doc = "Register `tzc_bmx_tzmid` reader"]
pub struct R(crate::R<TZC_BMX_TZMID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BMX_TZMID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BMX_TZMID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BMX_TZMID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_bmx_tzmid` writer"]
pub struct W(crate::W<TZC_BMX_TZMID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_BMX_TZMID_SPEC>;
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
impl From<crate::W<TZC_BMX_TZMID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_BMX_TZMID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_pico_tzmid` reader - "]
pub type TZC_PICO_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_tzmid` reader - "]
pub type TZC_MM_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_usb_tzmid` reader - "]
pub type TZC_USB_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_wifi_tzmid` reader - "]
pub type TZC_WIFI_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cci_tzmid` reader - "]
pub type TZC_CCI_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sdhm_tzmid` reader - "]
pub type TZC_SDHM_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_emaca_tzmid` reader - "]
pub type TZC_EMACA_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cpu_tzmid` reader - "]
pub type TZC_CPU_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma_tzmid` reader - "]
pub type TZC_DMA_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma2_tzmid` reader - "]
pub type TZC_DMA2_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_lz4_tzmid` reader - "]
pub type TZC_LZ4_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `reserved_11_15` reader - "]
pub type RESERVED_11_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_pico_tzmid_sel` reader - "]
pub type TZC_PICO_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mm_tzmid_sel` reader - "]
pub type TZC_MM_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_usb_tzmid_sel` reader - "]
pub type TZC_USB_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_wifi_tzmid_sel` reader - "]
pub type TZC_WIFI_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cci_tzmid_sel` reader - "]
pub type TZC_CCI_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sdhm_tzmid_sel` reader - "]
pub type TZC_SDHM_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_emaca_tzmid_sel` reader - "]
pub type TZC_EMACA_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_cpu_tzmid_sel` reader - "]
pub type TZC_CPU_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma_tzmid_sel` reader - "]
pub type TZC_DMA_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_dma2_tzmid_sel` reader - "]
pub type TZC_DMA2_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_lz4_tzmid_sel` reader - "]
pub type TZC_LZ4_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reserved_27_31` reader - "]
pub type RESERVED_27_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_pico_tzmid(&self) -> TZC_PICO_TZMID_R {
        TZC_PICO_TZMID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_tzmid(&self) -> TZC_MM_TZMID_R {
        TZC_MM_TZMID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_usb_tzmid(&self) -> TZC_USB_TZMID_R {
        TZC_USB_TZMID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid(&self) -> TZC_WIFI_TZMID_R {
        TZC_WIFI_TZMID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_cci_tzmid(&self) -> TZC_CCI_TZMID_R {
        TZC_CCI_TZMID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid(&self) -> TZC_SDHM_TZMID_R {
        TZC_SDHM_TZMID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid(&self) -> TZC_EMACA_TZMID_R {
        TZC_EMACA_TZMID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid(&self) -> TZC_CPU_TZMID_R {
        TZC_CPU_TZMID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_dma_tzmid(&self) -> TZC_DMA_TZMID_R {
        TZC_DMA_TZMID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid(&self) -> TZC_DMA2_TZMID_R {
        TZC_DMA2_TZMID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid(&self) -> TZC_LZ4_TZMID_R {
        TZC_LZ4_TZMID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn reserved_11_15(&self) -> RESERVED_11_15_R {
        RESERVED_11_15_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_sel(&self) -> TZC_PICO_TZMID_SEL_R {
        TZC_PICO_TZMID_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_sel(&self) -> TZC_MM_TZMID_SEL_R {
        TZC_MM_TZMID_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_sel(&self) -> TZC_USB_TZMID_SEL_R {
        TZC_USB_TZMID_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_sel(&self) -> TZC_WIFI_TZMID_SEL_R {
        TZC_WIFI_TZMID_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_sel(&self) -> TZC_CCI_TZMID_SEL_R {
        TZC_CCI_TZMID_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_sel(&self) -> TZC_SDHM_TZMID_SEL_R {
        TZC_SDHM_TZMID_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid_sel(&self) -> TZC_EMACA_TZMID_SEL_R {
        TZC_EMACA_TZMID_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_sel(&self) -> TZC_CPU_TZMID_SEL_R {
        TZC_CPU_TZMID_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_sel(&self) -> TZC_DMA_TZMID_SEL_R {
        TZC_DMA_TZMID_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_sel(&self) -> TZC_DMA2_TZMID_SEL_R {
        TZC_DMA2_TZMID_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_sel(&self) -> TZC_LZ4_TZMID_SEL_R {
        TZC_LZ4_TZMID_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn reserved_27_31(&self) -> RESERVED_27_31_R {
        RESERVED_27_31_R::new(((self.bits >> 27) & 0x1f) as u8)
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
#[doc = "tzc_bmx_tzmid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_bmx_tzmid](index.html) module"]
pub struct TZC_BMX_TZMID_SPEC;
impl crate::RegisterSpec for TZC_BMX_TZMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_bmx_tzmid::R](R) reader structure"]
impl crate::Readable for TZC_BMX_TZMID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_bmx_tzmid::W](W) writer structure"]
impl crate::Writable for TZC_BMX_TZMID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_tzmid to value 0x07ff_0000"]
impl crate::Resettable for TZC_BMX_TZMID_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff_0000;
}
