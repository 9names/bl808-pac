#[doc = "Register `cgen_cfg3` reader"]
pub struct R(crate::R<CGEN_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg3` writer"]
pub struct W(crate::W<CGEN_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG3_SPEC>;
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
impl From<crate::W<CGEN_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_mm_wifipll_160m` reader - "]
pub type CGEN_MM_WIFIPLL_160M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_mm_wifipll_160m` writer - "]
pub type CGEN_MM_WIFIPLL_160M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_mm_wifipll_240m` reader - "]
pub type CGEN_MM_WIFIPLL_240M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_mm_wifipll_240m` writer - "]
pub type CGEN_MM_WIFIPLL_240M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_mm_wifipll_320m` reader - "]
pub type CGEN_MM_WIFIPLL_320M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_mm_wifipll_320m` writer - "]
pub type CGEN_MM_WIFIPLL_320M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_mm_aupll_div1` reader - "]
pub type CGEN_MM_AUPLL_DIV1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_mm_aupll_div1` writer - "]
pub type CGEN_MM_AUPLL_DIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_mm_aupll_div2` reader - "]
pub type CGEN_MM_AUPLL_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_mm_aupll_div2` writer - "]
pub type CGEN_MM_AUPLL_DIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_emi_cpupll_400m` reader - "]
pub type CGEN_EMI_CPUPLL_400M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_emi_cpupll_400m` writer - "]
pub type CGEN_EMI_CPUPLL_400M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_emi_cpupll_200m` reader - "]
pub type CGEN_EMI_CPUPLL_200M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_emi_cpupll_200m` writer - "]
pub type CGEN_EMI_CPUPLL_200M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_emi_wifipll_320m` reader - "]
pub type CGEN_EMI_WIFIPLL_320M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_emi_wifipll_320m` writer - "]
pub type CGEN_EMI_WIFIPLL_320M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_emi_aupll_div1` reader - "]
pub type CGEN_EMI_AUPLL_DIV1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_emi_aupll_div1` writer - "]
pub type CGEN_EMI_AUPLL_DIV1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_cpupll_80m` reader - "]
pub type CGEN_TOP_CPUPLL_80M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_cpupll_80m` writer - "]
pub type CGEN_TOP_CPUPLL_80M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_cpupll_100m` reader - "]
pub type CGEN_TOP_CPUPLL_100M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_cpupll_100m` writer - "]
pub type CGEN_TOP_CPUPLL_100M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_cpupll_160m` reader - "]
pub type CGEN_TOP_CPUPLL_160M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_cpupll_160m` writer - "]
pub type CGEN_TOP_CPUPLL_160M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_cpupll_400m` reader - "]
pub type CGEN_TOP_CPUPLL_400M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_cpupll_400m` writer - "]
pub type CGEN_TOP_CPUPLL_400M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_wifipll_240m` reader - "]
pub type CGEN_TOP_WIFIPLL_240M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_wifipll_240m` writer - "]
pub type CGEN_TOP_WIFIPLL_240M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_wifipll_320m` reader - "]
pub type CGEN_TOP_WIFIPLL_320M_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_wifipll_320m` writer - "]
pub type CGEN_TOP_WIFIPLL_320M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_aupll_div2` reader - "]
pub type CGEN_TOP_AUPLL_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_aupll_div2` writer - "]
pub type CGEN_TOP_AUPLL_DIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `cgen_top_aupll_div1` reader - "]
pub type CGEN_TOP_AUPLL_DIV1_R = crate::BitReader<bool>;
#[doc = "Field `cgen_top_aupll_div1` writer - "]
pub type CGEN_TOP_AUPLL_DIV1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_160m(&self) -> CGEN_MM_WIFIPLL_160M_R {
        CGEN_MM_WIFIPLL_160M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_240m(&self) -> CGEN_MM_WIFIPLL_240M_R {
        CGEN_MM_WIFIPLL_240M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_320m(&self) -> CGEN_MM_WIFIPLL_320M_R {
        CGEN_MM_WIFIPLL_320M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div1(&self) -> CGEN_MM_AUPLL_DIV1_R {
        CGEN_MM_AUPLL_DIV1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div2(&self) -> CGEN_MM_AUPLL_DIV2_R {
        CGEN_MM_AUPLL_DIV2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_400m(&self) -> CGEN_EMI_CPUPLL_400M_R {
        CGEN_EMI_CPUPLL_400M_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_200m(&self) -> CGEN_EMI_CPUPLL_200M_R {
        CGEN_EMI_CPUPLL_200M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_emi_wifipll_320m(&self) -> CGEN_EMI_WIFIPLL_320M_R {
        CGEN_EMI_WIFIPLL_320M_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cgen_emi_aupll_div1(&self) -> CGEN_EMI_AUPLL_DIV1_R {
        CGEN_EMI_AUPLL_DIV1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cgen_top_cpupll_80m(&self) -> CGEN_TOP_CPUPLL_80M_R {
        CGEN_TOP_CPUPLL_80M_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_top_cpupll_100m(&self) -> CGEN_TOP_CPUPLL_100M_R {
        CGEN_TOP_CPUPLL_100M_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_top_cpupll_160m(&self) -> CGEN_TOP_CPUPLL_160M_R {
        CGEN_TOP_CPUPLL_160M_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_top_cpupll_400m(&self) -> CGEN_TOP_CPUPLL_400M_R {
        CGEN_TOP_CPUPLL_400M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cgen_top_wifipll_240m(&self) -> CGEN_TOP_WIFIPLL_240M_R {
        CGEN_TOP_WIFIPLL_240M_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cgen_top_wifipll_320m(&self) -> CGEN_TOP_WIFIPLL_320M_R {
        CGEN_TOP_WIFIPLL_320M_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cgen_top_aupll_div2(&self) -> CGEN_TOP_AUPLL_DIV2_R {
        CGEN_TOP_AUPLL_DIV2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_top_aupll_div1(&self) -> CGEN_TOP_AUPLL_DIV1_R {
        CGEN_TOP_AUPLL_DIV1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_mm_wifipll_160m(&mut self) -> CGEN_MM_WIFIPLL_160M_W<0> {
        CGEN_MM_WIFIPLL_160M_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_mm_wifipll_240m(&mut self) -> CGEN_MM_WIFIPLL_240M_W<1> {
        CGEN_MM_WIFIPLL_240M_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_mm_wifipll_320m(&mut self) -> CGEN_MM_WIFIPLL_320M_W<2> {
        CGEN_MM_WIFIPLL_320M_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_mm_aupll_div1(&mut self) -> CGEN_MM_AUPLL_DIV1_W<3> {
        CGEN_MM_AUPLL_DIV1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_mm_aupll_div2(&mut self) -> CGEN_MM_AUPLL_DIV2_W<4> {
        CGEN_MM_AUPLL_DIV2_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_emi_cpupll_400m(&mut self) -> CGEN_EMI_CPUPLL_400M_W<5> {
        CGEN_EMI_CPUPLL_400M_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_emi_cpupll_200m(&mut self) -> CGEN_EMI_CPUPLL_200M_W<6> {
        CGEN_EMI_CPUPLL_200M_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_emi_wifipll_320m(&mut self) -> CGEN_EMI_WIFIPLL_320M_W<7> {
        CGEN_EMI_WIFIPLL_320M_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_emi_aupll_div1(&mut self) -> CGEN_EMI_AUPLL_DIV1_W<8> {
        CGEN_EMI_AUPLL_DIV1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_cpupll_80m(&mut self) -> CGEN_TOP_CPUPLL_80M_W<9> {
        CGEN_TOP_CPUPLL_80M_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_cpupll_100m(&mut self) -> CGEN_TOP_CPUPLL_100M_W<10> {
        CGEN_TOP_CPUPLL_100M_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_cpupll_160m(&mut self) -> CGEN_TOP_CPUPLL_160M_W<11> {
        CGEN_TOP_CPUPLL_160M_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_cpupll_400m(&mut self) -> CGEN_TOP_CPUPLL_400M_W<12> {
        CGEN_TOP_CPUPLL_400M_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_wifipll_240m(&mut self) -> CGEN_TOP_WIFIPLL_240M_W<13> {
        CGEN_TOP_WIFIPLL_240M_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_wifipll_320m(&mut self) -> CGEN_TOP_WIFIPLL_320M_W<14> {
        CGEN_TOP_WIFIPLL_320M_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div2(&mut self) -> CGEN_TOP_AUPLL_DIV2_W<15> {
        CGEN_TOP_AUPLL_DIV2_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_top_aupll_div1(&mut self) -> CGEN_TOP_AUPLL_DIV1_W<16> {
        CGEN_TOP_AUPLL_DIV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg3](index.html) module"]
pub struct CGEN_CFG3_SPEC;
impl crate::RegisterSpec for CGEN_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg3::R](R) reader structure"]
impl crate::Readable for CGEN_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg3::W](W) writer structure"]
impl crate::Writable for CGEN_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg3 to value 0x0001_ffff"]
impl crate::Resettable for CGEN_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_ffff;
}
