#[doc = "Register `tzc_mm_bmx_tzmid_lock` reader"]
pub struct R(crate::R<TZC_MM_BMX_TZMID_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_TZMID_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_TZMID_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_TZMID_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_tzmid_lock` writer"]
pub struct W(crate::W<TZC_MM_BMX_TZMID_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_TZMID_LOCK_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_TZMID_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_TZMID_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mmcpu_tzmid_lock` reader - "]
pub type TZC_MMCPU_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmcpu_tzmid_lock` writer - "]
pub type TZC_MMCPU_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_blai_tzmid_lock` reader - "]
pub type TZC_BLAI_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_blai_tzmid_lock` writer - "]
pub type TZC_BLAI_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_codec_tzmid_lock` reader - "]
pub type TZC_CODEC_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_tzmid_lock` writer - "]
pub type TZC_CODEC_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_subsys_tzmid_lock` reader - "]
pub type TZC_SUBSYS_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_subsys_tzmid_lock` writer - "]
pub type TZC_SUBSYS_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_x2ddma_tzmid_lock` reader - "]
pub type TZC_X2DDMA_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_x2ddma_tzmid_lock` writer - "]
pub type TZC_X2DDMA_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_xdma_tzmid_lock` reader - "]
pub type TZC_XDMA_TZMID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_xdma_tzmid_lock` writer - "]
pub type TZC_XDMA_TZMID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_TZMID_LOCK_SPEC, bool, O>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_lock(&self) -> TZC_MMCPU_TZMID_LOCK_R {
        TZC_MMCPU_TZMID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_lock(&self) -> TZC_BLAI_TZMID_LOCK_R {
        TZC_BLAI_TZMID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_lock(&self) -> TZC_CODEC_TZMID_LOCK_R {
        TZC_CODEC_TZMID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_lock(&self) -> TZC_SUBSYS_TZMID_LOCK_R {
        TZC_SUBSYS_TZMID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_lock(&self) -> TZC_X2DDMA_TZMID_LOCK_R {
        TZC_X2DDMA_TZMID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_lock(&self) -> TZC_XDMA_TZMID_LOCK_R {
        TZC_XDMA_TZMID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved_6_31(&self) -> RESERVED_6_31_R {
        RESERVED_6_31_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmcpu_tzmid_lock(&mut self) -> TZC_MMCPU_TZMID_LOCK_W<0> {
        TZC_MMCPU_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_blai_tzmid_lock(&mut self) -> TZC_BLAI_TZMID_LOCK_W<1> {
        TZC_BLAI_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_codec_tzmid_lock(&mut self) -> TZC_CODEC_TZMID_LOCK_W<2> {
        TZC_CODEC_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_subsys_tzmid_lock(&mut self) -> TZC_SUBSYS_TZMID_LOCK_W<3> {
        TZC_SUBSYS_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_x2ddma_tzmid_lock(&mut self) -> TZC_X2DDMA_TZMID_LOCK_W<4> {
        TZC_X2DDMA_TZMID_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_xdma_tzmid_lock(&mut self) -> TZC_XDMA_TZMID_LOCK_W<5> {
        TZC_XDMA_TZMID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_mm_bmx_tzmid_lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_tzmid_lock](index.html) module"]
pub struct TZC_MM_BMX_TZMID_LOCK_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_TZMID_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_tzmid_lock::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_TZMID_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_tzmid_lock::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_TZMID_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_tzmid_lock to value 0"]
impl crate::Resettable for TZC_MM_BMX_TZMID_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
