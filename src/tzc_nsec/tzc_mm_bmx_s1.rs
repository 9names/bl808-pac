#[doc = "Register `tzc_mm_bmx_s1` reader"]
pub struct R(crate::R<TZC_MM_BMX_S1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_S1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_S1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_S1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_s1` writer"]
pub struct W(crate::W<TZC_MM_BMX_S1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_S1_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_S1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_S1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_s10_tzsid_en` reader - "]
pub type TZC_S10_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s11_tzsid_en` reader - "]
pub type TZC_S11_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s12_tzsid_en` reader - "]
pub type TZC_S12_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s13_tzsid_en` reader - "]
pub type TZC_S13_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s14_tzsid_en` reader - "]
pub type TZC_S14_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s15_tzsid_en` reader - "]
pub type TZC_S15_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s16_tzsid_en` reader - "]
pub type TZC_S16_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s17_tzsid_en` reader - "]
pub type TZC_S17_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s18_tzsid_en` reader - "]
pub type TZC_S18_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s19_tzsid_en` reader - "]
pub type TZC_S19_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1a_tzsid_en` reader - "]
pub type TZC_S1A_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1b_tzsid_en` reader - "]
pub type TZC_S1B_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1c_tzsid_en` reader - "]
pub type TZC_S1C_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1d_tzsid_en` reader - "]
pub type TZC_S1D_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1e_tzsid_en` reader - "]
pub type TZC_S1E_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_s1f_tzsid_en` reader - "]
pub type TZC_S1F_TZSID_EN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_s10_tzsid_en(&self) -> TZC_S10_TZSID_EN_R {
        TZC_S10_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_s11_tzsid_en(&self) -> TZC_S11_TZSID_EN_R {
        TZC_S11_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_s12_tzsid_en(&self) -> TZC_S12_TZSID_EN_R {
        TZC_S12_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_s13_tzsid_en(&self) -> TZC_S13_TZSID_EN_R {
        TZC_S13_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_s14_tzsid_en(&self) -> TZC_S14_TZSID_EN_R {
        TZC_S14_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_s15_tzsid_en(&self) -> TZC_S15_TZSID_EN_R {
        TZC_S15_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_s16_tzsid_en(&self) -> TZC_S16_TZSID_EN_R {
        TZC_S16_TZSID_EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_s17_tzsid_en(&self) -> TZC_S17_TZSID_EN_R {
        TZC_S17_TZSID_EN_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_s18_tzsid_en(&self) -> TZC_S18_TZSID_EN_R {
        TZC_S18_TZSID_EN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_s19_tzsid_en(&self) -> TZC_S19_TZSID_EN_R {
        TZC_S19_TZSID_EN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_s1a_tzsid_en(&self) -> TZC_S1A_TZSID_EN_R {
        TZC_S1A_TZSID_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_s1b_tzsid_en(&self) -> TZC_S1B_TZSID_EN_R {
        TZC_S1B_TZSID_EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_s1c_tzsid_en(&self) -> TZC_S1C_TZSID_EN_R {
        TZC_S1C_TZSID_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_s1d_tzsid_en(&self) -> TZC_S1D_TZSID_EN_R {
        TZC_S1D_TZSID_EN_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_s1e_tzsid_en(&self) -> TZC_S1E_TZSID_EN_R {
        TZC_S1E_TZSID_EN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_s1f_tzsid_en(&self) -> TZC_S1F_TZSID_EN_R {
        TZC_S1F_TZSID_EN_R::new(((self.bits >> 30) & 3) as u8)
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
#[doc = "tzc_mm_bmx_s1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_s1](index.html) module"]
pub struct TZC_MM_BMX_S1_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_S1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_s1::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_S1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_s1::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_S1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_s1 to value 0xffff_ffff"]
impl crate::Resettable for TZC_MM_BMX_S1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
