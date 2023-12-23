#[doc = "Register `uhs_pll_cfg3` reader"]
pub struct R(crate::R<UHS_PLL_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_PLL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_PLL_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_PLL_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_pll_cfg3` writer"]
pub struct W(crate::W<UHS_PLL_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_PLL_CFG3_SPEC>;
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
impl From<crate::W<UHS_PLL_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_PLL_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhspll_c4_en` reader - "]
pub type UHSPLL_C4_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_c4_en` writer - "]
pub type UHSPLL_C4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_r4` reader - "]
pub type UHSPLL_R4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_r4` writer - "]
pub type UHSPLL_R4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_r4_short` reader - "]
pub type UHSPLL_R4_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_r4_short` writer - "]
pub type UHSPLL_R4_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_9_11` reader - "]
pub type RESERVED_9_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_c3` reader - "]
pub type UHSPLL_C3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_c3` writer - "]
pub type UHSPLL_C3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `uhspll_cz` reader - "]
pub type UHSPLL_CZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_cz` writer - "]
pub type UHSPLL_CZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `uhspll_rz` reader - "]
pub type UHSPLL_RZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhspll_rz` writer - "]
pub type UHSPLL_RZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_PLL_CFG3_SPEC, u8, u8, 3, O>;
#[doc = "Field `uhspll_lf_test_en` reader - "]
pub type UHSPLL_LF_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_lf_test_en` writer - "]
pub type UHSPLL_LF_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `uhspll_fast_lock_en` reader - "]
pub type UHSPLL_FAST_LOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `uhspll_fast_lock_en` writer - "]
pub type UHSPLL_FAST_LOCK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_21_31` reader - "]
pub type RESERVED_21_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_c4_en(&self) -> UHSPLL_C4_EN_R {
        UHSPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_r4(&self) -> UHSPLL_R4_R {
        UHSPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_r4_short(&self) -> UHSPLL_R4_SHORT_R {
        UHSPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reserved_9_11(&self) -> RESERVED_9_11_R {
        RESERVED_9_11_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn uhspll_c3(&self) -> UHSPLL_C3_R {
        UHSPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn uhspll_cz(&self) -> UHSPLL_CZ_R {
        UHSPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn uhspll_rz(&self) -> UHSPLL_RZ_R {
        UHSPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uhspll_lf_test_en(&self) -> UHSPLL_LF_TEST_EN_R {
        UHSPLL_LF_TEST_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uhspll_fast_lock_en(&self) -> UHSPLL_FAST_LOCK_EN_R {
        UHSPLL_FAST_LOCK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn reserved_21_31(&self) -> RESERVED_21_31_R {
        RESERVED_21_31_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_c4_en(&mut self) -> UHSPLL_C4_EN_W<0> {
        UHSPLL_C4_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_r4(&mut self) -> UHSPLL_R4_W<4> {
        UHSPLL_R4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_r4_short(&mut self) -> UHSPLL_R4_SHORT_W<8> {
        UHSPLL_R4_SHORT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_c3(&mut self) -> UHSPLL_C3_W<12> {
        UHSPLL_C3_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_cz(&mut self) -> UHSPLL_CZ_W<14> {
        UHSPLL_CZ_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_rz(&mut self) -> UHSPLL_RZ_W<16> {
        UHSPLL_RZ_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_lf_test_en(&mut self) -> UHSPLL_LF_TEST_EN_W<19> {
        UHSPLL_LF_TEST_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn uhspll_fast_lock_en(&mut self) -> UHSPLL_FAST_LOCK_EN_W<20> {
        UHSPLL_FAST_LOCK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhs_pll_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_pll_cfg3](index.html) module"]
pub struct UHS_PLL_CFG3_SPEC;
impl crate::RegisterSpec for UHS_PLL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_pll_cfg3::R](R) reader structure"]
impl crate::Readable for UHS_PLL_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_pll_cfg3::W](W) writer structure"]
impl crate::Writable for UHS_PLL_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_pll_cfg3 to value 0x0011_a011"]
impl crate::Resettable for UHS_PLL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_a011;
}
