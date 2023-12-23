#[doc = "Register `tzc_mm_bmx_s_lock1` reader"]
pub struct R(crate::R<TZC_MM_BMX_S_LOCK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_S_LOCK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_S_LOCK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_S_LOCK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_s_lock1` writer"]
pub struct W(crate::W<TZC_MM_BMX_S_LOCK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_S_LOCK1_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_S_LOCK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_S_LOCK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_codec_s10_tzsid_lock` reader - "]
pub type TZC_CODEC_S10_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s11_tzsid_lock` reader - "]
pub type TZC_CODEC_S11_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s12_tzsid_lock` reader - "]
pub type TZC_CODEC_S12_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s13_tzsid_lock` reader - "]
pub type TZC_CODEC_S13_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s14_tzsid_lock` reader - "]
pub type TZC_CODEC_S14_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s15_tzsid_lock` reader - "]
pub type TZC_CODEC_S15_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s16_tzsid_lock` reader - "]
pub type TZC_CODEC_S16_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s17_tzsid_lock` reader - "]
pub type TZC_CODEC_S17_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s18_tzsid_lock` reader - "]
pub type TZC_CODEC_S18_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s19_tzsid_lock` reader - "]
pub type TZC_CODEC_S19_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1a_tzsid_lock` reader - "]
pub type TZC_CODEC_S1A_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1b_tzsid_lock` reader - "]
pub type TZC_CODEC_S1B_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1c_tzsid_lock` reader - "]
pub type TZC_CODEC_S1C_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1d_tzsid_lock` reader - "]
pub type TZC_CODEC_S1D_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1e_tzsid_lock` reader - "]
pub type TZC_CODEC_S1E_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_codec_s1f_tzsid_lock` reader - "]
pub type TZC_CODEC_S1F_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s10_tzsid_lock` reader - "]
pub type TZC_S10_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s11_tzsid_lock` reader - "]
pub type TZC_S11_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s12_tzsid_lock` reader - "]
pub type TZC_S12_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s13_tzsid_lock` reader - "]
pub type TZC_S13_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s14_tzsid_lock` reader - "]
pub type TZC_S14_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s15_tzsid_lock` reader - "]
pub type TZC_S15_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s16_tzsid_lock` reader - "]
pub type TZC_S16_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s17_tzsid_lock` reader - "]
pub type TZC_S17_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s18_tzsid_lock` reader - "]
pub type TZC_S18_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s19_tzsid_lock` reader - "]
pub type TZC_S19_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1a_tzsid_lock` reader - "]
pub type TZC_S1A_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1b_tzsid_lock` reader - "]
pub type TZC_S1B_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1c_tzsid_lock` reader - "]
pub type TZC_S1C_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1d_tzsid_lock` reader - "]
pub type TZC_S1D_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1e_tzsid_lock` reader - "]
pub type TZC_S1E_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_s1f_tzsid_lock` reader - "]
pub type TZC_S1F_TZSID_LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_codec_s10_tzsid_lock(&self) -> TZC_CODEC_S10_TZSID_LOCK_R {
        TZC_CODEC_S10_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_codec_s11_tzsid_lock(&self) -> TZC_CODEC_S11_TZSID_LOCK_R {
        TZC_CODEC_S11_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_s12_tzsid_lock(&self) -> TZC_CODEC_S12_TZSID_LOCK_R {
        TZC_CODEC_S12_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_codec_s13_tzsid_lock(&self) -> TZC_CODEC_S13_TZSID_LOCK_R {
        TZC_CODEC_S13_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_codec_s14_tzsid_lock(&self) -> TZC_CODEC_S14_TZSID_LOCK_R {
        TZC_CODEC_S14_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_codec_s15_tzsid_lock(&self) -> TZC_CODEC_S15_TZSID_LOCK_R {
        TZC_CODEC_S15_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_codec_s16_tzsid_lock(&self) -> TZC_CODEC_S16_TZSID_LOCK_R {
        TZC_CODEC_S16_TZSID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_codec_s17_tzsid_lock(&self) -> TZC_CODEC_S17_TZSID_LOCK_R {
        TZC_CODEC_S17_TZSID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_codec_s18_tzsid_lock(&self) -> TZC_CODEC_S18_TZSID_LOCK_R {
        TZC_CODEC_S18_TZSID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_codec_s19_tzsid_lock(&self) -> TZC_CODEC_S19_TZSID_LOCK_R {
        TZC_CODEC_S19_TZSID_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_codec_s1a_tzsid_lock(&self) -> TZC_CODEC_S1A_TZSID_LOCK_R {
        TZC_CODEC_S1A_TZSID_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_codec_s1b_tzsid_lock(&self) -> TZC_CODEC_S1B_TZSID_LOCK_R {
        TZC_CODEC_S1B_TZSID_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_codec_s1c_tzsid_lock(&self) -> TZC_CODEC_S1C_TZSID_LOCK_R {
        TZC_CODEC_S1C_TZSID_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_codec_s1d_tzsid_lock(&self) -> TZC_CODEC_S1D_TZSID_LOCK_R {
        TZC_CODEC_S1D_TZSID_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_codec_s1e_tzsid_lock(&self) -> TZC_CODEC_S1E_TZSID_LOCK_R {
        TZC_CODEC_S1E_TZSID_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_codec_s1f_tzsid_lock(&self) -> TZC_CODEC_S1F_TZSID_LOCK_R {
        TZC_CODEC_S1F_TZSID_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_s10_tzsid_lock(&self) -> TZC_S10_TZSID_LOCK_R {
        TZC_S10_TZSID_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_s11_tzsid_lock(&self) -> TZC_S11_TZSID_LOCK_R {
        TZC_S11_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_s12_tzsid_lock(&self) -> TZC_S12_TZSID_LOCK_R {
        TZC_S12_TZSID_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_s13_tzsid_lock(&self) -> TZC_S13_TZSID_LOCK_R {
        TZC_S13_TZSID_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_s14_tzsid_lock(&self) -> TZC_S14_TZSID_LOCK_R {
        TZC_S14_TZSID_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_s15_tzsid_lock(&self) -> TZC_S15_TZSID_LOCK_R {
        TZC_S15_TZSID_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_s16_tzsid_lock(&self) -> TZC_S16_TZSID_LOCK_R {
        TZC_S16_TZSID_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_s17_tzsid_lock(&self) -> TZC_S17_TZSID_LOCK_R {
        TZC_S17_TZSID_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_s18_tzsid_lock(&self) -> TZC_S18_TZSID_LOCK_R {
        TZC_S18_TZSID_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_s19_tzsid_lock(&self) -> TZC_S19_TZSID_LOCK_R {
        TZC_S19_TZSID_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_s1a_tzsid_lock(&self) -> TZC_S1A_TZSID_LOCK_R {
        TZC_S1A_TZSID_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_s1b_tzsid_lock(&self) -> TZC_S1B_TZSID_LOCK_R {
        TZC_S1B_TZSID_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_s1c_tzsid_lock(&self) -> TZC_S1C_TZSID_LOCK_R {
        TZC_S1C_TZSID_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_s1d_tzsid_lock(&self) -> TZC_S1D_TZSID_LOCK_R {
        TZC_S1D_TZSID_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_s1e_tzsid_lock(&self) -> TZC_S1E_TZSID_LOCK_R {
        TZC_S1E_TZSID_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_s1f_tzsid_lock(&self) -> TZC_S1F_TZSID_LOCK_R {
        TZC_S1F_TZSID_LOCK_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "tzc_mm_bmx_s_lock1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_s_lock1](index.html) module"]
pub struct TZC_MM_BMX_S_LOCK1_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_S_LOCK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_s_lock1::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_S_LOCK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_s_lock1::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_S_LOCK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_s_lock1 to value 0"]
impl crate::Resettable for TZC_MM_BMX_S_LOCK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
