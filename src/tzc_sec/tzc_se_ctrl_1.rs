#[doc = "Register `tzc_se_ctrl_1` reader"]
pub struct R(crate::R<TZC_SE_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SE_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SE_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SE_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_se_ctrl_1` writer"]
pub struct W(crate::W<TZC_SE_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SE_CTRL_1_SPEC>;
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
impl From<crate::W<TZC_SE_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SE_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_sf_cr_tzsid_en` reader - "]
pub type TZC_SF_CR_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_sf_cr_tzsid_en` writer - "]
pub type TZC_SF_CR_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_SE_CTRL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_sf_sec_tzsid_en` reader - "]
pub type TZC_SF_SEC_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_sf_sec_tzsid_en` writer - "]
pub type TZC_SF_SEC_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_SE_CTRL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_sf_tzsid_crmd` reader - "]
pub type TZC_SF_TZSID_CRMD_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsid_crmd` writer - "]
pub type TZC_SF_TZSID_CRMD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_1_SPEC, bool, O>;
#[doc = "Field `reserved_5_31` reader - "]
pub type RESERVED_5_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_en(&self) -> TZC_SF_CR_TZSID_EN_R {
        TZC_SF_CR_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_en(&self) -> TZC_SF_SEC_TZSID_EN_R {
        TZC_SF_SEC_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd(&self) -> TZC_SF_TZSID_CRMD_R {
        TZC_SF_TZSID_CRMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn reserved_5_31(&self) -> RESERVED_5_31_R {
        RESERVED_5_31_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_cr_tzsid_en(&mut self) -> TZC_SF_CR_TZSID_EN_W<0> {
        TZC_SF_CR_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_sec_tzsid_en(&mut self) -> TZC_SF_SEC_TZSID_EN_W<2> {
        TZC_SF_SEC_TZSID_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsid_crmd(&mut self) -> TZC_SF_TZSID_CRMD_W<4> {
        TZC_SF_TZSID_CRMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_se_ctrl_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_se_ctrl_1](index.html) module"]
pub struct TZC_SE_CTRL_1_SPEC;
impl crate::RegisterSpec for TZC_SE_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_se_ctrl_1::R](R) reader structure"]
impl crate::Readable for TZC_SE_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_se_ctrl_1::W](W) writer structure"]
impl crate::Writable for TZC_SE_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_se_ctrl_1 to value 0x0f"]
impl crate::Resettable for TZC_SE_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
