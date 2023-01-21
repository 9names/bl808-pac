#[doc = "Register `wifi_pll_cfg3` reader"]
pub struct R(crate::R<WIFI_PLL_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg3` writer"]
pub struct W(crate::W<WIFI_PLL_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG3_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_c4_en` reader - "]
pub type WIFIPLL_C4_EN_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_c4_en` writer - "]
pub type WIFIPLL_C4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFI_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_r4` reader - "]
pub type WIFIPLL_R4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_r4` writer - "]
pub type WIFIPLL_R4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_r4_short` reader - "]
pub type WIFIPLL_R4_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_r4_short` writer - "]
pub type WIFIPLL_R4_SHORT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_9_11` reader - "]
pub type RESERVED_9_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_c3` reader - "]
pub type WIFIPLL_C3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_c3` writer - "]
pub type WIFIPLL_C3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifipll_cz` reader - "]
pub type WIFIPLL_CZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_cz` writer - "]
pub type WIFIPLL_CZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifipll_rz` reader - "]
pub type WIFIPLL_RZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifipll_rz` writer - "]
pub type WIFIPLL_RZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG3_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_c4_en(&self) -> WIFIPLL_C4_EN_R {
        WIFIPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_r4(&self) -> WIFIPLL_R4_R {
        WIFIPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_r4_short(&self) -> WIFIPLL_R4_SHORT_R {
        WIFIPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reserved_9_11(&self) -> RESERVED_9_11_R {
        RESERVED_9_11_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_c3(&self) -> WIFIPLL_C3_R {
        WIFIPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn wifipll_cz(&self) -> WIFIPLL_CZ_R {
        WIFIPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn wifipll_rz(&self) -> WIFIPLL_RZ_R {
        WIFIPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
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
    pub fn wifipll_c4_en(&mut self) -> WIFIPLL_C4_EN_W<0> {
        WIFIPLL_C4_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_r4(&mut self) -> WIFIPLL_R4_W<4> {
        WIFIPLL_R4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_r4_short(&mut self) -> WIFIPLL_R4_SHORT_W<8> {
        WIFIPLL_R4_SHORT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_c3(&mut self) -> WIFIPLL_C3_W<12> {
        WIFIPLL_C3_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cz(&mut self) -> WIFIPLL_CZ_W<14> {
        WIFIPLL_CZ_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_rz(&mut self) -> WIFIPLL_RZ_W<16> {
        WIFIPLL_RZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg3](index.html) module"]
pub struct WIFI_PLL_CFG3_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg3::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg3::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg3 to value 0x0003_6120"]
impl crate::Resettable for WIFI_PLL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_6120;
}
