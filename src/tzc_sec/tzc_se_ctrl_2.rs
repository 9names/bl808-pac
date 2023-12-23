#[doc = "Register `tzc_se_ctrl_2` reader"]
pub struct R(crate::R<TZC_SE_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SE_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SE_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SE_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_se_ctrl_2` writer"]
pub struct W(crate::W<TZC_SE_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SE_CTRL_2_SPEC>;
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
impl From<crate::W<TZC_SE_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SE_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_se_sha_tzsid_lock` reader - "]
pub type TZC_SE_SHA_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_sha_tzsid_lock` writer - "]
pub type TZC_SE_SHA_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_aes_tzsid_lock` reader - "]
pub type TZC_SE_AES_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_aes_tzsid_lock` writer - "]
pub type TZC_SE_AES_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_trng_tzsid_lock` reader - "]
pub type TZC_SE_TRNG_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_trng_tzsid_lock` writer - "]
pub type TZC_SE_TRNG_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_pka_tzsid_lock` reader - "]
pub type TZC_SE_PKA_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_pka_tzsid_lock` writer - "]
pub type TZC_SE_PKA_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_cdet_tzsid_lock` reader - "]
pub type TZC_SE_CDET_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_cdet_tzsid_lock` writer - "]
pub type TZC_SE_CDET_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_gmac_tzsid_lock` reader - "]
pub type TZC_SE_GMAC_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_gmac_tzsid_lock` writer - "]
pub type TZC_SE_GMAC_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_se_tzsid_crmd_lock` reader - "]
pub type TZC_SE_TZSID_CRMD_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_se_tzsid_crmd_lock` writer - "]
pub type TZC_SE_TZSID_CRMD_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `reserved_7_15` reader - "]
pub type RESERVED_7_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_sf_cr_tzsid_lock` reader - "]
pub type TZC_SF_CR_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_cr_tzsid_lock` writer - "]
pub type TZC_SF_CR_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_sf_sec_tzsid_lock` reader - "]
pub type TZC_SF_SEC_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_sec_tzsid_lock` writer - "]
pub type TZC_SF_SEC_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` reader - "]
pub type TZC_SF_TZSID_CRMD_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` writer - "]
pub type TZC_SF_TZSID_CRMD_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_SE_CTRL_2_SPEC, bool, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_lock(&self) -> TZC_SE_SHA_TZSID_LOCK_R {
        TZC_SE_SHA_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_lock(&self) -> TZC_SE_AES_TZSID_LOCK_R {
        TZC_SE_AES_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_lock(&self) -> TZC_SE_TRNG_TZSID_LOCK_R {
        TZC_SE_TRNG_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_lock(&self) -> TZC_SE_PKA_TZSID_LOCK_R {
        TZC_SE_PKA_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_lock(&self) -> TZC_SE_CDET_TZSID_LOCK_R {
        TZC_SE_CDET_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_lock(&self) -> TZC_SE_GMAC_TZSID_LOCK_R {
        TZC_SE_GMAC_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd_lock(&self) -> TZC_SE_TZSID_CRMD_LOCK_R {
        TZC_SE_TZSID_CRMD_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn reserved_7_15(&self) -> RESERVED_7_15_R {
        RESERVED_7_15_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_lock(&self) -> TZC_SF_CR_TZSID_LOCK_R {
        TZC_SF_CR_TZSID_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_lock(&self) -> TZC_SF_SEC_TZSID_LOCK_R {
        TZC_SF_SEC_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd_lock(&self) -> TZC_SF_TZSID_CRMD_LOCK_R {
        TZC_SF_TZSID_CRMD_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn reserved_19_31(&self) -> RESERVED_19_31_R {
        RESERVED_19_31_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_sha_tzsid_lock(&mut self) -> TZC_SE_SHA_TZSID_LOCK_W<0> {
        TZC_SE_SHA_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_aes_tzsid_lock(&mut self) -> TZC_SE_AES_TZSID_LOCK_W<1> {
        TZC_SE_AES_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_trng_tzsid_lock(&mut self) -> TZC_SE_TRNG_TZSID_LOCK_W<2> {
        TZC_SE_TRNG_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_pka_tzsid_lock(&mut self) -> TZC_SE_PKA_TZSID_LOCK_W<3> {
        TZC_SE_PKA_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_cdet_tzsid_lock(&mut self) -> TZC_SE_CDET_TZSID_LOCK_W<4> {
        TZC_SE_CDET_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_gmac_tzsid_lock(&mut self) -> TZC_SE_GMAC_TZSID_LOCK_W<5> {
        TZC_SE_GMAC_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_tzsid_crmd_lock(&mut self) -> TZC_SE_TZSID_CRMD_LOCK_W<6> {
        TZC_SE_TZSID_CRMD_LOCK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_cr_tzsid_lock(&mut self) -> TZC_SF_CR_TZSID_LOCK_W<16> {
        TZC_SF_CR_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_sec_tzsid_lock(&mut self) -> TZC_SF_SEC_TZSID_LOCK_W<17> {
        TZC_SF_SEC_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsid_crmd_lock(&mut self) -> TZC_SF_TZSID_CRMD_LOCK_W<18> {
        TZC_SF_TZSID_CRMD_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_se_ctrl_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_se_ctrl_2](index.html) module"]
pub struct TZC_SE_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_SE_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_se_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_SE_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_se_ctrl_2::W](W) writer structure"]
impl crate::Writable for TZC_SE_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_se_ctrl_2 to value 0"]
impl crate::Resettable for TZC_SE_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
