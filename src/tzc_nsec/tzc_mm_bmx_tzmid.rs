#[doc = "Register `tzc_mm_bmx_tzmid` reader"]
pub struct R(crate::R<TZC_MM_BMX_TZMID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_TZMID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_TZMID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_TZMID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_tzmid` writer"]
pub struct W(crate::W<TZC_MM_BMX_TZMID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_TZMID_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_TZMID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_TZMID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mmcpu_tzmid` reader - "]
pub type TZC_MMCPU_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_blai_tzmid` reader - "]
pub type TZC_BLAI_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_tzmid` reader - "]
pub type TZC_CODEC_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_subsys_tzmid` reader - "]
pub type TZC_SUBSYS_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_x2ddma_tzmid` reader - "]
pub type TZC_X2DDMA_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `tzc_xdma_tzmid` reader - "]
pub type TZC_XDMA_TZMID_R = crate::BitReader<bool>;
#[doc = "Field `reserved_6_15` reader - "]
pub type RESERVED_6_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_mmcpu_tzmid_sel` reader - "]
pub type TZC_MMCPU_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_blai_tzmid_sel` reader - "]
pub type TZC_BLAI_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_tzmid_sel` reader - "]
pub type TZC_CODEC_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_subsys_tzmid_sel` reader - "]
pub type TZC_SUBSYS_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_x2ddma_tzmid_sel` reader - "]
pub type TZC_X2DDMA_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `tzc_xdma_tzmid_sel` reader - "]
pub type TZC_XDMA_TZMID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reserved_22_31` reader - "]
pub type RESERVED_22_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid(&self) -> TZC_MMCPU_TZMID_R {
        TZC_MMCPU_TZMID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_blai_tzmid(&self) -> TZC_BLAI_TZMID_R {
        TZC_BLAI_TZMID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_tzmid(&self) -> TZC_CODEC_TZMID_R {
        TZC_CODEC_TZMID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid(&self) -> TZC_SUBSYS_TZMID_R {
        TZC_SUBSYS_TZMID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid(&self) -> TZC_X2DDMA_TZMID_R {
        TZC_X2DDMA_TZMID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid(&self) -> TZC_XDMA_TZMID_R {
        TZC_XDMA_TZMID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn reserved_6_15(&self) -> RESERVED_6_15_R {
        RESERVED_6_15_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_sel(&self) -> TZC_MMCPU_TZMID_SEL_R {
        TZC_MMCPU_TZMID_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_sel(&self) -> TZC_BLAI_TZMID_SEL_R {
        TZC_BLAI_TZMID_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_sel(&self) -> TZC_CODEC_TZMID_SEL_R {
        TZC_CODEC_TZMID_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_sel(&self) -> TZC_SUBSYS_TZMID_SEL_R {
        TZC_SUBSYS_TZMID_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_sel(&self) -> TZC_X2DDMA_TZMID_SEL_R {
        TZC_X2DDMA_TZMID_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_sel(&self) -> TZC_XDMA_TZMID_SEL_R {
        TZC_XDMA_TZMID_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn reserved_22_31(&self) -> RESERVED_22_31_R {
        RESERVED_22_31_R::new(((self.bits >> 22) & 0x03ff) as u16)
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
#[doc = "tzc_mm_bmx_tzmid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_tzmid](index.html) module"]
pub struct TZC_MM_BMX_TZMID_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_TZMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_tzmid::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_TZMID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_tzmid::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_TZMID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_tzmid to value 0x003f_0000"]
impl crate::Resettable for TZC_MM_BMX_TZMID_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_0000;
}
